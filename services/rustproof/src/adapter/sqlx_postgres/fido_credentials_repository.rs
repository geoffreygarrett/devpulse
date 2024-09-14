use crate::adapter::GenericRepository;
use crate::errors::{DatabaseSnafu, Result};
use crate::repositories::{CreateFidoCredentialsParams, FidoCredentialsRecord, FidoCredentialsRepository};
use async_trait::async_trait;
use snafu::ResultExt;
use sqlx::PgPool;
use uuid::Uuid;

#[async_trait]
impl FidoCredentialsRepository for GenericRepository<PgPool> {
    async fn create_credential(&self, params: CreateFidoCredentialsParams) -> Result<FidoCredentialsRecord> {
        // language=SQL
        let query = r#"
            INSERT INTO fido_credentials (
                id, user_id, credential_id, public_key, sign_count, aaguid, transports, created_at, display_name
            )
            VALUES (
                gen_random_uuid(), $1, $2, $3, $4, $5, $6, now(), $7
            )
            RETURNING
                id, user_id, credential_id, public_key, sign_count, aaguid, transports, created_at, last_used_at, display_name
        "#;

        let record = sqlx::query_as::<_, FidoCredentialsRecord>(query)
            .bind(params.user_id)
            .bind(&params.credential_id)
            .bind(&params.public_key)
            .bind(params.sign_count)
            .bind(params.aaguid)
            .bind(&params.transports)
            .bind(params.display_name)
            .fetch_one(&self.pool)
            .await
            .context(DatabaseSnafu)?;
        Ok(record)
    }

    async fn get_credential_by_id(&self, credential_id: &[u8]) -> Result<Option<FidoCredentialsRecord>> {
        // language=SQL
        let query = r#"
            SELECT id, user_id, credential_id, public_key, sign_count, aaguid, transports, created_at, last_used_at, display_name
            FROM fido_credentials
            WHERE credential_id = $1
        "#;

        let record = sqlx::query_as::<_, FidoCredentialsRecord>(query)
            .bind(credential_id)
            .fetch_optional(&self.pool)
            .await
            .context(DatabaseSnafu)?;
        Ok(record)
    }

    async fn get_credentials_by_user_id(&self, user_id: Uuid) -> Result<Vec<FidoCredentialsRecord>> {
        // language=SQL
        let query = r#"
            SELECT id, user_id, credential_id, public_key, sign_count, aaguid, transports, created_at, last_used_at, display_name
            FROM fido_credentials
            WHERE user_id = $1
        "#;

        let records = sqlx::query_as::<_, FidoCredentialsRecord>(query)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
            .context(DatabaseSnafu)?;
        Ok(records)
    }

    async fn update_sign_count(&self, credential_id: &[u8], sign_count: i64) -> Result<()> {
        // language=SQL
        let query = r#"
            UPDATE fido_credentials
            SET sign_count = $1, last_used_at = now()
            WHERE credential_id = $2
        "#;

        sqlx::query(query)
            .bind(sign_count)
            .bind(credential_id)
            .execute(&self.pool)
            .await
            .context(DatabaseSnafu)?;
        Ok(())
    }

    async fn delete_credential(&self, credential_id: &[u8]) -> Result<()> {
        // language=SQL
        let query = r#"
            DELETE FROM fido_credentials
            WHERE credential_id = $1
        "#;

        sqlx::query(query)
            .bind(credential_id)
            .execute(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(())
    }
}
