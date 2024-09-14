use crate::adapter::{CreateSessionParams, GenericRepository, SessionRecord, SessionRepository};
use crate::errors::{DatabaseSnafu, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use snafu::ResultExt;
use sqlx::types::Uuid;
use sqlx::PgPool;

/// Implementation of the `SessionRepository` trait specifically for `PgPool`.
#[async_trait]
impl SessionRepository for GenericRepository<PgPool> {
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
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                RETURNING id, user_id, created_at, updated_at, factor_id, aal, not_after, refreshed_at, user_agent, ip, tag
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
                WHERE id = $1
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
                WHERE id = $1
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
                WHERE user_id = $1
            "#
        )
            .bind(user_id)
            .execute(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(())
    }
}
