// mod mailer;

use async_trait::async_trait;
use axum::response::Response;
use serde_json::json;
use std::collections::HashMap;
use std::io;
use tracing::{error, info, instrument};


#[derive(Debug, Snafu)]
pub enum SmtpError {
    #[snafu(display("Failed to create SMTP relay: {}", source))]
    RelayCreation { source: lettre::transport::smtp::Error },

    #[snafu(display("SMTP connection failed: {}", source))]
    Connection { source: lettre::transport::smtp::Error },

    #[snafu(display("Failed to authenticate with SMTP server: {}", source))]
    Authentication { source: lettre::transport::smtp::Error },

    #[snafu(display("Failed to send email: {}", source))]
    SendFailure { source: lettre::transport::smtp::Error },

    #[snafu(display("Failed to parse SMTP response: {}", source))]
    SmtpResponseParse { source: lettre::transport::smtp::Error },

    #[snafu(display("Received unexpected SMTP response"))]
    UnexpectedResponse { response: Response },

    #[snafu(display("General SMTP error: {}", message))]
    GeneralSmtp { message: String },
}

#[derive(Debug, Snafu)]
pub enum MailerError {
    #[snafu(display("SMTP error occurred: {}", source))]
    Smtp { source: SmtpError },

    #[snafu(display("Failed to render email template: {}", source))]
    TemplateRender { source: RenderError },

    #[snafu(display("HTTP request failed: {}", source))]
    HttpRequest { source: reqwest::Error },

    #[snafu(display("IO error: {}", source))]
    Io { source: io::Error },

    #[snafu(display("Invalid email address: {}", source))]
    InvalidEmailAddress { source: AddressError },

    #[snafu(display("General mailer error: {}", message))]
    GeneralMailer { message: String },
}


#[async_trait]
pub trait Mailer: Send + Sync {
    async fn send_template_email(
        &self,
        template: EmailTemplate,
        locale: &str,
    ) -> Result<(), MailerError>;
}

#[derive(Debug)]
pub enum EmailTemplate {
    Confirmation {
        to_name: Option<String>,
        site_url: String,
        email: String,
        confirmation_url: String,
    },
    Invite {
        to_name: Option<String>,
        site_url: String,
        confirmation_url: String,
        email: String,
    },
    Recovery {
        to_name: Option<String>,
        site_url: String,
        recovery_url: String,
        email: String,
    },
    EmailChange {
        to_name: Option<String>,
        site_url: String,
        email: String,
        new_email: String,
        confirmation_url: String,
    },
    Generic {
        to_name: Option<String>,
        site_url: String,
        subject: String,
        body: String,
        email: String,
    },
}


impl EmailTemplate {
    pub fn new_confirmation(to_name: Option<String>, site_url: String, email: String, confirmation_url: String) -> Self {
        EmailTemplate::Confirmation { to_name, site_url, email, confirmation_url }
    }

    pub fn new_invite(to_name: Option<String>, site_url: String, email: String, confirmation_url: String) -> Self {
        EmailTemplate::Invite { to_name, site_url, email, confirmation_url }
    }

    pub fn new_recovery(to_name: Option<String>, site_url: String, email: String, recovery_url: String) -> Self {
        EmailTemplate::Recovery { to_name, site_url, email, recovery_url }
    }

    pub fn new_email_change(to_name: Option<String>, site_url: String, email: String, new_email: String, confirmation_url: String) -> Self {
        EmailTemplate::EmailChange { to_name, site_url, email, new_email, confirmation_url }
    }

    pub fn new_generic(to_name: Option<String>, site_url: String, subject: String, body: String, email: String) -> Self {
        EmailTemplate::Generic { to_name, site_url, subject, body, email }
    }


    pub fn subject(&self) -> &str {
        match self {
            EmailTemplate::Confirmation { .. } => "Please Confirm Your Email Address",
            EmailTemplate::Invite { .. } => "You're Invited!",
            EmailTemplate::Recovery { .. } => "Password Recovery Instructions",
            EmailTemplate::EmailChange { .. } => "Confirm Your Email Address Change",
            EmailTemplate::Generic { subject, .. } => subject,
        }
    }

