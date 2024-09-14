use crate::adapter::RefreshTokenRepository;
use crate::errors::{InternalSnafu, JwtDecodeSnafu, JwtEncodeSnafu, Result};
use crate::models::api::access_token_response::AccessTokenResponse;
use crate::models::claims::RustProofClaims;
use crate::repositories::UserRecord;
use crate::services::token_service::AccessTokenService;
use async_trait::async_trait;
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use uuid::Uuid;

pub struct JwtTokenService {
    // refresh_token_repository: Arc<dyn RefreshTokenRepository>,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
    jwt_exp: usize,
    jwt_secret: String,
}


impl JwtTokenService {
    pub fn new(
        // refresh_token_repository: Arc<dyn RefreshTokenRepository>,
        encoding_key: EncodingKey, decoding_key: DecodingKey, validation: Validation, jwt_exp: usize, jwt_secret: String) -> Self {
        Self {
            // refresh_token_repository,
            encoding_key,
            decoding_key,
            validation,
            jwt_exp,
            jwt_secret,
        }
    }

    fn generate_claims(
        &self,
        user: &UserRecord,
        session_id: &Uuid,
        parent_token_id: Option<&Uuid>,
    ) -> RustProofClaims {
        RustProofClaims {
            issuer: Some("https://example.com".to_string()), // Typically the URL of the service issuing the token, e.g. "https://example.com
            issued_at: Some(Utc::now().timestamp() as u64),
            subject: Some(user.id.to_string()),
            roles: Some(vec!["authenticated".to_string()]), // Typically the roles of the user, e.g. ["user", "admin"]
            audience: Some("authenticated".to_string()), // Typically the audience of the token, e.g. "authenticated
            expiration: Some(((Utc::now().timestamp() as usize) + self.jwt_exp) as u64),
            session_id: Some(session_id.to_string()),
            email: Some(user.email.clone()),
            app_metadata: user.raw_app_meta_data.0.clone(),
            user_metadata: user.raw_user_meta_data.0.clone(),
            ..Default::default()
        }
    }

    async fn generate_token_with_claims(&self, claims: RustProofClaims) -> Result<String> {
        let header = Header::default();
        let token = encode(&header, &claims, &self.encoding_key)
            .context(JwtEncodeSnafu)
            .context(InternalSnafu { code: 500_u16 })?;
        Ok(token)
    }
}

impl Debug for JwtTokenService {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[async_trait]
impl AccessTokenService for JwtTokenService {
    async fn issue_tokens(
        &self,
        user: &UserRecord,
        session_id: &Uuid,
        parent_token_id: Option<&Uuid>,
    ) -> Result<AccessTokenResponse> {
        let claims = self.generate_claims(user, session_id, parent_token_id);
        let access_token = self.generate_token_with_claims(claims).await?;

        let refresh_token = Uuid::new_v4().to_string(); // Generate a refresh token or pull from DB

        Ok(AccessTokenResponse {
            token: access_token,
            token_type: "bearer".to_string(),
            expires_in: Some(self.jwt_exp as i32 as u64),
            refresh_token: Some(refresh_token),
            scope: None,
        })
    }

    async fn validate_access_token(&self, token: &str) -> Result<RustProofClaims> {
        let token_data = decode::<RustProofClaims>(token, &self.decoding_key, &self.validation)
            .context(JwtDecodeSnafu)
            .context(InternalSnafu { code: 500_u16 })?;
        Ok(token_data.claims)
    }

    async fn validate_refresh_token(&self, refresh_token: &str) -> Result<UserRecord> {


        let user = UserRecord {
            instance_id: Default::default(),
            id: Uuid::new_v4(),
            aud: "".to_string(),
            role: "".to_string(),
            email: "example@example.com".to_string(),
            encrypted_password: "".to_string(),
            confirmed_at: None,
            invited_at: None,
            confirmation_token: "".to_string(),
            confirmation_sent_at: None,
            recovery_token: "".to_string(),
            recovery_sent_at: None,
            email_change_token_current: "".to_string(),
            email_change: None,
            email_change_sent_at: None,
            last_sign_in_at: None,
            raw_app_meta_data: HashMap::new().into(),
            raw_user_meta_data: HashMap::new().into(),
            is_super_admin: false,
            created_at: Default::default(),
            updated_at: Default::default(),
        };
        Ok(user)
    }

