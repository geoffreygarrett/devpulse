use secrecy::Secret;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// Default values
const DEFAULT_ACCESS_TOKEN_EXPIRATION: u64 = 3600; // 1 hour
const DEFAULT_REFRESH_TOKEN_EXPIRATION: u64 = 2592000; // 30 days
const DEFAULT_REFRESH_TOKEN_REUSE_INTERVAL: u64 = 60; // 1 minute
const DEFAULT_AUDIENCE: &str = "authenticated";
const DEFAULT_ADMIN_GROUP_NAME: &str = "admin";
const DEFAULT_DEFAULT_GROUP_NAME: &str = "authenticated";

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct OAuth2Config {
    pub issuer: String,
    #[serde(default)]
    pub jwks_uri: Option<String>,
    #[serde(default)]
    pub scopes_supported: HashSet<String>,
    #[serde(default)]
    pub response_types_supported: HashSet<String>,
    #[serde(default)]
    pub token_endpoint_auth_methods_supported: HashSet<String>,
    #[serde(flatten, default)]
    pub access_token: AccessTokenConfig,
    #[serde(default)]
    pub refresh_token: RefreshTokenConfig,
    #[serde(default)]
    pub grants: GrantsConfig,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct AccessTokenConfig {
    #[serde(flatten, default)]
    pub driver: TokenDriver,

    #[serde(default = "default_access_token_expiration", rename = "exp")]
    pub expiration_time: u64,

    #[serde(default = "default_audience", rename = "aud")]
    pub audience: String,

    #[serde(default = "default_admin_group_name")]
    pub admin_group_name: String,

    #[serde(default = "default_default_group_name")]
    pub default_group_name: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct RefreshTokenConfig {
    #[serde(default = "default_refresh_token_expiration", rename = "refresh_token_exp")]
    pub expiration_time: u64,

    #[serde(default = "default_refresh_token_rotation_enabled")]
    pub rotation_enabled: bool,

    #[serde(default = "default_refresh_token_reuse_interval")]
    pub reuse_interval: u64,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct GrantsConfig {
    #[serde(default)]
    pub authorization_code: AuthorizationCodeConfig,
    #[serde(default)]
    pub client_credentials: ClientCredentialsConfig,
    #[serde(default)]
    pub refresh_token: RefreshTokenGrantConfig,
    #[serde(default)]
    pub implicit: ImplicitConfig,
    #[serde(default)]
    pub password: PasswordConfig,
    #[serde(default)]
    pub device_code: DeviceCodeConfig,
    #[serde(default)]
    pub jwt_bearer: JwtBearerConfig,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct AuthorizationCodeConfig {
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ClientCredentialsConfig {
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct RefreshTokenGrantConfig {
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ImplicitConfig {
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct PasswordConfig {
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DeviceCodeConfig {
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct JwtBearerConfig {
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "driver", rename_all = "snake_case")]
pub enum TokenDriver {
    Jwt(JwtConfig),
    Paseto(PasetoConfig),
}

impl Default for TokenDriver {
    fn default() -> Self {
        TokenDriver::Jwt(JwtConfig {
            jwt_secret: Secret::new("".to_string()),
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JwtConfig {
    #[serde(skip_serializing, serialize_with = "crate::config::utils::serialize_secret")]
    pub jwt_secret: Secret<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PasetoConfig {
    #[serde(skip_serializing)]
    pub paseto_secret: Secret<String>,
}

// Default value functions
fn default_access_token_expiration() -> u64 { DEFAULT_ACCESS_TOKEN_EXPIRATION }
fn default_refresh_token_expiration() -> u64 { DEFAULT_REFRESH_TOKEN_EXPIRATION }
fn default_audience() -> String { DEFAULT_AUDIENCE.to_string() }
fn default_admin_group_name() -> String { DEFAULT_ADMIN_GROUP_NAME.to_string() }
fn default_default_group_name() -> String { DEFAULT_DEFAULT_GROUP_NAME.to_string() }
fn default_refresh_token_rotation_enabled() -> bool { false }
fn default_refresh_token_reuse_interval() -> u64 { DEFAULT_REFRESH_TOKEN_REUSE_INTERVAL }
//
// // Example usage
// fn main() {
//     let config = OAuth2Config {
//         issuer: "https://example.com".to_string(),
//         jwks_uri: "https://example.com/.well-known/jwks.json".to_string(),
//         scopes_supported: vec!["openid".to_string(), "profile".to_string()],
//         response_types_supported: vec!["code".to_string(), "token".to_string()],
//         token_endpoint_auth_methods_supported: vec!["client_secret_basic".to_string()],
//         access_token: AccessTokenConfig {
//             driver: TokenDriver::Jwt(JwtConfig {}),
//             expiration_time: 3600,
//             audience: "https://api.example.com".to_string(),
//             admin_group_name: "admin".to_string(),
//             default_group_name: "users".to_string(),
//         },
//         refresh_token: RefreshTokenConfig {
//             expiration_time: 2592000,
//             rotation_enabled: true,
//             reuse_interval: 10,
//         },
//         grants: GrantsConfig {
//             authorization_code: AuthorizationCodeConfig { enabled: true },
//             client_credentials: ClientCredentialsConfig { enabled: true },
//             refresh_token: RefreshTokenGrantConfig { enabled: true },
//             implicit: ImplicitConfig { enabled: false },
//             password: PasswordConfig { enabled: false },
//             device_code: DeviceCodeConfig { enabled: false },
//             jwt_bearer: JwtBearerConfig { enabled: false },
//         },
//     };
//
//     println!("{:#?}", config);
// }