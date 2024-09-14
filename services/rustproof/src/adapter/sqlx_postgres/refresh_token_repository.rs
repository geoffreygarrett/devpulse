use crate::adapter::{GenericRepository, RefreshTokenRecord, RefreshTokenRepository, StoreRefreshTokenParams};
use crate::errors::{DatabaseSnafu, Result};
use async_trait::async_trait;
use snafu::ResultExt;
use sqlx::PgPool;
use sqlx::types::Uuid;

/// Implementation of the `RefreshTokenRepository` trait specifically for `PgPool`.
#[async_trait]
impl RefreshTokenRepository for GenericRepository<PgPool> {
    /// Stores a refresh token in the database using the provided parameters.
    ///
    /// # Errors
    /// Returns a `DatabaseSnafu` error if the SQL execution fails.
    async fn store_refresh_token<'a>(&'a self, params: StoreRefreshTokenParams<'a>) -> Result<()> {
        // language=SQL
        sqlx::query(
            r#"
            INSERT INTO refresh_tokens (user_id, token, parent_token_id, session_id, instance_id)
            VALUES ($1, $2, $3, $4, $5)
            "#
        )
            .bind(&params.user_id)
            .bind(&params.token.to_string())
            .bind(&params.parent_token_id)
            .bind(&params.session_id)
            .bind(&params.instance_id)
            .execute(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(())
    }

    /// Validates a refresh token and returns the associated `RefreshTokenRecord` if valid.
    ///
    /// # Errors
    /// Returns a `DatabaseSnafu` error if the SQL execution fails.
    async fn validate_refresh_token(&self, refresh_token: &str) -> Result<RefreshTokenRecord> {
        // language=SQL
        let refresh_token_record = sqlx::query_as::<_, RefreshTokenRecord>(
            r#"
            SELECT id, user_id, token, parent_token_id, revoked, created_at, updated_at, session_id, instance_id
            FROM refresh_tokens
            WHERE token = $1 AND revoked = FALSE
            "#
        )
            .bind(refresh_token)
            .fetch_one(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(refresh_token_record)
    }

    /// Revokes a refresh token by marking it as revoked in the database.
    ///
    /// # Errors
    /// Returns a `DatabaseSnafu` error if the SQL execution fails.
    async fn revoke_refresh_token(&self, refresh_token: &str) -> Result<()> {
        // language=SQL
        sqlx::query(
            r#"
            UPDATE refresh_tokens
            SET revoked = TRUE
            WHERE token = $1
            "#
        )
            .bind(refresh_token)
            .execute(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(())
    }

    /// Revokes all tokens associated with a specific session by marking them as revoked.
    ///
    /// # Errors
    /// Returns a `DatabaseSnafu` error if the SQL execution fails.
    async fn revoke_all_tokens_for_session(&self, session_id: &Uuid) -> Result<()> {
        // language=SQL
        sqlx::query(
            r#"
            UPDATE refresh_tokens
            SET revoked = TRUE
            WHERE session_id = $1
            "#
        )
            .bind(session_id)
            .execute(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(())
    }

    /// Revokes all tokens associated with a specific user by marking them as revoked.
    ///
    /// # Errors
    /// Returns a `DatabaseSnafu` error if the SQL execution fails.
    async fn revoke_all_tokens_for_user(&self, user_id: &Uuid) -> Result<()> {
        // language=SQL
        sqlx::query(
            r#"
            UPDATE refresh_tokens
            SET revoked = TRUE
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
