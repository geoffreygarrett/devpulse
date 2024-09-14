mod mailer;

use mailer::*;

use async_trait::async_trait;
use lettre::transport::smtp::Error as SmtpError;

#[async_trait]
pub trait Mailer: Send + Sync {
    async fn send_verification_email(
        &self,
        to_name: &str,
        to_email: &str,
        verification_link: &str,
    ) -> Result<(), SmtpError>;

    async fn send_invite_email(
        &self,
        to_name: &str,
        to_email: &str,
        invite_link: &str,
    ) -> Result<(), SmtpError>;

    async fn send_recovery_email(
        &self,
        to_name: &str,
        to_email: &str,
        recovery_link: &str,
    ) -> Result<(), SmtpError>;

    async fn send_email_change_email(
        &self,
        to_name: &str,
        to_email: &str,
        change_link: &str,
    ) -> Result<(), SmtpError>;

    async fn send_generic_email(
        &self,
        to_name: &str,
        to_email: &str,
        subject: &str,
        body: &str,
    ) -> Result<(), SmtpError>;
}

pub enum EmailTemplate {
    Verification { to_name: String, verification_link: String },
    Invite { to_name: String, invite_link: String },
    Recovery { to_name: String, recovery_link: String },
    EmailChange { to_name: String, change_link: String },
    Generic { to_name: String, subject: String, body: String },
}

impl EmailTemplate {
    pub fn subject(&self) -> &str {
        match self {
            EmailTemplate::Verification { .. } => "Please Verify Your Email Address",
            EmailTemplate::Invite { .. } => "You Are Invited!",
            EmailTemplate::Recovery { .. } => "Password Recovery Instructions",
            EmailTemplate::EmailChange { .. } => "Confirm Your Email Address Change",
            EmailTemplate::Generic { subject, .. } => subject,
        }
    }

    pub fn generate_html(&self) -> String {
        match self {
            EmailTemplate::Verification { to_name, verification_link } => {
                format!(
                    "Hello {to_name},\n\nPlease verify your email by clicking the link below:\n\n{verification_link}",
                    to_name = to_name,
                    verification_link = verification_link
                )
            }
            EmailTemplate::Invite { to_name, invite_link } => {
                format!(
                    "Hello {to_name},\n\nYou have been invited! Click the link to join:\n\n{invite_link}",
                    to_name = to_name,
                    invite_link = invite_link
                )
            }
            EmailTemplate::Recovery { to_name, recovery_link } => {
                format!(
                    "Hello {to_name},\n\nReset your password by clicking the link below:\n\n{recovery_link}",
                    to_name = to_name,
                    recovery_link = recovery_link
                )
            }
            EmailTemplate::EmailChange { to_name, change_link } => {
                format!(
                    "Hello {to_name},\n\nConfirm your email address change by clicking the link below:\n\n{change_link}",
                    to_name = to_name,
                    change_link = change_link
                )
            }
            EmailTemplate::Generic { to_name, body, .. } => {
                format!("Hello {to_name},\n\n{body}", to_name = to_name, body = body)
            }
        }
    }

    pub fn generate_plain_text(&self) -> String {
        self.generate_html() // For simplicity, we can use the same content for plain text
    }
}