    pub fn email(&self) -> &str {
        match self {
            EmailTemplate::Confirmation { email, .. } => email,
            EmailTemplate::Invite { email, .. } => email,
            EmailTemplate::Recovery { email, .. } => email,
            EmailTemplate::EmailChange { email, .. } => email,
            EmailTemplate::Generic { email, .. } => email,
        }
    }

    pub fn template_type(&self) -> TemplateType {
        match self {
            EmailTemplate::Confirmation { .. } => TemplateType::Confirmation,
            EmailTemplate::Invite { .. } => TemplateType::Invite,
            EmailTemplate::Recovery { .. } => TemplateType::Recovery,
            EmailTemplate::EmailChange { .. } => TemplateType::EmailChange,
            EmailTemplate::Generic { .. } => TemplateType::Generic,
        }
    }

    pub fn to_json(&self) -> serde_json::Value {
        match self {
            EmailTemplate::Confirmation {
                to_name,
                site_url,
                email,
                confirmation_url,
            } => json!({ "to_name": to_name, "site_url": site_url, "email": email, "confirmation_url": confirmation_url }),
            EmailTemplate::Invite {
                to_name,
                site_url,
                email,
                confirmation_url,
            } => json!({ "to_name": to_name, "site_url": site_url, "email": email, "confirmation_url": confirmation_url }),
            EmailTemplate::Recovery {
                to_name,
                site_url,
                email,
                recovery_url,
            } => json!({ "to_name": to_name, "site_url": site_url, "email": email, "recovery_url": recovery_url }),
            EmailTemplate::EmailChange {
                to_name,
                site_url,
                email,
                new_email,
                confirmation_url,
            } => json!({ "to_name": to_name, "site_url": site_url, "email": email, "new_email": new_email, "confirmation_url": confirmation_url }),
            EmailTemplate::Generic {
                to_name,
                site_url,
                subject,
                body,
                email,
            } => json!({ "to_name": to_name, "site_url": site_url, "subject": subject, "body": body, "email": email }),
        }
    }
}

use crate::config::{MailerConfig, MailerContent, MailerTemplateSource, SmtpConfig, TemplateType};
use crate::email::mailer::MailerError::GeneralMailer;
use handlebars::{Handlebars, RenderError};
use lettre::address::AddressError;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::SmtpTransportBuilder;
use lettre::{Message, SmtpTransport, Transport};
use reqwest::Client;
use secrecy::ExposeSecret;
use snafu::{ResultExt, Snafu};

pub struct SMTPMailer {
    config: MailerConfig,
    smtp_config: SmtpConfig,
    smtp_transport: SmtpTransport,
    handlebars: Handlebars<'static>,
    http_client: Client,
}

pub trait SmtpTransportBuilderExt {
    fn optional_credentials(self, creds: Option<Credentials>) -> Self;
}

impl SmtpTransportBuilderExt for SmtpTransportBuilder {
    fn optional_credentials(mut self, creds: Option<Credentials>) -> Self {
        if let Some(creds) = creds {
            self = self.credentials(creds);
        }
        self
    }
}

impl SMTPMailer {
    #[instrument(skip(smtp_config, mailer_config))]
    pub fn new(smtp_config: SmtpConfig, mailer_config: MailerConfig) -> Result<Self, String> {
        // Validation for user and password
        if smtp_config.user.is_some() != smtp_config.pass.is_some() {
            error!("Both SMTP user and password must be provided together.");
            return Err("Both SMTP user and password must be provided together.".into());
        }

        let creds = smtp_config
            .user
            .clone()
            .zip(smtp_config.pass.clone())
            .map(|(user, pass)| Credentials::new(user, pass.expose_secret().to_string()));

        let smtp_transport = SmtpTransport::relay(&*smtp_config.host.clone())
            .map_err(|e| format!("Failed to create relay: {}", e))?
            .port(smtp_config.port)
            .optional_credentials(creds)
            .build();

        let handlebars = Handlebars::new();
        let http_client = Client::new();

        info!("SMTPMailer created successfully with host: {}", smtp_config.host);

        Ok(Self {
            config: mailer_config,
            smtp_config,
            smtp_transport,
            handlebars,
            http_client,
        })
    }

