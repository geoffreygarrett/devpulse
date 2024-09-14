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


// use std::fs::File;
// use std::io::{self, Read};
// use std::path::Path;
// use std::fmt;
// use anyhow::{Result, Context};
//
// /// A struct representing a symmetric key.
// pub struct SymmetricKey {
//     key: Vec<u8>,
// }
//
// impl SymmetricKey {
//     /// Creates a new `SymmetricKey` from a byte vector.
//     pub fn new(key: Vec<u8>) -> Self {
//         Self { key }
//     }
//
//     /// Loads a `SymmetricKey` from a file.
//     pub fn from_file(path: &Path) -> Result<Self> {
//         let mut file = File::open(path).with_context(|| format!("Failed to open file: {:?}", path))?;
//         let mut key = Vec::new();
//         file.read_to_end(&mut key).with_context(|| format!("Failed to read key from file: {:?}", path))?;
//         Self::validate_key_size(&key)?;
//         Ok(Self::new(key))
//     }
//
//     /// Validates the size of the symmetric key (e.g., 256-bit for AES).
//     fn validate_key_size(key: &[u8]) -> Result<()> {
//         if key.len() == 32 {
//             Ok(())
//         } else {
//             Err(anyhow::anyhow!("Invalid symmetric key size: expected 256-bit (32 bytes), got {} bytes", key.len()))
//         }
//     }
//
//     /// Returns the key as a byte slice.
//     pub fn as_slice(&self) -> &[u8] {
//         &self.key
//     }
// }
//
// impl fmt::Debug for SymmetricKey {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "SymmetricKey({} bytes)", self.key.len())
//     }
// }
//
// /// A struct representing an asymmetric key pair (public and private keys).
// pub struct AsymmetricKeyPair {
//     private_key: Vec<u8>,
//     public_key: Vec<u8>,
// }
//
// impl AsymmetricKeyPair {
//     /// Creates a new `AsymmetricKeyPair` from byte vectors for the private and public keys.
//     pub fn new(private_key: Vec<u8>, public_key: Vec<u8>) -> Self {
//         Self {
//             private_key,
//             public_key,
//         }
//     }
//
//     /// Loads an `AsymmetricKeyPair` from files for the private and public keys.
//     pub fn from_files(private_key_path: &Path, public_key_path: &Path) -> Result<Self> {
//         let private_key = Self::load_key_from_file(private_key_path, "private")?;
//         let public_key = Self::load_key_from_file(public_key_path, "public")?;
//         Ok(Self::new(private_key, public_key))
//     }
//
//     /// Loads a key from a file.
//     fn load_key_from_file(path: &Path, key_type: &str) -> Result<Vec<u8>> {
//         let mut file = File::open(path).with_context(|| format!("Failed to open {} key file: {:?}", key_type, path))?;
//         let mut key = Vec::new();
//         file.read_to_end(&mut key).with_context(|| format!("Failed to read {} key from file: {:?}", key_type, path))?;
//         Ok(key)
//     }
//
//     /// Validates the sizes of the asymmetric keys.
//     pub fn validate_key_pair(&self) -> Result<()> {
//         if self.private_key.len() >= 64 && self.public_key.len() >= 32 {
//             Ok(())
//         } else {
//             Err(anyhow::anyhow!("Invalid key pair sizes: expected at least 64 bytes for private key and 32 bytes for public key"))
//         }
//     }
//
//     /// Returns the private key as a byte slice.
//     pub fn private_key_as_slice(&self) -> &[u8] {
//         &self.private_key
//     }
//
//     /// Returns the public key as a byte slice.
//     pub fn public_key_as_slice(&self) -> &[u8] {
//         &self.public_key
//     }
// }
//
// impl fmt::Debug for AsymmetricKeyPair {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "AsymmetricKeyPair(private: {} bytes, public: {} bytes)", self.private_key.len(), self.public_key.len())
//     }
// }
