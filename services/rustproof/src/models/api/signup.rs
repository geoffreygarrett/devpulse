use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Parameters for signing up a new user.
#[derive(Debug, Serialize, Deserialize)]
pub struct SignupParams {
    /// User's email address.
    pub email: String,

    /// User's password.
    pub password: String,

    /// User's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Additional user data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, serde_json::Value>>,

    /// Provider information (not serialized).
    #[serde(skip_serializing)]
    pub provider: Option<String>,

    /// Audience information (not serialized).
    #[serde(skip_serializing)]
    pub aud: Option<String>,

    /// Invitation token (not serialized).
    #[serde(skip_serializing)]
    pub invitation_token: Option<String>,
}
