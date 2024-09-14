use crate::adapter::{GenericOneTimeTokenRepository, OneTimeToken, OneTimeTokenRepository};
use crate::errors::{DatabaseSnafu, Result};
use async_trait::async_trait;
use snafu::ResultExt;
use sqlx::SqlitePool;

#[async_trait]
impl OneTimeTokenRepository for GenericOneTimeTokenRepository<SqlitePool> {
    async fn store_one_time_token(&self, token: OneTimeToken) -> Result<()> {
        // language=SQL
        sqlx::query(
            r#"
                INSERT INTO one_time_tokens (id, user_id, token_type, token, metadata, created_at, expires_at, used, revoked)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
            .bind(token.id)
            .bind(token.user_id)
            .bind(token.token_type)
            .bind(token.token)
            .bind(token.metadata)
            .bind(token.created_at)
            .bind(token.expires_at)
            .bind(token.used)
            .bind(token.revoked)
            .execute(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(())
    }

    async fn validate_one_time_token(&self, token: &str, token_type: &str) -> Result<OneTimeToken> {
        // language=SQL
        let token = sqlx::query_as::<_, OneTimeToken>(
            r#"
                SELECT id, user_id, token_type, token, metadata, created_at, expires_at, used, revoked
                FROM one_time_tokens
                WHERE token = ? AND token_type = ? AND revoked = FALSE AND used = FALSE AND expires_at > CURRENT_TIMESTAMP
            "#
        )
            .bind(token)
            .bind(token_type)
            .fetch_one(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(token)
    }

    async fn revoke_one_time_token(&self, token: &str) -> Result<()> {
        // language=SQL
        sqlx::query(
            r#"
                UPDATE one_time_tokens
                SET revoked = TRUE
                WHERE token = ?
            "#
        )
            .bind(token)
            .execute(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(())
    }
}
