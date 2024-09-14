use crate::config::fallback::*;
use lazy_static::lazy_static;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use tokio::time::{Duration, Instant};

// Enum representing different sources for mailer content (templates)
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum MailerTemplateSource {
    Url(String),  // Templated URL with {{locale}}, {{format}}, and {{type}} available
    Path(String), // Templated file path with {{locale}}, {{format}}, and {{type}} available
    #[default]
    Fallback,     // Fallback to compile-time strings
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum MailerSubjectSource {
    Url(String),  // Templated URL with {{locale}} and {{type}} available
    Path(String), // Templated file path with {{locale}} and {{type}} available
    #[default]
    Fallback,     // Fallback to compile-time strings
}

// Enum for different template types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TemplateType {
    Invite,
    Confirmation,
    Recovery,
    MagicLink,
    EmailChange,
    Generic,
}

impl TemplateType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TemplateType::Invite => "invite",
            TemplateType::Confirmation => "confirmation",
            TemplateType::Recovery => "recovery",
            TemplateType::MagicLink => "magic_link",
            TemplateType::EmailChange => "email_change",
            TemplateType::Generic => "generic",
        }
    }
}
serde_with::with_prefix!(confirmation_ "confirmation_");
serde_with::with_prefix!(recovery_ "recovery_");
serde_with::with_prefix!(magic_link_ "magic_link_");
serde_with::with_prefix!(email_change_ "email_change_");
serde_with::with_prefix!(invite_ "invite_");

// Struct for holding the configuration of mailer templates
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MailerTemplatesConfig {
    #[serde(default = "default_invite_template")]
    pub invite: MailerTemplateSource,
    #[serde(default = "default_confirmation_template")]
    pub confirmation: MailerTemplateSource,
    #[serde(default = "default_recovery_template")]
    pub recovery: MailerTemplateSource,
    #[serde(default = "default_magic_link_template")]
    pub magic_link: MailerTemplateSource,
    #[serde(default = "default_email_change_template")]
    pub email_change: MailerTemplateSource,
}

// Struct for holding the configuration of mailer subjects
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MailerSubjectsConfig {
    #[serde(default = "default_mailer_subject_invite")]
    pub invite: MailerSubjectSource,
    #[serde(default = "default_mailer_subject_confirmation")]
    pub confirmation: MailerSubjectSource,
    #[serde(default = "default_mailer_subject_recovery")]
    pub recovery: MailerSubjectSource,
    #[serde(default = "default_mailer_subject_magic_link")]
    pub magic_link: MailerSubjectSource,
    #[serde(default = "default_mailer_subject_email_change")]
    pub email_change: MailerSubjectSource,
}

// Struct for holding the configuration of mailer URL paths
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct MailerUrlPathsConfig {
    pub invite: Option<String>,         // MAILER_URLPATHS_INVITE
    pub confirmation: Option<String>,   // MAILER_URLPATHS_CONFIRMATION
    pub recovery: Option<String>,       // MAILER_URLPATHS_RECOVERY
    pub email_change: Option<String>,   // MAILER_URLPATHS_EMAIL_CHANGE
}

serde_with::with_prefix!(templates_ "templates_");

// Struct for storing the complete mailer configuration with proper defaults
#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub struct MailerConfig {
    #[serde(default)]
    pub autoconfirm: bool,

    #[serde(default = "default_otp_exp")]
    pub otp_exp: u64,

    pub url_paths: MailerUrlPathsConfig,

    pub subjects: MailerSubjectsConfig, // Subjects configuration (plain text only)

    pub templates: MailerTemplatesConfig, // Templates configuration

    pub localization: bool,
}

impl MailerConfig {
    pub fn get_template_source(&self, template_type: TemplateType) -> &MailerTemplateSource {
        match template_type {
            TemplateType::Invite => &self.templates.invite,
            TemplateType::Confirmation => &self.templates.confirmation,
            TemplateType::Recovery => &self.templates.recovery,
            TemplateType::MagicLink => &self.templates.magic_link,
            TemplateType::EmailChange => &self.templates.email_change,
            _ => &MailerTemplateSource::Fallback,
        }
    }

