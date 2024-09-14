use crate::errors::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use ipnetwork::IpNetwork;
use serde;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(any(feature = "adapter-postgres"))]
pub mod sqlx_postgres;

#[cfg(any(feature = "adapter-postgres"))]
pub use sqlx_postgres::*;

use crate::models::records::RefreshTokenRecord;

pub mod prelude {
    pub use super::DbPool;
    pub use super::GenericRepository;
    pub use super::OneTimeTokenRepository;
    pub use super::PasswordHistoryRepository;
    pub use super::RefreshTokenRepository;
    pub use super::SessionRepository;
    pub use super::StoreRefreshTokenParams;
    pub use async_trait::async_trait;
}

// #[derive(Debug)]
// #[cfg_attr(feature = "adapter-postgres", derive(sqlx::FromRow))]
// pub struct UserRecord {
//     pub id: uuid::Uuid,
//     pub email: String,
//     pub password_hash: String,
// }

#[derive(Debug)]
#[cfg_attr(feature = "adapter-postgres", derive(sqlx::FromRow))]
pub struct OneTimeToken {
    pub id: uuid::Uuid,
    pub user_id: Option<uuid::Uuid>,
    pub token_type: String,
    pub token: String,
    pub metadata: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub used: bool,
    pub revoked: bool,
}

#[derive(Debug)]
#[cfg_attr(feature = "adapter-postgres", derive(sqlx::FromRow))]
pub struct SessionRecord {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub factor_id: Option<uuid::Uuid>,
    pub aal: Option<AalLevel>, // For simplicity, you can use a string or create a custom enum.
    pub not_after: Option<chrono::DateTime<chrono::Utc>>,
    pub refreshed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub user_agent: Option<String>,
    pub ip: Option<IpNetwork>, // Using String for IP, you can customize to your specific needs.
    pub tag: Option<String>,
}

#[derive(Debug)]
#[cfg_attr(feature = "adapter-postgres", derive(sqlx::FromRow))]
pub struct HistoricalPasswordRecord {
    pub user_id: uuid::Uuid,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct StoreRefreshTokenParams<'a> {
    pub user_id: &'a Uuid,
    pub token: &'a str,
    pub parent_token_id: Option<&'a Uuid>,
    pub session_id: Option<&'a Uuid>,
    pub instance_id: &'a Uuid,
}


#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[cfg_attr(feature = "adapter-postgres", derive(sqlx::Type))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "aal_level", rename_all = "snake_case")]
pub enum AalLevel {
    Aal1,
    Aal2,
    Aal3,
}

#[derive(Debug)]
pub struct CreateSessionParams {
    pub user_id: Uuid,
    pub factor_id: Option<Uuid>,
    pub aal: Option<AalLevel>,
    pub not_after: Option<DateTime<Utc>>,
    pub refreshed_at: Option<DateTime<Utc>>,
    pub user_agent: Option<String>,
    pub ip: Option<IpNetwork>,
    pub tag: Option<String>,
}


#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    async fn store_refresh_token<'a>(&'a self, params: StoreRefreshTokenParams<'a>) -> Result<()>;

    async fn validate_refresh_token(&self, refresh_token: &str) -> Result<RefreshTokenRecord>;

    async fn revoke_refresh_token(&self, refresh_token: &str) -> Result<()>;
    async fn revoke_all_tokens_for_session(&self, session_id: &Uuid) -> Result<()>;

    async fn revoke_all_tokens_for_user(&self, user_id: &Uuid) -> Result<()>;
}

#[async_trait]
pub trait PasswordHistoryRepository: Send + Sync {
    async fn store_password_hash(&self, user_id: &uuid::Uuid, password_hash: String) -> Result<()>;
    async fn get_password_history(&self, user_id: &uuid::Uuid, limit: Option<i32>) -> Result<Vec<HistoricalPasswordRecord>>;
}

#[async_trait]
pub trait OneTimeTokenRepository: Send + Sync {
    async fn store_one_time_token(&self, token: OneTimeToken) -> Result<()>;
    async fn validate_one_time_token(&self, token: &str, token_type: &str) -> Result<OneTimeToken>;
    async fn revoke_one_time_token(&self, token: &str) -> Result<()>;
}

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create_session(&self, params: CreateSessionParams) -> Result<SessionRecord>;
    async fn get_session_by_id(&self, session_id: &Uuid) -> Result<SessionRecord>;
    async fn revoke_session(&self, session_id: &Uuid) -> Result<()>;
    async fn revoke_all_sessions_for_user(&self, user_id: &Uuid) -> Result<()>;
    //
    // async fn get<D>(&self, id: &Uuid) -> Result<D>
    // where
    //     D: serde::de::DeserializeOwned;
}


#[async_trait]
pub trait DbPool: Send + Sync {}


pub struct GenericRepository<DBP: DbPool> {
    pool: DBP,
}

impl<DBP: DbPool> GenericRepository<DBP> {
    pub fn new(pool: DBP) -> Self {
        Self { pool }
    }
}

