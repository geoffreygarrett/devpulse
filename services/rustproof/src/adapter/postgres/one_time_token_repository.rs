use crate::adapter::{GenericOneTimeTokenRepository, OneTimeToken, OneTimeTokenRepository};
use crate::errors::{DatabaseSnafu, Result};
use async_trait::async_trait;
use snafu::ResultExt;
use sqlx::PgPool;


#[async_trait]
impl OneTimeTokenRepository for GenericOneTimeTokenRepository<PgPool> {
    async fn store_one_time_token(&self, token: OneTimeToken) -> Result<()> {
        // language=SQL
        sqlx::query(
            r#"
                INSERT INTO one_time_tokens (id, user_id, token_type, token, metadata, created_at, expires_at, used, revoked)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
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
                WHERE token = $1 AND token_type = $2 AND revoked = FALSE AND used = FALSE AND expires_at > NOW()
            "#
        )
            .bind(token.to_string())
            .bind(token_type.to_string())
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
                WHERE token = $1
            "#
        )
            .bind(token.to_string())
            .execute(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(())
    }
}
