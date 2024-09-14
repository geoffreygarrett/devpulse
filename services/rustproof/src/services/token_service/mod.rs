pub mod jwts_service;
pub mod paseto_service;
pub mod auth_token_service;
pub mod token_generator;

use std::fmt::Debug;
use crate::errors::Result;
use crate::models::api::access_token_response::AccessTokenResponse;
use async_trait::async_trait;
use uuid::Uuid;
use crate::models::claims::RustProofClaims;
use crate::repositories::UserRecord;

#[async_trait]
pub trait AccessTokenService : Send + Sync + Debug {
    async fn issue_tokens(
        &self,
        user: &UserRecord,
        session_id: &Uuid,
        parent_token_id: Option<&Uuid>,
    ) -> Result<AccessTokenResponse>;

    /// Validate the provided access token and return the extracted claims.
    async fn validate_access_token(&self, token: &str) -> Result<RustProofClaims>;

    /// Validate the provided refresh token and return the associated user record.
    async fn validate_refresh_token(&self, refresh_token: &str) -> Result<UserRecord>;

    /// Invalidate the provided token (access or refresh).
    async fn invalidate_token(&self, token: &str) -> Result<()>;
}
