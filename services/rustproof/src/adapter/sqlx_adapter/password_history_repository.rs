use crate::adapter::{DbPool, GenericRepository, HistoricalPasswordRecord, PasswordHistoryRepository};
use crate::errors::{DatabaseSnafu, Result};
use async_trait::async_trait;
use snafu::ResultExt;
use sqlx::types::Uuid;
use sqlx::{Database, Executor, FromRow, IntoArguments, Pool, Row, Type};
use chrono::{DateTime, Utc};

// /// A struct representing a historical password record.
// pub struct HistoricalPasswordRecord {
//     pub password_hash: String,
//     pub created_at: DateTime<Utc>,
// }

/// Manually implementing the `FromRow` trait for `HistoricalPasswordRecord`.
/// Implementing `FromRow` generically for any SQLx database.

/// Implementing `FromRow` for PostgreSQL
impl<'r> FromRow<'r, sqlx::postgres::PgRow> for HistoricalPasswordRecord {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(HistoricalPasswordRecord {
            user_id: row.try_get("user_id")?,
            password_hash: row.try_get("password_hash")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

/// Implementing `FromRow` for SQLite
impl<'r> FromRow<'r, sqlx::sqlite::SqliteRow> for HistoricalPasswordRecord {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(HistoricalPasswordRecord {
            user_id: row.try_get("user_id")?,
            password_hash: row.try_get("password_hash")?,
            created_at: row.try_get("created_at")?,
        })
    }
}


/// A generic implementation of the `PasswordHistoryRepository` trait for any SQL database
/// supported by `sqlx`. This allows the repository to be used with PostgreSQL, MySQL,
/// SQLite, and others.
///
/// The `GenericRepository` is initialized with a connection pool, which is then
/// used to execute database queries.
///
/// # Type Parameters
/// - `DB`: The specific database type, such as `Postgres`, `MySql`, or `Sqlite`.
#[async_trait]
impl<DB> PasswordHistoryRepository for GenericRepository<Pool<DB>>
where
    DB: Database + Send + Sync,
    for<'q> DB::Arguments<'q>: IntoArguments<'q, DB>,
    for<'c> &'c Pool<DB>: Executor<'c, Database = DB>,
    for<'c> Uuid: sqlx::Encode<'c, DB> + sqlx::Decode<'c, DB> + Type<DB>,
    for<'c> String: sqlx::Encode<'c, DB> + sqlx::Decode<'c, DB> + Type<DB>,
    for<'c> DateTime<Utc>: sqlx::Encode<'c, DB> + sqlx::Decode<'c, DB> + Type<DB>,
    for<'r> HistoricalPasswordRecord: FromRow<'r, <DB as sqlx::Database>::Row>,
    for<'c> i32: sqlx::Encode<'c, DB> + sqlx::Type<DB>, // Corrected to use specific lifetime
{
    /// Stores a password hash in the password history for a given user.
    ///
    /// # Parameters
    /// - `user_id`: The unique identifier of the user.
    /// - `password_hash`: The hashed password to store.
    ///
    /// # Errors
    /// Returns a `DatabaseSnafu` error if the SQL execution fails.
    async fn store_password_hash(&self, user_id: &Uuid, password_hash: String) -> Result<()> {
        // language=SQL
        sqlx::query(
            r#"
            INSERT INTO password_history (user_id, password_hash)
            VALUES (?, ?)
            "#
        )
            .bind(user_id)
            .bind(password_hash)
            .execute(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(())
    }

    /// Retrieves the password history for a given user, limited to a specified number of entries.
    ///
    /// # Parameters
    /// - `user_id`: The unique identifier of the user.
    /// - `limit`: The maximum number of password history records to retrieve. Defaults to all records if not specified.
    ///
    /// # Errors
    /// Returns a `DatabaseSnafu` error if the SQL execution fails.
    async fn get_password_history(&self, user_id: &Uuid, limit: Option<i32>) -> Result<Vec<HistoricalPasswordRecord>> {
        // language=SQL
        let hashes = sqlx::query_as::<_, HistoricalPasswordRecord>(
            r#"
            SELECT password_hash, created_at FROM password_history
            WHERE user_id = ?
            ORDER BY created_at DESC
            LIMIT ?
            "#
        )
            .bind(user_id)
            .bind(limit.unwrap_or(i32::MAX))
            .fetch_all(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(hashes)
    }
}
