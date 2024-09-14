use serde::{Deserialize, Serialize};

/// Represents an OAuth2 success response.
///
/// # Resources
/// - https://www.oauth.com/oauth2-servers/access-tokens/access-token-response/
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenResponse {
    /// The access token.
    #[serde(rename = "access_token")]
    pub token: String,

    /// The type of the token (e.g., Bearer).
    #[serde(rename = "token_type")]
    pub token_type: String,

    /// The number of seconds until the token expires.
    #[serde(rename = "expires_in", skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<u64>, // BUT RECOMMENDED

    /// The refresh token.
    #[serde(rename = "refresh_token", skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,

    /// The scope of the token.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
