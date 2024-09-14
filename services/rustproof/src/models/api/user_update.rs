use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Parameters for updating a user.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdateParams {
    /// User's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// User's password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,

    /// Token for email change verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_change_token: Option<String>,

    /// Token for phone change verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_change_token: Option<String>,

    /// Additional user data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<HashMap<String, serde_json::Value>>,

    /// Optional application-specific metadata.
    #[serde(rename = "app_metadata", skip_serializing_if = "Option::is_none")]
    pub app_data: Option<HashMap<String, serde_json::Value>>,
}
