use jsonwebtoken::{DecodingKey, EncodingKey};

/// Represents the encoding and decoding keys for JWT.
pub struct Keys {
    pub(crate) encoding: EncodingKey,
    pub(crate) decoding: DecodingKey,
}

impl Keys {
    /// Creates a new `Keys` instance with a symmetric secret.
    ///
    /// # Arguments
    ///
    /// * `secret` - The secret key as bytes.
    ///
    /// # Returns
    ///
    /// A new `Keys` instance.
    pub fn new_symmetric(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }

    /// Creates a new `Keys` instance with asymmetric keys.
    ///
    /// # Arguments
    ///
    /// * `private_key` - The private key as PEM bytes.
    /// * `public_key` - The public key as PEM bytes.
    ///
    /// # Returns
    ///
    /// A new `Keys` instance.
    pub fn new_asymmetric(private_key: &[u8], public_key: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_rsa_pem(private_key).expect("Invalid private key"),
            decoding: DecodingKey::from_rsa_pem(public_key).expect("Invalid public key"),
        }
    }
}
