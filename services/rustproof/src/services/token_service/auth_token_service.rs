// use crate::adapter::{RefreshTokenRepository, StoreRefreshTokenParams, UserRepository};
// use crate::errors::{AuthError, Result};
// use crate::models::api::access_token::AccessTokenResponse;
// use crate::models::records::UserRecord;
// use crate::services::token_service::token_generator::TokenGenerator;
// use crate::services::token_service::TokenService;
// use async_trait::async_trait;
// use serde_json::json;
// use std::sync::Arc;
// use uuid::Uuid;
//
// // Redefine your AuthTokenService to use Arc internally
// #[nject::injectable]
// pub struct AuthTokenService {
//     user_repository: Arc<dyn UserRepository>,
//     token_generator: Arc<dyn TokenGenerator>,
//     refresh_token_repository: Arc<dyn RefreshTokenRepository>,
// }
//
// impl<'a> AuthTokenService<'a>
// where
// {
//     pub fn new(
//         user_repository: &'a dyn UserRepository,
//         token_generator: &'a dyn TokenGenerator,
//         refresh_token_repository: &'a dyn RefreshTokenRepository,
//     ) -> Self {
//         Self {
//             user_repository,
//             token_generator,
//             refresh_token_repository,
//         }
//     }
// }
// #[async_trait]
// impl TokenService for AuthTokenService
// {
//     async fn issue_tokens(
//         &self,
//         user: &UserRecord,
//         session_id: &Uuid,
//         parent_token_id: Option<&Uuid>,
//     ) -> Result<AccessTokenResponse> {
//         // Generate access token claims
//         let access_token_claims = json!({
//             "sub": user.id.to_string(),
//             "email": user.email,
//             "role": user.role,
//             "exp": (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp(),
//         });
//
//         // Generate access token
//         let access_token = self
//             .token_generator
//             .generate_token(&access_token_claims)
//             .await?;
//
//         // Generate and store the refresh token
//         let refresh_token = Uuid::new_v4().to_string();  // You can use a more secure method if needed
//
//         self.refresh_token_repository
//             .store_refresh_token(StoreRefreshTokenParams {
//                 user_id: &user.id,
//                 token: &refresh_token,
//                 parent_token_id,
//                 session_id: Some(session_id),
//                 instance_id: &Uuid::new_v4(), // Or use a specific instance ID if needed
//             })
//             .await?;
//
//         // Return the combined AccessTokenResponse
//         Ok(AccessTokenResponse {
//             token: access_token,
//             token_type: "Bearer".to_string(),
//             expires_in: 3600, // Adjust token expiration time as needed
//             refresh_token,
//         })
//     }
//
//     async fn validate_access_token(&self, token: &str) -> Result<bool> {
//         self.token_generator.validate_token(token).await
//     }
//
//     async fn validate_refresh_token(&self, refresh_token: &str) -> Result<UserRecord> {
//         let refresh_token_record = self
//             .refresh_token_repository
//             .validate_refresh_token(refresh_token)
//             .await?;
//
//         self.user_repository
//             .get_user_by_id(&refresh_token_record.user_id)
//             .await
//     }
//
//     async fn invalidate_token(&self, token: &str) -> Result<()> {
//         self.refresh_token_repository.revoke_refresh_token(token).await
//     }
// }
