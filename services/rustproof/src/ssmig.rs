use crate::adapter::GenericRepository;
use crate::services::PasswordService;
use derive_more::Into;
use once_cell::sync::Lazy;
use serde::Deserialize;
use sqlx::migrate::{AppliedMigration as SqlxAppliedMigration, Migration, MigrationType};
use sqlx::postgres::PgRow;
use sqlx::{AnyPool, PgPool, Postgres, Row};
use std::borrow::Cow;
use std::collections::HashMap;
use tracing::log::info;

const DEFAULT_SCHEMA: &str = "_auth";
const MIGRATIONS_TABLE: &str = "_migrations";
const DEFAULT_OWNER: &str = "postgres";
const USERS_TABLE: &str = "users";
const REFRESH_TOKENS_TABLE: &str = "refresh_tokens";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
enum Driver {
    Postgres,
    Sqlite,
}

#[derive(Deserialize, Debug)]
struct Config {
    #[serde(default = "default_driver")]
    driver: Driver,
    #[serde(default = "default_url")]
    url: String,
    #[serde(default = "default_schema")]
    schema: String,
    #[serde(default = "default_owner")]
    owner: String,
}

fn default_url() -> String {
    "postgres://postgres.xijufqypeyrpkypqdzbi:mYxunFHqdzIdmV00@aws-0-eu-west-2.pooler.supabase.com:5432/postgres?options=-c%20search_path=_auth"
        .to_string()
}
fn default_schema() -> String { DEFAULT_SCHEMA.to_string() }
fn default_owner() -> String { DEFAULT_OWNER.to_string() }

fn default_driver() -> Driver { Driver::Postgres }

static MIGRATIONS: Lazy<Vec<Migration>> = Lazy::new(|| {
    vec![
        Migration::new(
            1,
            Cow::Borrowed("Initial setup with schemas, migrations table, users table, and refresh tokens table"),
            MigrationType::Simple,
            Cow::Borrowed(include_str!("../migrations/postgres/001_initial_setup.sql")),
            false,
        ),
    ]
});


/// Replaces placeholders in the SQL template with actual configuration values.
fn replace_placeholders(sql: &str, config: &HashMap<&str, &str>) -> String {
    let mut replaced = sql.to_string();
    for (key, value) in config {
        replaced = replaced.replace(&format!(":{}", key), value);
    }
    replaced
}


async fn schema_exists(pool: &PgPool, schema_name: &str) -> Result<bool, sqlx::Error> {
    // language=SQL
    let result: (bool,) = sqlx::query_as(
        "SELECT EXISTS (SELECT 1 FROM information_schema.schemata WHERE schema_name = $1)"
    )
        .bind(schema_name)
        .fetch_one(pool)
        .await?;

    Ok(result.0)
}
//
async fn table_exists(pool: &PgPool, schema_name: &str, table_name: &str) -> Result<bool, sqlx::Error> {
    // language=SQL
    let result: (bool,) = sqlx::query_as(
        "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_schema = $1 AND table_name = $2)"
    )
        .bind(schema_name)
        .bind(table_name)
        .fetch_one(pool)
        .await?;

    Ok(result.0)
}
//
// async fn table_exists(pool: &PgPool, table_name: &str) -> Result<bool, sqlx::Error> {
//     // language=SQL
//     let result: (bool,) = sqlx::query_as(
//         "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_schema = $1 AND table_name = $2)"
//     )
//         .bind(table_name)
//         .fetch_one(pool)
//         .await?;
//
//     Ok(result.0)
// }
async fn create_migration_table_if_not_exists(pool: &PgPool, migrations_schema: &str) -> Result<(), sqlx::Error> {
    if schema_exists(pool, migrations_schema).await? {
        info!("Schema {} already exists", migrations_schema);
    } else {
        // Create schema
        sqlx::query(&format!("CREATE SCHEMA IF NOT EXISTS {}", migrations_schema))
            .execute(pool)
            .await?;
        info!("Created schema {}", migrations_schema);
    }

    // Check if the migration table exists
    if table_exists(pool, migrations_schema, "_migrations").await? {
        info!("Table _migrations already exists in schema {}", migrations_schema);
    } else {
        // Create the migration table
        sqlx::raw_sql(&format!(
            r#"
            SET search_path TO {};
            CREATE TABLE IF NOT EXISTS _migrations
            (
                version        BIGINT PRIMARY KEY,
                description    TEXT NOT NULL,
                installed_on   TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
                success        BOOLEAN NOT NULL,
                checksum       BYTEA NOT NULL,
                execution_time BIGINT NOT NULL
            );
            "#,
            migrations_schema
        ))
            .execute(pool)
            .await?;
        info!("Created _migrations table in schema {}", migrations_schema);
    }
    Ok(())
}