    async fn invalidate_token(&self, token: &str) -> Result<()> {
        // Implementation would involve marking the token as invalid in a database
        // or removing it from an in-memory store
        Ok(())
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation};
//     use std::collections::HashMap;
//     use uuid::Uuid;
//
//     fn create_test_service() -> JwtTokenService {
//         let encoding_key = EncodingKey::from_secret("test_secret".as_ref());
//         let decoding_key = DecodingKey::from_secret("test_secret".as_ref());
//         let validation = Validation::new(Algorithm::HS256);
//         JwtTokenService::new(encoding_key, decoding_key, validation, 3600, "test_secret".to_string())
//     }
//
//     fn create_test_user() -> UserRecord {
//         UserRecord {
//             instance_id: Default::default(),
//             id: Uuid::new_v4(),
//             aud: "test_audience".to_string(),
//             role: "test_role".to_string(),
//             email: "test@example.com".to_string(),
//             encrypted_password: "hashed_password".to_string(),
//             confirmed_at: None,
//             invited_at: None,
//             confirmation_token: "confirmation_token".to_string(),
//             confirmation_sent_at: None,
//             recovery_token: "recovery_token".to_string(),
//             recovery_sent_at: None,
//             email_change_token_current: "email_change_token".to_string(),
//             email_change: None,
//             email_change_sent_at: None,
//             last_sign_in_at: None,
//             raw_app_meta_data: HashMap::new().into(),
//             raw_user_meta_data: HashMap::new().into(),
//             is_super_admin: false,
//             created_at: Default::default(),
//             updated_at: Default::default(),
//         }
//     }
//
//     #[tokio::test]
//     async fn test_issue_tokens() {
//         let service = create_test_service();
//         let user = create_test_user();
//         let session_id = Uuid::new_v4();
//
//         let result = service.issue_tokens(&user, &session_id, None).await;
//
//         assert!(result.is_ok());
//         let token_response = result.unwrap();
//         assert_eq!(token_response.token_type, "bearer");
//         assert_eq!(token_response.expires_in, 3600);
//         assert!(!token_response.token.is_empty());
//         assert!(!token_response.refresh_token);
//         println!("{}", serde_json::to_string_pretty(&token_response).unwrap());
//     }
//
//     #[tokio::test]
//     async fn test_validate_access_token() {
//         let service = create_test_service();
//         let user = create_test_user();
//         let session_id = Uuid::new_v4();
//
//         let token_response = service.issue_tokens(&user, &session_id, None).await.unwrap();
//         let valid = service.validate_access_token(&token_response.token).await;
//
//         assert!(valid.is_ok());
//         assert!(valid.unwrap());
//     }
//
//     #[tokio::test]
//     async fn test_validate_access_token_invalid() {
//         let service = create_test_service();
//         let invalid_token = "invalid.token.here";
//
//         let valid = service.validate_access_token(invalid_token).await;
//
//         assert!(valid.is_err());
//     }
//
//     #[tokio::test]
//     async fn test_validate_refresh_token() {
//         let service = create_test_service();
//         let user = create_test_user();
//         let session_id = Uuid::new_v4();
//
//         let token_response = service.issue_tokens(&user, &session_id, None).await.unwrap();
//         let validated_user = service.validate_refresh_token(&token_response.refresh_token).await;
//
//         assert!(validated_user.is_ok());
//         let validated_user = validated_user.unwrap();
//         assert_eq!(validated_user.email, "example@example.com");
//     }
//
//     #[tokio::test]
//     async fn test_invalidate_token() {
//         let service = create_test_service();
//         let token = "some_token_to_invalidate";
//
//         let result = service.invalidate_token(token).await;
//
//         assert!(result.is_ok());
//     }
// }
