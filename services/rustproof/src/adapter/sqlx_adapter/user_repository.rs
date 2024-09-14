use std::collections::HashMap;
use crate::adapter::{DbPool, GenericRepository, SessionRecord, UserRecord, UserRepository};
use crate::errors::{DatabaseSnafu, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use snafu::ResultExt;
use sqlx::{ColumnIndex, Database, Executor, FromRow, IntoArguments, Pool, Row, Type};
use sqlx::types::JsonValue;
use uuid::Uuid;

/// A generic implementation of the `UserRepository` trait for any SQL database
/// supported by `sqlx`. This allows the repository to be used with PostgreSQL,
/// MySQL, SQLite, and others.
///
/// The `GenericRepository` is initialized with a connection pool, which is then
/// used to execute database queries.
///
/// # Type Parameters
/// - `DB`: The specific database type, such as `Postgres`, `MySql`, or `Sqlite`.
#[async_trait]
impl<DB> UserRepository for GenericRepository<Pool<DB>>
where
    DB: Database + Send + Sync,
    for<'q> DB::Arguments<'q>: IntoArguments<'q, DB>, // Ensure that the arguments are compatible
    for<'c> &'c Pool<DB>: Executor<'c, Database = DB>,
    for<'c> Uuid: sqlx::Encode<'c, DB> + sqlx::Decode<'c, DB> + Type<DB>,
    for<'c> String: sqlx::Encode<'c, DB> + sqlx::Decode<'c, DB> + Type<DB>,
    for<'c> DateTime<Utc>: sqlx::Encode<'c, DB> + sqlx::Decode<'c, DB> + Type<DB>,
    for<'r> SessionRecord: FromRow<'r, <DB as sqlx::Database>::Row>,
    for<'c> std::option::Option<Uuid>: sqlx::Encode<'c, DB>,
    for<'c> std::option::Option<std::string::String>: sqlx::Encode<'c, DB>,
    for<'c> std::option::Option<DateTime<Utc>>: sqlx::Encode<'c, DB>,
    for<'c> sqlx::types::Json<HashMap<std::string::String, JsonValue>>: sqlx::Decode<'c, DB>,
    for<'c>  sqlx::types::Json<HashMap<std::string::String, JsonValue>>: Type<DB>,
    for<'c> bool: sqlx::Decode<'c, DB>, bool: Type<DB>,
    for<'r> UserRecord: FromRow<'r, <DB as sqlx::Database>::Row>,
    usize: ColumnIndex<<DB as sqlx::Database>::Row>
{
    /// Creates a new user with the given email and password hash, returning the
    /// `UserRecord` of the newly created user.
    ///
    /// # Errors
    /// Returns a `DatabaseSnafu` error if the SQL execution fails.
    async fn create_user(&self, email: String, password_hash: String) -> Result<UserRecord> {
        // language=SQL
        let user = sqlx::query_as::<_, UserRecord>(
            r#"
            INSERT INTO users (email, password_hash)
            VALUES (?, ?)
            RETURNING id, email, password_hash
            "#
        )
            .bind(email)
            .bind(password_hash)
            .fetch_one(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(user)
    }

    /// Retrieves a user by their email address, returning an optional `UserRecord`.
    ///
    /// # Errors
    /// Returns a `DatabaseSnafu` error if the SQL execution fails.
    async fn get_user_by_email(&self, email: &str) -> Result<UserRecord> {
        // language=SQL
        let user = sqlx::query_as::<_, UserRecord>(
            r#"
            SELECT id, email, password_hash, metadata
            FROM users
            WHERE email = ?
            "#
        )
            .bind(email.to_string())
            .fetch_one(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(user)
    }

    /// Retrieves a user by their ID, returning the associated `UserRecord`.
    ///
    /// # Errors
    /// Returns a `DatabaseSnafu` error if the SQL execution fails.
    async fn get_user_by_id(&self, user_id: &Uuid) -> Result<UserRecord> {
        // language=SQL
        let user = sqlx::query_as::<_, UserRecord>(
            r#"
            SELECT id, email, password_hash, given_name
            FROM users
            WHERE id = ?
            "#
        )
            .bind(user_id)
            .fetch_one(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(user)
    }

    /// Updates the password hash for a user identified by their ID.
    ///
    /// # Errors
    /// Returns a `DatabaseSnafu` error if the SQL execution fails.
    async fn update_user(&self, user_id: &Uuid, password_hash: String) -> Result<()> {
        // language=SQL
        sqlx::query(
            r#"
            UPDATE users
            SET password_hash = ?
            WHERE id = ?
            "#
        )
            .bind(password_hash)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(())
    }

    /// Checks if a user exists by their email address, returning a boolean.
    ///
    /// # Errors
    /// Returns a `DatabaseSnafu` error if the SQL execution fails.
    async fn user_exists_by_email(&self, email: &str) -> Result<bool> {
        // language=SQL
        let exists = sqlx::query_scalar(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM users WHERE email = ?
            )
            "#
        )
            .bind(email.to_string())
            .fetch_one(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(exists)
    }

    /// Checks if a user exists by their ID, returning a boolean.
    ///
    /// # Errors
    /// Returns a `DatabaseSnafu` error if the SQL execution fails.
    async fn user_exists_by_id(&self, user_id: &Uuid) -> Result<bool> {
        // language=SQL
        let exists = sqlx::query_scalar(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM users WHERE id = ?
            )
            "#
        )
            .bind(user_id)
            .fetch_one(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(exists)
    }
}
