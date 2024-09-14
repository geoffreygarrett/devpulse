use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
// https://www.iana.org/assignments/jwt/jwt.xhtml#claims
// https://id4d.worldbank.org/guide/levels-assurance-loas

/// Represents the claims stored in the JWT.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct RustProofClaims {
    // Restricted PASETO claims
    #[serde(rename = "iss", skip_serializing_if = "Option::is_none")]
    pub issuer: Option<String>,

    #[serde(rename = "sub", skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,

    #[serde(rename = "aud", skip_serializing_if = "Option::is_none")]
    pub audience: Option<String>,

    #[serde(rename = "exp", skip_serializing_if = "Option::is_none")]
    pub expiration: Option<u64>,

    #[serde(rename = "nbf", skip_serializing_if = "Option::is_none")]
    pub not_before: Option<u64>,

    #[serde(rename = "iat", skip_serializing_if = "Option::is_none")]
    pub issued_at: Option<u64>,

    #[serde(rename = "jti", skip_serializing_if = "Option::is_none")]
    pub jwt_id: Option<String>,

    #[serde(rename = "kid", skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,

    // JWT Registered claims rest
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,

    #[serde(rename = "org", skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,

    #[serde(rename = "roles", skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,

    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,

    #[serde(rename = "given_name", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,

    #[serde(rename = "family_name", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,

    #[serde(rename = "middle_name", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,

    #[serde(rename = "nickname", skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,

    #[serde(rename = "app_metadata")]
    pub app_metadata: HashMap<String, Value>,

    #[serde(rename = "user_metadata")]
    pub user_metadata: HashMap<String, Value>,

    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}
