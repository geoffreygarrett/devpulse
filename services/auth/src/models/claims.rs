use serde::{Deserialize, Serialize};

/// Represents the claims stored in the JWT.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub(crate) sub: String,
    pub(crate) exp: u64,
    pub(crate) roles: Vec<String>,
    pub(crate) org: Option<String>,
    pub(crate) iat: Option<u64>,
    pub(crate) iss: Option<String>,
    pub(crate) aud: Option<String>,
    pub(crate) jti: Option<String>,
}
