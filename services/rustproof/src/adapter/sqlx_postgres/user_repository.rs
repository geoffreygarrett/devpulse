use crate::adapter::GenericRepository;
use crate::errors::{DatabaseSnafu, Result};
use crate::repositories::UserRepository;
use crate::repositories::{CreateUserParams, UserRecord};
use async_trait::async_trait;
use chrono::Utc;
use snafu::ResultExt;
use sqlx::{PgPool, Postgres, Transaction};
use tokio::try_join;
use tracing::{info, instrument};
use uuid::Uuid;


/// Implementation of the `UserRepository` trait specifically for `PgPool`.
#[async_trait]
impl UserRepository for GenericRepository<PgPool> {
    /// Creates a new user with the given email and password hash, returning the
    /// `UserRecord` of the newly created user.
    ///
    /// # Errors
    /// Returns a `DatabaseSnafu` error if the SQL execution fails.
    async fn create_user(&self, params: &CreateUserParams) -> Result<UserRecord> {
        // language=SQL
        let query = r#"
            INSERT INTO users (
                instance_id,
                id,
                aud,
                role,
                email,
                encrypted_password,
                email_confirmed_at,
                last_sign_in_at,
                raw_app_meta_data,
                raw_user_meta_data,
                is_super_admin,
                created_at,
                updated_at,
                phone,
                phone_confirmed_at,
                confirmation_token,
                email_change,
                email_change_token_new,
                recovery_token
            )
            VALUES (
                '00000000-0000-0000-0000-000000000000'::uuid,
                $1,
                'authenticated',
                'authenticated',
                $2,
                $3,
                NULL,
                NULL,
                '{"provider": "email", "providers": ["email"]}',
                '{}',
                FALSE,
                $4,
                $5,
                $6,
                NULL,
                '',
                '',
                '',
                ''
            )
            RETURNING
                instance_id,
                id,
                aud,
                role,
                email,
                encrypted_password,
                email_confirmed_at,
                last_sign_in_at,
                raw_app_meta_data,
                raw_user_meta_data,
                is_super_admin,
                created_at,
                updated_at,
                phone,
                phone_confirmed_at,
                confirmation_token,
                email_change,
                email_change_token_new,
                recovery_token,
                confirmed_at,
                invited_at,
                confirmation_sent_at,
                recovery_sent_at,
                email_change_token_current,
                email_change_sent_at,
                raw_app_meta_data,
                raw_user_meta_data
        "#;

        let user = sqlx::query_as::<_, UserRecord>(query)
            .bind(params.id.unwrap_or_else(Uuid::new_v4))  // Use provided id or generate a new one
            .bind(&params.email)
            .bind(&params.encrypted_password)
            .bind(params.created_at.unwrap_or_else(Utc::now)) // Use provided created_at or current time
            .bind(params.updated_at.unwrap_or_else(Utc::now)) // Use provided updated_at or current time
            .bind(params.confirmation_sent_at.unwrap_or_else(Utc::now)) // Use provided confirmation_sent_at or current time
            .fetch_one(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(user)
    }


    /// Retrieves a user by their email address, returning an optional `UserRecord`.
    ///
    /// # Errors
    /// Returns a `DatabaseSnafu` error if the SQL execution fails.
    async fn get_user_by_email(&self, email: &str) -> Result<Option<UserRecord>> {
        // language=SQL
        let user = sqlx::query_as::<_, UserRecord>(
            r#"
        SELECT
            instance_id,
            id,
            aud,
            role,
            email,
            encrypted_password,
            email_confirmed_at,
            last_sign_in_at,
            raw_app_meta_data,
            raw_user_meta_data,
            is_super_admin,
            created_at,
            updated_at,
            phone,
            phone_confirmed_at,
            confirmation_token,
            email_change,
            email_change_token_new,
            recovery_token,
            confirmed_at,
            invited_at,
            confirmation_sent_at,
            recovery_sent_at,
            email_change_token_current,
            email_change_sent_at,
            raw_app_meta_data,
            raw_user_meta_data
        FROM users
        WHERE email = $1
        "#
        )
            .bind(email)
            .fetch_optional(&self.pool)  // Changed from `fetch_one` to `fetch_optional`
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
        SELECT
            instance_id,
            id,
            aud,
            role,
            email,
            encrypted_password,
            email_confirmed_at,
            last_sign_in_at,
            raw_app_meta_data,
            raw_user_meta_data,
            is_super_admin,
            created_at,
            updated_at,
            phone,
            phone_confirmed_at,
            confirmation_token,
            email_change,
            email_change_token_new,
            recovery_token,
            confirmed_at,
            invited_at,
            confirmation_sent_at,
            recovery_sent_at,
            email_change_token_current,
            email_change_sent_at,
            raw_app_meta_data,
            raw_user_meta_data
        FROM users
        WHERE id = $1
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
            SET encrypted_password = $1, updated_at = now()
            WHERE id = $2
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
                SELECT 1 FROM users WHERE email = $1
            )
            "#
        )
            .bind(email)
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
                SELECT 1 FROM users WHERE id = $1
            )
            "#
        )
            .bind(user_id)
            .fetch_one(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(exists)
    }

    /// Deletes a user by their ID.
    async fn delete_user(&self, user_id: &Uuid) -> Result<()> {
        // language=SQL
        let query = r#"
            DELETE FROM users WHERE id = $1
        "#;

        sqlx::query(query)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .context(DatabaseSnafu)?;

        Ok(())
    }

    /// Lists users with pagination.
    #[tracing::instrument(skip(self))]
    async fn list_users(&self, offset: i32, limit: i32) -> Result<(Vec<UserRecord>, u64)> {
        // TODO: Sanitize user record, remove tokens and password hash.
        info!("Fetching users with pagination");

        // language=SQL
        let users_query = r#"
            SELECT * FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2
        "#;
        let users_future = sqlx::query_as::<_, UserRecord>(users_query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool);

        // language=SQL
        let count_query = r#"
            SELECT COUNT(*) FROM users
        "#;
        let count_future = sqlx::query_scalar::<_, i64>(count_query)
            .fetch_one(&self.pool);

        // Execute both futures concurrently
        let (users, total): (Vec<UserRecord>, i64) = try_join!(users_future, count_future)
            .context(DatabaseSnafu)?;

        info!("Successfully fetched users and total count");

        Ok((users, total as u64))
    }
}
