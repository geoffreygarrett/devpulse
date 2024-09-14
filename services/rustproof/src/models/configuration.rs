// use serde::{Deserialize, Serialize};
// use serde_with::{serde_as, DurationSeconds};
// use chrono::Duration;
// use std::collections::HashMap;
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct OAuthProviderConfiguration {
//     pub client_id: String,
//     pub secret: String,
//     pub redirect_uri: String,
//     pub url: String,
//     pub enabled: bool,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct EmailProviderConfiguration {
//     pub disabled: bool,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct SamlProviderConfiguration {
//     pub enabled: bool,
//     pub metadata_url: String,
//     #[serde(rename = "API_BASE")]
//     pub api_base: String,
//     #[serde(rename = "SIGNING_CERT")]
//     pub signing_cert: String,
//     #[serde(rename = "SIGNING_KEY")]
//     pub signing_key: String,
//     pub name: String,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct DBConfiguration {
//     pub driver: String,
//     #[serde(rename = "DATABASE_URL")]
//     pub url: String,
//     pub namespace: Option<String>,
//     #[serde(default = "default_migrations_path")]
//     pub migrations_path: String,
// }
//
// fn default_migrations_path() -> String {
//     "./migrations".to_string()
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct JWTConfiguration {
//     pub secret: String,
//     pub exp: Option<i64>,
//     pub aud: Option<String>,
//     #[serde(rename = "admin_group_name")]
//     pub admin_group_name: Option<String>,
//     #[serde(rename = "default_group_name")]
//     pub default_group_name: Option<String>,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct EmailContentConfiguration {
//     pub invite: String,
//     pub confirmation: String,
//     pub recovery: String,
//     #[serde(rename = "email_change")]
//     pub email_change: String,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct ProviderConfiguration {
//     pub bitbucket: OAuthProviderConfiguration,
//     pub github: OAuthProviderConfiguration,
//     pub gitlab: OAuthProviderConfiguration,
//     pub google: OAuthProviderConfiguration,
//     pub facebook: OAuthProviderConfiguration,
//     pub email: EmailProviderConfiguration,
//     pub saml: SamlProviderConfiguration,
//     pub redirect_url: String,
// }
//
// #[serde_as]
// #[derive(Debug, Serialize, Deserialize)]
// pub struct SMTPConfiguration {
//     #[serde_as(as = "DurationSeconds<i64>")]
//     #[serde(rename = "max_frequency")]
//     pub max_frequency: Duration,
//     pub host: String,
//     #[serde(default = "default_smtp_port")]
//     pub port: u16,
//     pub user: String,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub pass: Option<String>,
//     #[serde(rename = "admin_email")]
//     pub admin_email: String,
// }
//
// fn default_smtp_port() -> u16 {
//     587
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct MailerConfiguration {
//     pub autoconfirm: bool,
//     pub subjects: EmailContentConfiguration,
//     pub templates: EmailContentConfiguration,
//     pub url_paths: EmailContentConfiguration,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct CookieConfiguration {
//     pub key: String,
//     pub duration: i32,
// }
//
// ///////////
// #[derive(Debug, Serialize, Deserialize)]
// struct WebhookConfig {
//     url: String,
//     secret: String,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// struct LoggingConfig {
//     level: String,
//     format: String,
// }
//
//
// #[derive(Debug, Serialize, Deserialize)]
// struct TracingConfig {
//     enabled: bool,
//     service_name: String,
//     agent_host: String,
//     agent_port: u16,
// }
// ///////////
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Configuration {
//     #[serde(rename = "site_url")]
//     pub site_url: String,
//     pub jwt: JWTConfiguration,
//     pub smtp: SMTPConfiguration,
//     pub mailer: MailerConfiguration,
//     pub external: ProviderConfiguration,
//     #[serde(rename = "disable_signup")]
//     pub disable_signup: bool,
//     pub webhook: Option<WebhookConfig>, // Assuming you have a WebhookConfig struct
//     pub cookies: CookieConfiguration,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct GlobalConfiguration {
//     pub api: ApiConfiguration,
//     pub db: DBConfiguration,
//     pub external: ProviderConfiguration,
//     pub logging: LoggingConfig, // Assuming you have a LoggingConfig struct
//     #[serde(rename = "operator_token")]
//     pub operator_token: String,
//     pub multi_instance_mode: bool,
//     pub tracing: TracingConfig, // Assuming you have a TracingConfig struct
//     pub smtp: SMTPConfiguration,
//     #[serde(rename = "rate_limit_header")]
//     pub rate_limit_header: String,
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct ApiConfiguration {
//     pub host: String,
//     #[serde(default = "default_port")]
//     pub port: u16,
//     pub endpoint: String,
//     #[serde(rename = "request_id_header")]
//     pub request_id_header: Option<String>,
// }
//
// fn default_port() -> u16 {
//     8081
// }
