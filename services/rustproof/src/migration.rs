// use crate::config::{DatabaseConfig, DatabaseDriver};
// use async_trait::async_trait;
// use once_cell::sync::Lazy;
// use sqlx::migrate::{Migration, MigrationSource};
// use sqlx::{Database, Pool, Row, Transaction};
// use std::borrow::Cow;
// use std::collections::HashMap;
// use thiserror::Error;
//
// pub const DEFAULT_SCHEMA: &str = "_auth";
// pub const MIGRATIONS_TABLE: &str = "_migrations";
// pub const DEFAULT_OWNER: &str = "postgres";
// pub const USERS_TABLE: &str = "users";
// pub const REFRESH_TOKENS_TABLE: &str = "refresh_tokens";
//
// #[async_trait]
// impl MigrationManager for PostgresMigrationManager {
//     type DB = sqlx::Postgres;
//
//     async fn schema_exists(&self, pool: &Pool<Self::DB>, schema_name: &str) -> Result<bool> {
//         let row = sqlx::query(
//             "SELECT EXISTS (SELECT 1 FROM information_schema.schemata WHERE schema_name = $1)",
//         )
//             .bind(schema_name)
//             .fetch_one(pool)
//             .await?;
//
//         Ok(row.get::<bool, _>(0))
//     }
//
//     async fn table_exists(&self, pool: &Pool<Self::DB>, schema_name: &str, table_name: &str) -> Result<bool> {
//         let row = sqlx::query(
//             "SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_schema = $1 AND table_name = $2)",
//         )
//             .bind(schema_name)
//             .bind(table_name)
//             .fetch_one(pool)
//             .await?;
//
//         Ok(row.get::<bool, _>(0))
//     }
//
//     async fn create_schema(&self, pool: &Pool<Self::DB>, schema_name: &str) -> Result<()> {
//         sqlx::query(&format!("CREATE SCHEMA IF NOT EXISTS {}", schema_name))
//             .execute(pool)
//             .await?;
//         Ok(())
//     }
//
//     async fn create_migrations_table(&self, pool: &Pool<Self::DB>, schema_name: &str) -> Result<()> {
//         let query = format!(
//             r#"
//             CREATE TABLE IF NOT EXISTS {}.{}
//             (
//                 version        BIGINT PRIMARY KEY,
//                 description    TEXT NOT NULL,
//                 installed_on   TIMESTAMP WITH TIME ZONE DEFAULT NOW() NOT NULL,
//                 success        BOOLEAN NOT NULL,
//                 checksum       BYTEA NOT NULL,
//                 execution_time BIGINT NOT NULL
//             );
//             "#,
//             schema_name, MIGRATIONS_TABLE
//         );
//         sqlx::query(&query).execute(pool).await?;
//         Ok(())
//     }
//
//     async fn fetch_applied_migrations(&self, pool: &Pool<Self::DB>, schema: &str) -> Result<HashMap<i64, sqlx::migrate::AppliedMigration>> {
//         let query = format!(
//             "SELECT version, checksum FROM {}.{}",
//             schema, MIGRATIONS_TABLE
//         );
//         let rows = sqlx::query(&query).fetch_all(pool).await?;
//
//         let mut applied_migrations = HashMap::new();
//         for row in rows {
//             let version: i64 = row.get("version");
//             let checksum: Vec<u8> = row.get("checksum");
//             applied_migrations.insert(version, sqlx::migrate::AppliedMigration {
//                 version,
//                 checksum: Cow::Owned(checksum),
//             });
//         }
//
//         Ok(applied_migrations)
//     }
//
//     async fn apply_migration(&self, transaction: &mut Transaction<'_, Self::DB>, migration: &Migration, placeholders: &HashMap<&str, &str>) -> Result<()> {
//         let sql = replace_placeholders(&migration.sql, placeholders);
//
//         transaction.execute(&*sql).await?;
//
//         sqlx::query(
//             "INSERT INTO _migrations (version, description, installed_on, success, checksum, execution_time) VALUES ($1, $2, NOW(), TRUE, $3, 0)",
//         )
//             .bind(migration.version)
//             .bind(migration.description.as_ref())
//             .bind(&*migration.checksum)
//             .execute(&mut **transaction)
//             .await?;
//
//         Ok(())
//     }
//
//     async fn set_search_path(&self, pool: &Pool<Self::DB>, schema: &str) -> Result<()> {
//         sqlx::query(&format!("SET search_path TO {}", schema))
//             .execute(pool)
//             .await?;
//         Ok(())
//     }
// }
// pub async fn initialize_database(config: &DatabaseConfig) -> Result<()> {
//     match config.driver {
//         DatabaseDriver::Postgres => {
//             let pool = Pool::<sqlx::Postgres>::connect(&config.url).await?;
//             let manager = PostgresMigrationManager { config: config.clone() };
//             run_migrations(&manager, &pool, config).await?;
//         }
//         _ => return Err(MigrationError::UnsupportedDatabase(format!("{:?}", config.driver))),
//     }
//     Ok(())
// }
//
//
// fn replace_placeholders(sql: &str, placeholders: &HashMap<&str, &str>) -> String {
//     let mut replaced = sql.to_string();
//     for (key, value) in placeholders {
//         replaced = replaced.replace(&format!(":{}", key), value);
//     }
//     replaced
// }
//
// pub async fn run_migrations<M: MigrationManager>(
//     manager: &M,
//     pool: &Pool<M::DB>,
//     config: &DatabaseConfig,
// ) -> Result<()> {
//     tracing::info!("Running migrations");
//
//     manager.set_search_path(pool, &config.schema).await?;
//
//     let placeholders = HashMap::from([
//         ("schema", config.schema.as_str()),
//         ("owner", config.owner.as_str()),
//     ]);
//
//     if !manager.schema_exists(pool, &config.schema).await? {
//         manager.create_schema(pool, &config.schema).await?;
//     }
//
//     manager.create_migrations_table(pool, &config.schema).await?;
//
//     let applied_migrations = manager.fetch_applied_migrations(pool, &config.schema).await?;
//
//     for migration in MIGRATIONS.iter() {
//         tracing::info!("Checking migration version {}", migration.version);
//
//         if let Some(applied_migration) = applied_migrations.get(&migration.version) {
//             if migration.checksum != applied_migration.checksum {
//                 return Err(MigrationError::ChecksumMismatch(format!(
//                     "Checksum mismatch for migration version {}: expected {:?}, found {:?}",
//                     migration.version,
//                     migration.checksum,
//                     applied_migration.checksum
//                 )));
//             }
//             tracing::info!("Migration version {} already applied, skipping", migration.version);
//         } else {
//             tracing::info!("Applying migration version {}", migration.version);
//             let mut transaction = pool.begin().await?;
//             manager.apply_migration(&mut transaction, migration, &placeholders).await?;
//             transaction.commit().await?;
//             tracing::info!("Migration version {} applied successfully", migration.version);
//         }
//     }
//
//     tracing::info!("All migrations are up to date.");
//     Ok(())
// }
