use config::{Config, ConfigError, Environment, File};
use secrecy::Secret;
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use ::tracing::warn;

const DEFAULT_SITE_URL: &str = "http://localhost:3000";


fn default_site_url() -> String {
    DEFAULT_SITE_URL.to_string()
}

pub mod utils;
mod mailer;
mod fallback;
mod token;
mod smtp;
pub mod env;
mod tracing;
mod observability;
mod auth;
mod oauth2;

use crate::config::oauth2::OAuth2Config;
pub use mailer::*;
pub use observability::*;
pub use smtp::*;
pub use token::*;
pub use tracing::*;
pub use oauth2::*;

const CONFIG_FILE_NAME: &str = "rustproof";


#[derive(Debug, Deserialize, Serialize)]
pub struct RustproofConfig {
    #[serde(default)]
    pub external: External,
    #[serde(default = "default_site_url")]
    pub site_url: String,
    pub uri_allow_list: Option<String>,
    pub operator_token: Option<String>,
    pub disable_signup: Option<bool>,
    #[serde(default)]
    pub oauth2: OAuth2Config,
    #[serde(default)]
    pub smtp: Option<SmtpConfig>,
    #[serde(default)]
    pub mailer: MailerConfig,
    #[serde(default)]
    pub auth: AuthConfig,
    // #[serde(default)]
    pub database: DatabaseConfig,
    #[serde(default)]
    pub logging: LoggingConfig,
    #[serde(default)]
    pub tracing: Option<TracingConfig>,
    #[serde(default)]
    pub metrics: Option<MetricsConfig>,
    #[serde(default)]
    pub rate_limiting: RateLimitingConfig,
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub security: SecurityConfig,
    #[serde(default)]
    pub sms: SmsConfig,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct External {
    #[serde(default)]
    pub apple: bool,
    #[serde(default)]
    pub azure: bool,
    #[serde(default)]
    pub bitbucket: bool,
    #[serde(default)]
    pub discord: bool,
    #[serde(default)]
    pub facebook: bool,
    #[serde(default)]
    pub figma: bool,
    #[serde(default)]
    pub github: bool,
    #[serde(default)]
    pub gitlab: bool,
    #[serde(default)]
    pub google: bool,
    #[serde(default)]
    pub keycloak: bool,
    #[serde(default)]
    pub linkedin: bool,
    #[serde(default)]
    pub notion: bool,
    #[serde(default)]
    pub slack: bool,
    #[serde(default)]
    pub spotify: bool,
    #[serde(default)]
    pub twitch: bool,
    #[serde(default)]
    pub twitter: bool,
    #[serde(default)]
    pub workos: bool,
    #[serde(default = "crate::utils::serde::default_true")]
    pub email: bool,
    #[serde(default)]
    pub phone: bool,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct AuthConfig {
    #[serde(default)]
    pub anonymous_users_enabled: bool,
    #[serde(default = "crate::utils::serde::default_6_u8")]
    pub password_min_length: u8,
    /// A string of character sets separated by ":". A password must contain at least one
    /// character of each set to be accepted. To use the ":" character, escape it with \.
    pub password_required_characters: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    #[serde(default = "default_database_driver")]
    pub driver: DatabaseDriver,
    #[serde(serialize_with = "crate::utils::serde::serialize_secret_redacted")]
    pub connection_string: Secret<String>,
    pub max_pool_size: Option<u32>,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
    #[serde(default = "default_namespace")]
    pub namespace: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseDriver {
    Postgres,
    Sqlite,
    #[default]
    NotSet,
    #[serde(other)]
    Other,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct RateLimitingConfig {
    #[serde(default = "crate::utils::serde::default_rate_limit_header")]
    pub rate_limit_header: String,
    /// The maximum number of requests that can be made in a given time period.
    #[serde(default = "crate::utils::serde::default_rate_limit_email_sent_hourly")]
    pub rate_limit_email_sent_hourly: u32,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ServerConfig {
    pub api_host: Option<String>,
    /// Port number to listen on for HTTP. Defaults to 8081.
    #[serde(default = "crate::utils::serde::default_http_port")]
    pub http_port: u16,
    /// Port number to listen on for gRPC. Defaults to 50051.
    #[serde(default = "crate::utils::serde::default_grpc_port")]
    pub grpc_port: u16,
    /// Controls what endpoint Netlify can access this API on. (Multi-instance deployments)
    #[serde(default)]
    pub api_endpoint: Option<String>,
    /// If you wish to inherit a request ID from the incoming request, specify the name in this value.
    #[serde(default)]
    pub request_id_header: Option<String>,
    /// The URL on which Rustproof might be accessed at.
    #[serde(default = "crate::utils::serde::default_api_external_url")]
    pub api_external_url: Option<String>,
    /// Enable or disable the HTTP server.
    #[serde(default = "crate::utils::serde::default_enable_http")]
    pub enable_http: bool,
    /// Enable or disable the gRPC server.
    #[serde(default = "crate::utils::serde::default_enable_grpc")]
    pub enable_grpc: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[derive(Default)]
pub struct SecurityConfig {
    pub captcha_enabled: Option<bool>,
    pub captcha_provider: Option<String>,
    #[serde(serialize_with = "crate::utils::serde::serialize_option_secret_redacted")]
    pub captcha_secret: Option<Secret<String>>,
    pub captcha_timeout: Option<String>,
    #[serde(default = "crate::utils::serde::default_false")]
    pub update_password_require_reauthentication: bool,
    #[serde(default = "crate::utils::serde::default_true")]
    pub refresh_token_rotation_enabled: bool,
    #[serde(default = "crate::utils::serde::default_10_u64")]
    pub refresh_token_reuse_interval: u64,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct SmsConfig {
    #[serde(default)]
    pub autoconfirm: bool,
    pub max_frequency: Option<u64>,
    pub otp_exp: u64,
    pub otp_length: Option<u8>,
    pub provider: Option<String>,
    pub twilio_account_sid: Option<String>,
    #[serde(serialize_with = "crate::utils::serde::serialize_option_secret_redacted")]
    pub twilio_auth_token: Option<Secret<String>>,
    pub twilio_message_service_sid: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct SMTP {
    pub user: String,
    pub pass: String,
    pub admin_email: String,
    pub test_list: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct MAIN {
    pub smtp: SMTP,
}


impl RustproofConfig {
    pub fn new_arc<P: AsRef<Path>>(config_path: Option<P>) -> Result<Arc<Self>, ConfigError> {
        let config = RustproofConfig::new(config_path)?;
        Ok(Arc::new(config))
    }
    pub fn new<P: AsRef<Path>>(config_path: Option<P>) -> Result<Self, ConfigError> {
        let run_mode = std::env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        let mut builder = Config::builder()
            .add_source(File::from_str(include_str!("../../config/default.toml").into(), config::FileFormat::Toml));

        // // Add environment source
        let env = Environment::with_prefix("RUSTPROOF")
            .try_parsing(true)
            .separator("__")
            .list_separator(",")
            .with_list_parse_key("smtp.test_list");

        builder = builder.add_source(env);
        let s = builder.build()?;
        let config = s.try_deserialize::<RustproofConfig>()?;
        if config.site_url == DEFAULT_SITE_URL {
            warn!("Using default site URL '{}'. This should be changed for production use.", DEFAULT_SITE_URL);
        }
        Ok(config)
    }
}

fn default_database_driver() -> DatabaseDriver {
    DatabaseDriver::Postgres
}

fn default_namespace() -> Option<String> {
    Some("_auth".to_string())
}

fn default_max_connections() -> u32 {
    10
}
// println!("Config: {:?}", s);
// Print out our settings (as a HashMap)
// println!(
//     "{:#?}",
//     s.clone()
//         .try_deserialize::<HashMap<String, serde_json::Value>>()?
// );

// // Debug: Try to deserialize into a HashMap to see all key-value pairs
// let debug_config: HashMap<String, config::Value> = s.clone().try_deserialize()?;
// for (key, value) in &debug_config {
//     println!("Config key: {}, value: {:?}", key, value);
// }

// Usage in main.rs
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let config_path = std::env::var("RUSTPROOF_CONFIG_PATH").ok();
//     let config = RustproofConfig::new(config_path.as_ref())?;
//
//     // Use config...
//     Ok(())
// }   //
//         // println!("Run mode: {}", run_mode);
//         // let source = Environment::default()
//         //     .separator("__")
//         //     .list_separator(",")
//         //     .with_list_parse_key("smtp.test_list")
//         //     .try_parsing(true)
//         //     .source(Some({
//         //         let mut env = HashMap::new();
//         //         env.insert("SMTP__USER".into(), "my-value".into());
//         //         env.insert("SMTP__PASS".into(), "my-secret".into());
//         //         env.insert("SMTP__ADMIN_EMAIL".into(), "test@gmail.com".into());
//         //         env.insert("SMTP__TEST_LIST".into(), "test1,test2,test3".into());
//         //         env
//         //     }));
//         //
//         // println!("Environment source: {:?}", source);
//         // let config_test = Config::builder()
//         //     .add_source(source)
//         //     .build()?;
//
//         // Print out our settings (as a HashMap)
//         // println!(
//         //     "{:#?}",
//         //     config_test.clone()
//         //         .try_deserialize::<HashMap<String, serde_json::Value>>()?
//         // );
//
//         // let config_typed: MAIN = config_test.clone().try_deserialize::<MAIN>()?;
//         // println!("Config typed: {:#?}", config_typed);
//         // println!("Environment source: {:?}", source);
//         // ?.try_deserialize()?;
//
//         // println!("Config test: {:?}", config_test);