// use secrecy::{ExposeSecret, Secret};
// use serde::{Deserialize, Serialize};
// serde_with::with_prefix!(token_ "token_");
//
// // Default values
// const DEFAULT_EXPIRATION: u64 = 3600; // 1 hour
// const DEFAULT_REFRESH_TOKEN_REUSE_INTERVAL: u64 = 10; // 10 seconds
// const DEFAULT_AUDIENCE: &str = "authenticated"; // Default audience
// const DEFAULT_ADMIN_GROUP_NAME: &str = "admin"; // Default admin group name
// const DEFAULT_DEFAULT_GROUP_NAME: &str = "authenticated"; // Default group name
//
// // Common fields for token configurations
// #[derive(Serialize, Deserialize, Default, Debug)]
// pub struct CommonTokenConfig {
//     #[serde(default = "default_expiration", rename = "exp")]
//     pub expiration_time: u64, // Common to JWT and PASETO
//
//     #[serde(default = "default_audience", rename = "aud")]
//     pub audience: String, // Common to JWT and PASETO
//
//     #[serde(default = "default_admin_group_name")]
//     pub admin_group_name: String, // Common to JWT and PASETO
//
//     #[serde(default = "default_default_group_name")]
//     pub default_group_name: String, // Common to JWT and PASETO
//
//     #[serde(default = "default_refresh_token_rotation_enabled")]
//     pub refresh_token_rotation_enabled: bool, // REFRESH_TOKEN_ROTATION_ENABLED
//
//     #[serde(default = "default_refresh_token_reuse_interval")]
//     pub refresh_token_reuse_interval: u64, // REFRESH_TOKEN_REUSE_INTERVAL
// }
//
// // JWT-specific configuration
// #[derive(Serialize, Deserialize, Debug)]
// pub struct JwtConfig {
//     #[serde(skip_serializing, serialize_with = "crate::config::utils::serialize_secret")]
//     pub jwt_secret: Secret<String>,
// }
//
// // Paseto-specific configuration
// #[derive(Serialize, Deserialize, Debug)]
// pub struct PasetoConfig {
//     #[serde(skip_serializing)]
//     pub paseto_secret: Secret<String>,
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(tag = "driver", rename_all = "snake_case")]
// pub enum TokenDriver {
//     Jwt(JwtConfig),
//     Paseto(PasetoConfig),
// }
//
// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "snake_case")]
// pub struct AccessTokenConfig {
//     pub driver: TokenDriver,
//
//     #[serde(default = "default_expiration", rename = "exp")]
//     pub expiration_time: u64, // Common to JWT and PASETO
//
//     #[serde(default = "default_audience", rename = "aud")]
//     pub audience: String, // Common to JWT and PASETO
//
//     #[serde(default = "default_admin_group_name")]
//     pub admin_group_name: String, // Common to JWT and PASETO
//
//     #[serde(default = "default_default_group_name")]
//     pub default_group_name: String, // Common to JWT and PASETO
//
//     #[serde(default = "default_refresh_token_rotation_enabled")]
//     pub refresh_token_rotation_enabled: bool, // REFRESH_TOKEN_ROTATION_ENABLED
//
//     #[serde(default = "default_refresh_token_reuse_interval")]
//     pub refresh_token_reuse_interval: u64, // REFRESH_TOKEN_REUSE_INTERVAL
// }
//
// // Default values functions
// fn default_expiration() -> u64 {
//     DEFAULT_EXPIRATION
// }
//
// fn default_audience() -> String {
//     DEFAULT_AUDIENCE.to_string()
// }
//
// fn default_admin_group_name() -> String {
//     DEFAULT_ADMIN_GROUP_NAME.to_string()
// }
//
// fn default_default_group_name() -> String {
//     DEFAULT_DEFAULT_GROUP_NAME.to_string()
// }
//
// fn default_refresh_token_reuse_interval() -> u64 {
//     DEFAULT_REFRESH_TOKEN_REUSE_INTERVAL
// }
//
// fn default_refresh_token_rotation_enabled() -> bool {
//     true
// }