    pub fn get_subject_source(&self, template_type: TemplateType) -> &MailerSubjectSource {
        match template_type {
            TemplateType::Invite => &self.subjects.invite,
            TemplateType::Confirmation => &self.subjects.confirmation,
            TemplateType::Recovery => &self.subjects.recovery,
            TemplateType::MagicLink => &self.subjects.magic_link,
            TemplateType::EmailChange => &self.subjects.email_change,
            _ => &MailerSubjectSource::Fallback,
        }
    }
}

// Caching structure for fetched content
struct CachedContent {
    content: String,
    expiry: Instant,
}

// Shared global cache with an RwLock for async safety
lazy_static! {
    static ref CACHE: RwLock<HashMap<String, CachedContent>> = RwLock::new(HashMap::new());
    static ref HTTP_CLIENT: Client = Client::new();
}

// Fetch content from a URL with caching
async fn fetch_content_from_url(url: &str) -> String {
    let mut cache = CACHE.write().await;
    let now = Instant::now();

    // Check cache first
    if let Some(cached) = cache.get(url) {
        if cached.expiry > now {
            return cached.content.clone();
        }
    }

    // Fetch from URL
    let response = HTTP_CLIENT.get(url).send().await;
    match response {
        Ok(res) => {
            if let Ok(body) = res.text().await {
                // Cache the result with an expiry time (e.g., 1 hour)
                let expiry = now + Duration::from_secs(3600);
                cache.insert(url.to_string(), CachedContent { content: body.clone(), expiry });
                body
            } else {
                String::new()
            }
        }
        Err(_) => String::new(),
    }
}

// Trait to handle content retrieval based on localization settings
#[async_trait::async_trait]
pub trait MailerContent {
    async fn get_subject(&self, template_type: TemplateType, locale: &str) -> String;
    async fn get_template_content(&self, template_type: TemplateType, locale: &str) -> (String, String);
}

#[async_trait::async_trait]
impl MailerContent for MailerConfig {
    async fn get_subject(&self, template_type: TemplateType, locale: &str) -> String {
        match self.get_subject_source(template_type) {
            MailerSubjectSource::Url(url) => fetch_content_from_url(url).await,
            MailerSubjectSource::Path(path) => std::fs::read_to_string(path).unwrap_or_else(|_| get_fallback_subject(locale, template_type).to_string()),
            MailerSubjectSource::Fallback => get_fallback_subject(locale, template_type).to_string(),
        }
    }

    async fn get_template_content(&self, template_type: TemplateType, locale: &str) -> (String, String) {
        match self.get_template_source(template_type) {
            MailerTemplateSource::Url(url) => {
                let plain = fetch_content_from_url(&url).await;
                let html = fetch_content_from_url(&url).await;
                (plain, html)
            }
            MailerTemplateSource::Path(path) => {
                let plain = std::fs::read_to_string(path.clone()).unwrap_or_else(|_| get_fallback_template(locale, template_type).0.to_string());
                let html = std::fs::read_to_string(path).unwrap_or_else(|_| get_fallback_template(locale, template_type).1.to_string());
                (plain, html)
            }
            MailerTemplateSource::Fallback => {
                let (plain, html) = get_fallback_template(locale, template_type);
                (plain.to_string(), html.to_string())
            }
        }
    }
}

// Default values for enabling localization
fn default_enable_localization() -> bool {
    true
}

// Default functions for each template type
fn default_invite_template() -> MailerTemplateSource {
    MailerTemplateSource::Fallback
}

fn default_confirmation_template() -> MailerTemplateSource {
    MailerTemplateSource::Fallback
}

fn default_recovery_template() -> MailerTemplateSource {
    MailerTemplateSource::Fallback
}

fn default_magic_link_template() -> MailerTemplateSource {
    MailerTemplateSource::Fallback
}

fn default_email_change_template() -> MailerTemplateSource {
    MailerTemplateSource::Fallback
}

fn default_mailer_subject_invite() -> MailerSubjectSource {
    MailerSubjectSource::Fallback
}

fn default_mailer_subject_confirmation() -> MailerSubjectSource {
    MailerSubjectSource::Fallback
}

fn default_mailer_subject_recovery() -> MailerSubjectSource {
    MailerSubjectSource::Fallback
}

fn default_mailer_subject_magic_link() -> MailerSubjectSource {
    MailerSubjectSource::Fallback
}

fn default_mailer_subject_email_change() -> MailerSubjectSource {
    MailerSubjectSource::Fallback
}

fn default_autoconfirm() -> bool {
    true
}

fn default_otp_exp() -> u64 {
    300 // 5 minutes
}
