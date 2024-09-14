use crate::adapter::{GenericRepository};
use crate::errors::{DatabaseSnafu, Result};
use async_trait::async_trait;
use chrono::Utc;
use snafu::ResultExt;
use sqlx::PgPool;
use uuid::Uuid;
use crate::repositories::{CreateIdentityParams, IdentityRecord, IdentityRepository};

#[async_trait]
impl IdentityRepository for GenericRepository<PgPool> {
    async fn create_identity(&self, params: &CreateIdentityParams) -> Result<IdentityRecord> {
        // language=SQL
        let query = r#"
            INSERT INTO identities (
                id, user_id, identity_type, value, data, verified, provider, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, user_id, identity_type, value, data, verified, provider, created_at, updated_at
        "#;

        let identity_record = sqlx::query_as::<_, IdentityRecord>(query)
            .bind(Uuid::new_v4())
            .bind(&params.user_id)
            .bind(&params.identity_type)
            .bind(&params.value)
            .bind(&params.data)
            .bind(params.verified)
            .bind(&params.provider)
            .bind(Utc::now())
            .bind(Utc::now())
            .fetch_one(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(identity_record)
    }

    async fn get_identity_by_id(&self, id: &Uuid) -> Result<Option<IdentityRecord>> {
        // language=SQL
        let query = r#"
            SELECT id, user_id, identity_type, value, data, verified, provider, created_at, updated_at
            FROM identities
            WHERE id = $1
        "#;

        let identity = sqlx::query_as::<_, IdentityRecord>(query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(identity)
    }

    async fn get_identity_by_type_and_value(&self, identity_type: &str, value: &str) -> Result<Option<IdentityRecord>> {
        // language=SQL
        let query = r#"
            SELECT id, user_id, identity_type, value, data, verified, provider, created_at, updated_at
            FROM identities
            WHERE identity_type = $1 AND value = $2
        "#;

        let identity = sqlx::query_as::<_, IdentityRecord>(query)
            .bind(identity_type)
            .bind(value)
            .fetch_optional(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(identity)
    }

    async fn identity_exists(&self, identity_type: &str, value: &str) -> Result<bool> {
        // language=SQL
        let query = r#"
            SELECT EXISTS(
                SELECT 1 FROM identities WHERE identity_type = $1 AND value = $2
            )
        "#;

        let exists = sqlx::query_scalar(query)
            .bind(identity_type)
            .bind(value)
            .fetch_one(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(exists)
    }

    async fn link_identity(&self, user_id: &Uuid, params: &CreateIdentityParams) -> Result<()> {
        // language=SQL
        let query = r#"
            INSERT INTO identities (
                id, user_id, identity_type, value, data, verified, provider, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (identity_type, value) DO NOTHING
        "#;

        sqlx::query(query)
            .bind(Uuid::new_v4())
            .bind(user_id)
            .bind(&params.identity_type)
            .bind(&params.value)
            .bind(&params.data)
            .bind(params.verified)
            .bind(&params.provider)
            .bind(Utc::now())
            .bind(Utc::now())
            .execute(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(())
    }

    async fn unlink_identity(&self, identity_id: &Uuid) -> Result<()> {
        // language=SQL
        let query = r#"
            DELETE FROM identities WHERE id = $1
        "#;

        sqlx::query(query)
            .bind(identity_id)
            .execute(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(())
    }

    async fn get_user_identities(&self, user_id: &Uuid) -> Result<Vec<IdentityRecord>> {
        // language=SQL
        let query = r#"
            SELECT id, user_id, identity_type, value, data, verified, provider, created_at, updated_at
            FROM identities WHERE user_id = $1
        "#;

        let identities = sqlx::query_as::<_, IdentityRecord>(query)
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(identities)
    }

    async fn remove_unverified_identities(&self, user_id: &Uuid) -> Result<()> {
        // language=SQL
        let query = r#"
            DELETE FROM identities WHERE user_id = $1 AND verified = FALSE
        "#;

        sqlx::query(query)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(())
    }

    async fn update_identity_verification(&self, id: &Uuid, verified: bool) -> Result<()> {
        // language=SQL
        let query = r#"
            UPDATE identities
            SET verified = $1, updated_at = now()
            WHERE id = $2
        "#;

        sqlx::query(query)
            .bind(verified)
            .bind(id)
            .execute(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(())
    }
}
