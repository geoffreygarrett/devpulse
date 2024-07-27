use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use nject::injectable;
use once_cell::sync::Lazy;
use prost::Message;

use crate::core::errors::ServiceError;

static KEYS: Lazy<RwLock<HashMap<String, Keys>>> = Lazy::new(|| {
    let mut keys = HashMap::new();
    keys.insert("key1".to_string(), Keys::new("secret1".as_bytes()));
    keys.insert("key2".to_string(), Keys::new("secret2".as_bytes()));
    RwLock::new(keys)
});

/// Represents the encoding and decoding keys for JWT.
pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    /// Creates a new `Keys` instance with the provided secret.
    ///
    /// # Arguments
    ///
    /// * `secret` - The secret key as bytes.
    ///
    /// # Returns
    ///
    /// A new `Keys` instance.
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub struct JwtConfig {
    pub secret: String,
    pub expiration: usize, // Expiration time in seconds
}

impl JwtConfig {
    /// Creates a new `JwtConfig` instance.
    ///
    /// # Arguments
    ///
    /// * `secret` - The secret key as a string.
    /// * `expiration` - The expiration time in seconds.
    ///
    /// # Returns
    ///
    /// A new `JwtConfig` instance.
    pub fn new(secret: String, expiration: usize) -> Self {
        Self { secret, expiration }
    }
}

#[injectable]
pub struct JwtService {
    config: Arc<JwtConfig>,
}

impl JwtService {
    pub fn new(config: Arc<JwtConfig>) -> Self {
        Self { config }
    }

    pub fn generate_token(
        &self, user_id: &str, kid: &str, roles: Vec<String>,
    ) -> Result<String, ServiceError> {
        let expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u64
            + self.config.expiration as u64;

        let claims = auth::Claims {
            sub: user_id.to_string(),
            exp: expiration,
            roles,
            org: None,
            iat: Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as u64,
            ),
            iss: None,
            aud: None,
            jti: None,
        };

        let mut claims_buf = Vec::new();
        claims.encode(&mut claims_buf).unwrap();

        let header = Header {
            kid: Some(kid.to_string()),
            ..Default::default()
        };

        let keys = KEYS.read().unwrap();
        let key = keys.get(kid).ok_or(ServiceError::AuthError)?;

        encode(&header, &claims_buf, &key.encoding).map_err(|_| ServiceError::AuthError)
    }

    pub fn validate_token(&self, token: &str) -> Result<auth::Claims, ServiceError> {
        let header = jsonwebtoken::decode_header(token).map_err(|_| ServiceError::AuthError)?;
        let kid = header.kid.ok_or(ServiceError::AuthError)?;

        let keys = KEYS.read().unwrap();
        let key = keys.get(&kid).ok_or(ServiceError::AuthError)?;

        let token_data = decode::<Vec<u8>>(token, &key.decoding, &Validation::default())
            .map_err(|_| ServiceError::AuthError)?;

        auth::Claims::decode(&*token_data.claims).map_err(|_| ServiceError::AuthError)
    }

    pub fn rotate_keys(&self, new_kid: &str, new_secret: &[u8]) {
        let mut keys = KEYS.write().unwrap();
        keys.insert(new_kid.to_string(), Keys::new(new_secret));
    }
}
