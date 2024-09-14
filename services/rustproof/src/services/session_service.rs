// use crate::errors::{AuthError, Result};
// use crate::services::token_service::{TokenService, TokenStrategy, TokenServiceFactory, Session};
// use async_trait::async_trait;
// use serde_json::json;
// use uuid::Uuid;
// use chrono::{Utc, Duration};
// use crate::models::rustproof::Session;
//
// pub struct SessionService {
//     token_service: Box<dyn TokenService>,
//     expiration_seconds: i64,
// }
//
// impl SessionService {
//     pub fn new(strategy: TokenStrategy, expiration_seconds: i64) -> Self {
//         let token_service = TokenServiceFactory::create(strategy);
//         Self { token_service, expiration_seconds }
//     }
//     // {
//     // "aud": "authenticated",
//     // "exp": 1615824388,
//     // "sub": "0334744a-f2a2-4aba-8c8a-6e748f62a172",
//     // "email": "d.l.solove@gmail.com",
//     // "app_metadata": {
//     // "provider": "email"
//     // },
//     // "user_metadata": null,
//     // "role": "authenticated"
//     // }
//     pub async fn create_session(&self, user_id: &Uuid) -> Result<Session> {
//         let access_claims = json!({
//             "iss": "gg_auth",
//             "sub": user_id.to_string(),
//             "exp": (Utc::now() + Duration::seconds(self.expiration_seconds)).timestamp(),
//         });
//
//         let refresh_claims = json!({
//             "iss": "gg_auth",
//             "sub": user_id.to_string(),
//             "exp": (Utc::now() + Duration::days(30)).timestamp(), // Example refresh expiration
//         });
//
//         let access_token = self.token_service.generate_token_with_claims(access_claims).await?;
//         let refresh_token = self.token_service.generate_token_with_claims(refresh_claims).await?;
//
//         Ok(Session {
//             access_token,
//             refresh_token,
//             token_type: "Bearer".into(),
//             expires_in: self.expiration_seconds,
//             user_id: *user_id,
//             expires_at: Utc::now() + Duration::seconds(self.expiration_seconds),
//         })
//     }
//
//     pub async fn validate_session(&self, access_token: &str) -> Result<bool> {
//         self.token_service.validate_token(access_token).await
//     }
//
//     pub async fn refresh_session(&self, refresh_token: &str) -> Result<Session> {
//         let is_valid = self.token_service.validate_token(refresh_token).await?;
//         if is_valid {
//             let user_id = self.extract_user_id(refresh_token)?;
//             self.create_session(&user_id).await
//         } else {
//             Err(AuthError::BadJwt.into())
//         }
//     }
//
//     fn extract_user_id(&self, token: &str) -> Result<Uuid> {
//         // Logic to extract user_id from token
//         Ok(Uuid::new_v4()) // Placeholder implementation
//     }
//
//     pub async fn invalidate_session(&self, token: &str) -> Result<()> {
//         self.token_service.invalidate_token(token).await
//     }
// }
