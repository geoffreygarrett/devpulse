use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{decode, encode, Header, Validation};
use once_cell::sync::Lazy;
use prost::Message;
use snafu::*;

use crate::errors::{AuthError, BadJwtSnafu, InternalSnafu, JwtDecodeSnafu, JwtEncodeSnafu};
use crate::prelude::{AuthConfig, Claims, Keys};

/// Static key storage for JWT encoding and decoding keys.
static KEYS: Lazy<RwLock<HashMap<String, Keys>>> = Lazy::new(|| {
    let mut keys = HashMap::new();
    keys.insert("key1".to_string(), Keys::new_symmetric("secret1".as_bytes()));
    keys.insert("key2".to_string(), Keys::new_symmetric("secret2".as_bytes()));
    RwLock::new(keys)
});

impl AuthConfig {
    /// Creates a new `JwtConfig` instance for symmetric algorithms.
    ///
    /// # Arguments
    ///
    /// * `secret` - The secret key as a string.
    /// * `expiration` - The expiration time in seconds.
    /// * `algorithm` - The algorithm to use for signing the token.
    ///
    /// # Returns
    ///
    /// A new `JwtConfig` instance.
    pub fn new_symmetric(secret: String, expiration: usize, validation: Validation) -> Self {
        Self {
            secret,
            expiration,
            validation,
            private_key: None,
            public_key: None,
        }
    }

    /// Creates a new `JwtConfig` instance for asymmetric algorithms.
    ///
    /// # Arguments
    ///
    /// * `private_key` - The private key as a string.
    /// * `public_key` - The public key as a string.
    /// * `expiration` - The expiration time in seconds.
    /// * `algorithm` - The algorithm to use for signing the token.
    ///
    /// # Returns
    ///
    /// A new `JwtConfig` instance.
    pub fn new_asymmetric(
        private_key: String, public_key: String, expiration: usize, validation: Validation,
    ) -> Self {
        Self {
            secret: None,
            expiration,
            validation,
            private_key: Some(private_key),
            public_key: Some(public_key),
        }
    }
}

/// Service for managing JWT operations such as token generation, validation, and key rotation.
pub struct JwtsService {
    config: Arc<AuthConfig>,
}

impl JwtsService {
    /// Creates a new `JwtsService` instance.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration for the JWT service.
    ///
    /// # Returns
    ///
    /// A new `JwtsService` instance.
    pub fn new(config: Arc<JwtConfig>) -> Self {
        Self { config }
    }

    /// Generates a new JWT token.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID to include in the token.
    /// * `kid` - The key ID to use for signing the token.
    /// * `roles` - The roles to include in the token.
    ///
    /// # Returns
    ///
    /// A Result containing the token or an AuthError.
    pub fn generate_token(
        &self, user_id: &str, kid: &str, roles: Vec<String>,
    ) -> Result<String, AuthError> {
        let expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + self.config.expiration as u64;

        let claims = Claims {
            sub: user_id.to_string(),
            exp: expiration,
            roles,
            org: None,
            iat: Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            ),
            iss: None,
            aud: None,
            jti: None,
        };

        let header = Header {
            kid: Some(kid.to_string()),
            alg: self.config.validation.algorithms[0],
            ..Default::default()
        };

        let keys = KEYS.read().unwrap();
        let key = keys.get(kid).context(BadJwtSnafu)?;
        encode(&header, &claims, &key.encoding)
            .context(JwtEncodeSnafu)
            .context(InternalSnafu { code: 500u16 })
    }

    pub fn generate_refresh_token(&self, user_id: &str, kid: &str) -> Result<String, AuthError> {
        // Assuming a significantly longer expiration period for refresh tokens
        let refresh_expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + (self.config.refresh_expiration as u64 * 24 * 60 * 60); // Refresh token expiration in days

        let claims = Claims {
            sub: user_id.to_string(),
            exp: refresh_expiration,
            // Minimized claims for refresh tokens
            roles: Vec::new(),
            org: None,
            iat: Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            ),
            iss: None,
            aud: None,
            jti: None, // Consider using a unique identifier for tracking/revoke purposes
        };

        let header = Header {
            kid: Some(kid.to_string()),
            alg: self.config.validation.algorithms[0],
            ..Default::default()
        };

        let keys = KEYS.read().unwrap();
        let key = keys.get(kid).context(BadJwtSnafu)?;
        encode(&header, &claims, &key.encoding)
            .context(JwtEncodeSnafu)
            .context(InternalSnafu { code: 500u16 })
    }

    /// Validates a given JWT token.
    ///
    /// # Arguments
    ///
    /// * `token` - The JWT token to validate.
    ///
    /// # Returns
    ///
    /// A Result containing the claims or an AuthError.
    pub fn validate_token(&self, token: &str) -> Result<Claims, AuthError> {
        let header = jsonwebtoken::decode_header(token)
            .context(JwtDecodeSnafu)
            .context(InternalSnafu { code: 500u16 })?;
        let kid = header.kid.context(BadJwtSnafu)?;
        let keys = KEYS.read().unwrap();
        let key = keys.get(&kid).context(BadJwtSnafu)?;
        decode::<Claims>(token, &key.decoding, &self.config.validation)
            .map_err(|_| AuthError::BadJwt)
            .map(|data| data.claims)
    }

    /// Rotates the JWT keys.
    ///
    /// # Arguments
    ///
    /// * `new_kid` - The new key ID.
    /// * `new_secret` - The new secret key.
    /// * `is_symmetric` - Whether the key is symmetric or asymmetric.
    pub fn rotate_keys(&self, new_kid: &str, new_secret: &[u8], is_symmetric: bool) {
        let mut keys = KEYS.write().unwrap();
        if is_symmetric {
            keys.insert(new_kid.to_string(), Keys::new_symmetric(new_secret));
        } else {
            let private_key = new_secret;
            let public_key = new_secret; // This would be replaced with the actual public key for asymmetric keys
            keys.insert(new_kid.to_string(), Keys::new_asymmetric(private_key, public_key));
        }
    }
}