async fn fetch_applied_migrations(pool: &PgPool, schema: &str) -> Result<HashMap<i64, SqlxAppliedMigration>, sqlx::Error> {
    let sql = format!(
        "SELECT version, checksum FROM {schema}._migrations",
        schema = schema
    );

    let rows = sqlx::query(&sql)
        .fetch_all(pool)
        .await?;

    let mut applied_migrations = HashMap::new();

    for row in rows {
        let version: i64 = row.try_get("version")?;
        let checksum: Vec<u8> = row.try_get("checksum")?;
        applied_migrations.insert(version, SqlxAppliedMigration {
            version,
            checksum: Cow::Owned(checksum),
        });
    }

    Ok(applied_migrations)
}

async fn apply_migration(pool: &PgPool, migration: &Migration, placeholders: &HashMap<&str, &str>) -> Result<(), sqlx::Error> {
    let sql = replace_placeholders(&migration.sql, placeholders);

    // Execute the migration
    sqlx::raw_sql(&sql).execute(pool).await?;

    // language=SQL
    sqlx::query("INSERT INTO _migrations (version, description, installed_on, success, checksum, execution_time) VALUES ($1, $2, NOW(), TRUE, $3, 0)")
        .bind(migration.version)
        .bind(migration.description.as_ref())
        .bind(&*migration.checksum)
        .execute(pool)
        .await?;

    Ok(())
}

async fn set_search_path(pool: &PgPool, schema: &str) -> Result<(), sqlx::Error> {
    sqlx::query(&format!("SET search_path TO {}", schema))
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn run_migrations() -> Result<(), sqlx::Error> {
    info!("Running migrations");

    // let config = envy::prefixed("GG_AUTH_").from_env::<Config>()
    //     .expect("Configuration could not be loaded from environment variables");

    let pool = PgPool::connect(&config.url).await?;

    set_search_path(&pool, &config.schema).await?;

    let placeholders = HashMap::from([
        ("schema", config.schema.as_str()),
        ("owner", config.owner.as_str()),
    ]);

    create_migration_table_if_not_exists(&pool, &config.schema).await?;

    let applied_migrations = fetch_applied_migrations(&pool, &config.schema).await?;

    for migration in MIGRATIONS.iter() {
        info!("Checking migration version {}", migration.version);

        if let Some(applied_migration) = applied_migrations.get(&migration.version) {
            if migration.checksum != applied_migration.checksum {
                return Err(sqlx::Error::Protocol(format!(
                    "Checksum mismatch for migration version {}: expected {:?}, found {:?}",
                    migration.version,
                    migration.checksum,
                    applied_migration.checksum
                )));
            }
            info!("Migration version {} already applied, skipping", migration.version);
        } else {
            info!("Applying migration version {}", migration.version);
            apply_migration(&pool, migration, &placeholders).await?;
            info!("Migration version {} applied successfully", migration.version);
        }
    }

    let db = GenericRepository::<PgPool>::new(pool);

    // let test = AnyPool::connect(&config.url).await?;
    let password_service = PasswordService::new();
    let hashed= password_service.hash_password("password").expect("TODO: panic message");

    // test verifications
    password_service.verify_password("password", &hashed).expect("TODO: panic message");


    // db.create_user("test77sss@gmail.com".to_string(), hashed.to_string()).await.map(|user| {
    //     info!("User created: {:?}", user);
    // }).map_err(|e| {
    //     info!("Failed to create user: {:?}", e);
    // }).expect("TODO: panic message");

    info!("All migrations are up to date.");
    Ok(())
}