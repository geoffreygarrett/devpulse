use crate::adapter::{CreateSessionParams, DbPool, GenericRepository, SessionRecord, SessionRepository};
use crate::errors::{DatabaseSnafu, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use snafu::ResultExt;
use sqlx::types::Uuid;
use sqlx::{Database, Executor, IntoArguments, Any, Pool, Type, FromRow, Row};


// Implement `FromRow` for `SessionRecord` for `Postgres`
impl<'r> FromRow<'r, sqlx::postgres::PgRow> for SessionRecord {
    fn from_row(row: &'r sqlx::postgres::PgRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(SessionRecord {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            factor_id: row.try_get("factor_id")?,
            aal: row.try_get("aal")?,
            not_after: row.try_get("not_after")?,
            refreshed_at: row.try_get("refreshed_at")?,
            user_agent: row.try_get("user_agent")?,
            ip: row.try_get("ip")?,
            tag: row.try_get("tag")?,
        })
    }
}

// Implement `FromRow` for `SessionRecord` for `Sqlite`
impl<'r> FromRow<'r, sqlx::sqlite::SqliteRow> for SessionRecord {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> std::result::Result<Self, sqlx::Error> {
        Ok(SessionRecord {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            factor_id: row.try_get("factor_id")?,
            aal: row.try_get("aal")?,
            not_after: row.try_get("not_after")?,
            refreshed_at: row.try_get("refreshed_at")?,
            user_agent: row.try_get("user_agent")?,
            ip: row.try_get("ip")?,
            tag: row.try_get("tag")?,
        })
    }
}

/// A generic implementation of the `SessionRepository` trait for any SQL database
/// supported by `sqlx`. This allows the repository to be used with PostgreSQL,
/// MySQL, SQLite, and others.
///
/// The `GenericRepository` is initialized with a connection pool, which is then
/// used to execute database queries.
///
/// # Type Parameters
/// - `DB`: The specific database type, such as `Postgres`, `MySql`, or `Sqlite`.
#[async_trait]
impl<DB: Database> SessionRepository for GenericRepository<Pool<DB>>
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
    for<'c> std::option::Option<DateTime<Utc>>: sqlx::Encode<'c, DB>
{
    /// Creates a new session using the provided parameters and returns the `SessionRecord`.
    ///
    /// # Errors
    /// Returns a `DatabaseSnafu` error if the SQL execution fails.
    async fn create_session(&self, params: CreateSessionParams) -> Result<SessionRecord> {
        let session_id = Uuid::new_v4();
        let created_at = Utc::now();
        let updated_at = created_at;

        let session = SessionRecord {
            id: session_id,
            user_id: params.user_id,
            created_at,
            updated_at,
            factor_id: params.factor_id,
            aal: params.aal,
            not_after: params.not_after,
            refreshed_at: params.refreshed_at,
            user_agent: params.user_agent,
            ip: params.ip,
            tag: params.tag,
        };

        // language=SQL
        let response = sqlx::query_as::<_, SessionRecord>(
            r#"
                INSERT INTO sessions (id, user_id, created_at, updated_at, factor_id, aal, not_after, refreshed_at, user_agent, ip, tag)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
            .bind(session.id)
            .bind(session.user_id)
            .bind(session.created_at)
            .bind(session.updated_at)
            .bind(session.factor_id)
            .bind(session.aal)
            .bind(session.not_after)
            .bind(session.refreshed_at)
            .bind(session.user_agent)
            .bind(session.ip)
            .bind(session.tag)
            .fetch_one(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(response)
    }

    /// Retrieves a session by its ID, returning the associated `SessionRecord`.
    ///
    /// # Errors
    /// Returns a `DatabaseSnafu` error if the SQL execution fails.
    async fn get_session_by_id(&self, session_id: &Uuid) -> Result<SessionRecord> {
        // language=SQL
        let session = sqlx::query_as::<_, SessionRecord>(
            r#"
                SELECT id, user_id, created_at, updated_at, factor_id, aal, not_after, refreshed_at, user_agent, ip, tag
                FROM sessions
                WHERE id = ?
            "#
        )
            .bind(session_id)
            .fetch_one(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(session)
    }

    /// Revokes (deletes) a session by its ID.
    ///
    /// # Errors
    /// Returns a `DatabaseSnafu` error if the SQL execution fails.
    async fn revoke_session(&self, session_id: &Uuid) -> Result<()> {
        // language=SQL
        sqlx::query(
            r#"
                DELETE FROM sessions
                WHERE id = ?
            "#
        )
            .bind(session_id)
            .execute(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(())
    }

    /// Revokes (deletes) all sessions for a specific user by their ID.
    ///
    /// # Errors
    /// Returns a `DatabaseSnafu` error if the SQL execution fails.
    async fn revoke_all_sessions_for_user(&self, user_id: &Uuid) -> Result<()> {
        // language=SQL
        sqlx::query(
            r#"
                DELETE FROM sessions
                WHERE user_id = ?
            "#
        )
            .bind(user_id)
            .execute(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(())
    }
}
