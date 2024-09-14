use crate::errors::{InternalSnafu, JwtDecodeSnafu, JwtEncodeSnafu, Result};
use crate::services::token_service::token_generator::TokenGenerator;
use async_trait::async_trait;
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use snafu::ResultExt;
use std::collections::HashMap;
use uuid::Uuid;

pub struct JwtTokenGenerator {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    validation: Validation,
    jwt_exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
    sub: String,
    email: String,
    exp: usize,
    app_metadata: HashMap<String, Value>,
    user_metadata: HashMap<String, Value>,
}

impl JwtTokenGenerator {
    pub fn new(encoding_key: EncodingKey, decoding_key: DecodingKey, validation: Validation, jwt_exp: usize) -> Self {
        Self {
            encoding_key,
            decoding_key,
            validation,
            jwt_exp,
        }
    }

    fn generate_claims(&self, user_id: &Uuid, email: &str, app_metadata: HashMap<String, Value>, user_metadata: HashMap<String, Value>) -> JwtClaims {
        JwtClaims {
            sub: user_id.to_string(),
            email: email.to_string(),
            exp: (Utc::now().timestamp() as usize) + self.jwt_exp,
            app_metadata,
            user_metadata,
        }
    }
}

#[async_trait]
impl TokenGenerator for JwtTokenGenerator {
    async fn generate_token(&self, claims: &Value) -> Result<String> {
        let header = Header::default();
        let token = encode(&header, claims, &self.encoding_key)
            .context(JwtEncodeSnafu)
            .context(InternalSnafu { code: 500_u16 })?;
        Ok(token)
    }

    async fn validate_token(&self, token: &str) -> Result<bool> {
        let token_data = decode::<Value>(token, &self.decoding_key, &self.validation)
            .context(JwtDecodeSnafu)
            .context(InternalSnafu { code: 500_u16 })?;
        let exp = token_data.claims["exp"].as_i64().unwrap_or(0);
        Ok(exp > Utc::now().timestamp())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
    use serde_json::json;
    use uuid::Uuid;

    // Mock keys for testing
    fn generate_test_keys() -> (EncodingKey, DecodingKey) {
        let encoding_key = EncodingKey::from_secret("test_secret".as_ref());
        let decoding_key = DecodingKey::from_secret("test_secret".as_ref());
        (encoding_key, decoding_key)
    }

    // Helper function to create a JwtTokenGenerator
    fn create_test_generator() -> JwtTokenGenerator {
        let (encoding_key, decoding_key) = generate_test_keys();
        let validation = Validation::default();
        JwtTokenGenerator::new(encoding_key, decoding_key, validation, 3600) // 1 hour expiry
    }

    #[tokio::test]
    async fn test_generate_token() {
        let generator = create_test_generator();
        let user_id = Uuid::new_v4();
        let email = "test@example.com".to_string();
        let app_metadata = HashMap::new();
        let user_metadata = HashMap::new();
        let claims = generator.generate_claims(&user_id, &email, app_metadata, user_metadata);
        let claims_value = json!(claims);

        let token = generator.generate_token(&claims_value).await.unwrap();
        println!("Token: {}", token);
        assert!(!token.is_empty(), "Token should not be empty");
    }

    #[tokio::test]
    async fn test_validate_token() {
        let generator = create_test_generator();
        let user_id = Uuid::new_v4();
        let email = "test@example.com".to_string();
        let app_metadata = HashMap::new();
        let user_metadata = HashMap::new();
        let claims = generator.generate_claims(&user_id, &email, app_metadata, user_metadata);
        let claims_value = json!(claims);

        let token = generator.generate_token(&claims_value).await.unwrap();
        let is_valid = generator.validate_token(&token).await.unwrap();
        assert!(is_valid, "Token should be valid");
    }

    // #[tokio::test]
    // async fn test_validate_expired_token() {
    //     let encoding_key = EncodingKey::from_secret("test_secret".as_ref());
    //     let decoding_key = DecodingKey::from_secret("test_secret".as_ref());
    //     let mut validation = Validation::default();
    //     validation.leeway = 0; // Disable leeway to test expired token
    //
    //     let generator = JwtTokenGenerator::new(encoding_key, decoding_key, validation, 3600); // Expire immediately
    //     let user_id = Uuid::new_v4();
    //     let email = "test@example.com".to_string();
    //     let app_metadata = HashMap::new();
    //     let user_metadata = HashMap::new();
    //     let claims = generator.generate_claims(&user_id, &email, app_metadata, user_metadata);
    //     let claims_value = json!(claims);
    //
    //     let token = generator.generate_token(&claims_value).await.unwrap();
    //     let is_valid = generator.validate_token(&token).await.unwrap();
    //     assert!(!is_valid, "Token should be invalid due to expiration");
    // }
}

// A Result containing the claims or an AuthError.
//     pub fn validate_token(&self, token: &str) -> Result<Claims, AuthError> {
//         let header = jsonwebtoken::decode_header(token)
//             .context(JwtDecodeSnafu)
//             .context(InternalSnafu { code: 500u16 })?;
//         let kid = header.kid.context(BadJwtSnafu)?;
//         let keys = KEYS.read().unwrap();
//         let key = keys.get(&kid).context(BadJwtSnafu)?;
//         decode::<Claims>(token, &key.decoding, &self.config.validation)
//             .map_err(|_| AuthError::BadJwt)
//             .map(|data| data.claims)
//     }