    #[instrument(skip(self))]
    async fn fetch_template_content(&self, template_type: TemplateType, locale: &str) -> Result<(String, String), MailerError> {
        // Create the context for rendering with Handlebars
        let mut context = HashMap::new();
        context.insert("locale", locale);
        context.insert("format", "html"); // or "plain", depending on your logic
        context.insert("type", template_type.as_str());

        // Initialize Handlebars
        // let handlebars = Handlebars::new();
        let (plain_text, html_text) = self.config.get_template_content(template_type, locale).await;
        Ok((plain_text.to_string(), html_text.to_string()))
    }

    #[instrument(skip(self))]
    async fn send_email(
        &self,
        template: EmailTemplate,
        locale: &str,
    ) -> Result<(), MailerError> {
        let subject = template.subject();
        let (plain_template, html_template) = self.fetch_template_content(template.template_type(), locale)
            .await.map_err(|e| GeneralMailer { message: "Failed to fetch template content".to_string() })?;

        let vars = template.to_json();
        let to_email = template.email();
        let plain_body = self.render_template(&plain_template, &vars).await?;
        let html_body = self.render_template(&html_template, &vars).await?;

        let email = Message::builder()
            .from(format!("{} <{}>", self.smtp_config.sender_name.clone().unwrap_or(self.smtp_config.admin_email.clone()), self.smtp_config.admin_email).parse().unwrap())
            .to(to_email.parse().context(InvalidEmailAddressSnafu)?)
            .subject(subject)
            .multipart(lettre::message::MultiPart::alternative_plain_html(plain_body, html_body))
            .unwrap();

        println!("Email: {:?}", email);

        self.smtp_transport.send(&email).context(SendFailureSnafu).context(SmtpSnafu)?;
        info!("Email sent successfully to {}", to_email);
        Ok(())
    }

    async fn render_template(&self, template: &str, vars: &serde_json::Value) -> Result<String, MailerError> {
        self.handlebars.render_template(template, vars).context(TemplateRenderSnafu)
    }
}

#[async_trait]
impl Mailer for SMTPMailer {
    #[instrument(skip(self))]
    async fn send_template_email(
        &self,
        template: EmailTemplate,
        locale: &str,
    ) -> Result<(), MailerError> {
        self.send_email(template, locale).await
    }
}

#[cfg(test)] // load from dotenv,
mod tests {
    use super::*;
    use crate::config::MailerConfig;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_send_verification_email() {
        dotenv().ok();
        let current_dir = std::env::current_dir().unwrap();
        println!("The current directory is {}", current_dir.display());
        let smtp_config = crate::config::env::prefixed("RUSTPROOF_").from_env::<SmtpConfig>().unwrap();
        let mailer_config = crate::config::env::prefixed("RUSTPROOF_").from_env::<MailerConfig>().unwrap();
        let mailer = SMTPMailer::new(smtp_config, mailer_config).unwrap();
        let to_name = "John Doe";
        let to_email = "geoffreygarrett1311@gmail.com";
        let verification_link = "https://example.com";
        mailer.send_template_email(EmailTemplate::new_confirmation(
            Some(to_name.to_string()),
            "https://example.com".to_string(),
            to_email.to_string(),
            verification_link.to_string(),
        ), "en").await.unwrap();
    }

    #[tokio::test]
    async fn test_send_invite_email() {
        dotenv().ok();
        let current_dir = std::env::current_dir().unwrap();
        println!("The current directory is {}", current_dir.display());
        let smtp_config = crate::config::env::prefixed("RUSTPROOF_").from_env::<SmtpConfig>().unwrap();
        let mailer_config = crate::config::env::prefixed("RUSTPROOF_").from_env::<MailerConfig>().unwrap();
        let mailer = SMTPMailer::new(smtp_config, mailer_config).unwrap();
        let to_name = "John Doe";
        let to_email = "geoffreygarrett1311@gmail.com";
        let invite_link = "https://example.com/invite/1234";
        mailer.send_template_email(EmailTemplate::new_invite(
            Some(to_name.to_string()),
            "https://example.com".to_string(),
            to_email.to_string(),
            invite_link.to_string(),
        ), "en").await.unwrap();
    }
}
