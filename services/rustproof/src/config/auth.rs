use secrecy::Secret;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct AuthConfig {
    // Access token configuration
    access_token_driver: TokenDriver,
    access_token_expiration_time: u64,
    access_token_audience: String,
    access_token_admin_group_name: String,
    access_token_default_group_name: String,

    // Refresh token configuration
    refresh_token_rotation_enabled: bool,
    refresh_token_reuse_interval: u64,

    // Other authentication settings
    anonymous_users_enabled: Option<bool>,
    password_min_length: Option<usize>,
    password_required_characters: Option<Vec<char>>,

    // Signup status
    signup_status: SignupStatus,
}

#[derive(Debug, Deserialize, Serialize)]
enum SignupStatus {
    Open,
    Closed,
    InviteOnly,
}

#[derive(Debug, Deserialize, Serialize)]
enum TokenDriver {
    Jwt(JwtConfig),
    // Other token drivers can be added here
}

#[derive(Debug, Deserialize, Serialize)]
struct JwtConfig {
    #[serde(serialize_with = "crate::utils::serde::serialize_secret_redacted")]
    jwt_secret: Secret<String>,
}

