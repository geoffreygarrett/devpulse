use crate::adapter::{GenericPasswordHistoryRepository, HistoricalPasswordRecord, PasswordHistoryRepository};
use crate::errors::{DatabaseSnafu, Result};
use async_trait::async_trait;
use snafu::ResultExt;
use sqlx::types::uuid::Uuid;
use sqlx::PgPool;

#[async_trait]
impl PasswordHistoryRepository for GenericPasswordHistoryRepository<PgPool> {
    async fn store_password_hash(&self, user_id: &Uuid, password_hash: String) -> Result<()> {
        // language=SQL
        sqlx::query(
            r#"
            INSERT INTO password_history (user_id, password_hash)
            VALUES ($1, $2)
            "#
        )
            .bind(user_id)
            .bind(password_hash)
            .execute(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(())
    }

    async fn get_password_history(&self, user_id: &Uuid, limit: Option<i32>) -> Result<Vec<HistoricalPasswordRecord>> {
        // language=SQL
        let hashes = sqlx::query_as::<_, HistoricalPasswordRecord>(
            r#"
            SELECT password_hash FROM password_history
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2
            "#
        )
            .bind(user_id)
            .bind(limit)
            .fetch_all(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(hashes)
    }
}
