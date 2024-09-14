use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "service_type", content = "config")]
pub enum EmailServiceConfig {
    Smtp(SmtpConfig),
    Resend(ResendConfig),
    // SendGrid(SendGridConfig),
}

// Implement `From` trait for `SmtpConfig` to `EmailServiceConfig`
impl From<SmtpConfig> for EmailServiceConfig {
    fn from(config: SmtpConfig) -> Self {
        EmailServiceConfig::Smtp(config)
    }
}

// Implement `From` trait for `ResendConfig` to `EmailServiceConfig`
impl From<ResendConfig> for EmailServiceConfig {
    fn from(config: ResendConfig) -> Self {
        EmailServiceConfig::Resend(config)
    }
}

// // Implement `From` trait for `SendGridConfig` to `EmailServiceConfig`
// impl From<SendGridConfig> for EmailServiceConfig {
//     fn from(config: SendGridConfig) -> Self {
//         EmailServiceConfig::SendGrid(config)
//     }
// }

#[derive(Debug, Deserialize)]
pub struct SmtpConfig {
    pub server: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub from_name: String,
    pub from_email: String,
}

#[derive(Debug, Deserialize)]
pub struct SendGridConfig {
    pub api_key: String,
    pub from_name: String,
    pub from_email: String,
}

#[derive(Debug, Deserialize)]
pub struct ResendConfig {
    pub api_key: String,
    pub from_name: String,
    pub from_email: String,
}

impl From<ResendConfig> for SmtpConfig {
    fn from(resend_config: ResendConfig) -> Self {
        SmtpConfig {
            server: "smtp.resend.com".to_string(),
            port: 465,
            username: "resend".to_string(),
            password: resend_config.api_key,
            from_name: resend_config.from_name,
            from_email: resend_config.from_email,
        }
    }
}

pub struct EmailService {
    config: SmtpConfig,
    mailer: SmtpTransport,
}

impl EmailService {
    pub fn new(config: EmailServiceConfig) -> Self {
        match config {
            EmailServiceConfig::Smtp(smtp_config) => {
                let creds = Credentials::new(smtp_config.username.clone(), smtp_config.password.clone());
                let mailer = SmtpTransport::relay(&smtp_config.server)
                    .unwrap()
                    .port(smtp_config.port)
                    .credentials(creds)
                    .build();
                EmailService {
                    config: smtp_config,
                    mailer,
                }
            }
            EmailServiceConfig::Resend(resend_config) => {
                let smtp_config: SmtpConfig = resend_config.into();
                let creds = Credentials::new(smtp_config.username.clone(), smtp_config.password.clone());
                let mailer = SmtpTransport::relay(&smtp_config.server)
                    .unwrap()
                    .port(smtp_config.port)
                    .credentials(creds)
                    .build();
                EmailService {
                    config: smtp_config,
                    mailer,
                }
            }
        }
    }

    pub fn send_verification_email(
        &self,
        to_name: &str,
        to_email: &str,
        verification_link: &str,
    ) -> Result<(), lettre::transport::smtp::Error> {
        let subject = "Please Verify Your Email Address";
        let html_body = self.build_verification_html(to_name, verification_link);
        let plain_body = self.build_verification_plain_text(to_name, verification_link);

        let email = Message::builder()
            .from(format!("{} <{}>", self.config.from_name, self.config.from_email).parse().unwrap())
            .to(format!("{} <{}>", to_name, to_email).parse().unwrap())
            .subject(subject)
            .multipart(lettre::message::MultiPart::alternative_plain_html(
                plain_body, html_body,
            ))
            .unwrap();

        self.mailer.send(&email).map(|_| ())
    }

    pub fn send_email(
        &self,
        to_name: &str,
        to_email: &str,
        subject: &str,
        body: &str,
    ) -> Result<(), lettre::transport::smtp::Error> {
        let email = Message::builder()
            .from(format!("{} <{}>", self.config.from_name, self.config.from_email).parse().unwrap())
            .to(format!("{} <{}>", to_name, to_email).parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body.to_string())
            .unwrap();

        self.mailer.send(&email).map(|_| ())
    }

    fn build_verification_html(&self, to_name: &str, verification_link: &str) -> String {
        // language=HTML
        format!(
            r#"
        <html lang="en">
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Email Verification</title>
            </head>
            <body style="font-family: Arial, sans-serif; background-color: #f9f9f9; color: #333; margin: 0; padding: 20px;">
                <div style="max-width: 600px; margin: 0 auto; padding: 20px; background-color: #ffffff; border: 1px solid #ababab; border-radius: 5px;">
                    <h2 style="color: #0056b3; text-align: center;">Verify Your Email</h2>
                    <p>Hello {to_name},</p>
                    <p>Thank you for registering with us. Please verify your email address by clicking the link below:</p>
                    <p style="text-align: center; margin: 20px 0;">
                        <a href="{verification_link}" style="display: inline-block; padding: 12px 20px; background-color: #0056b3; color: #ffffff; text-decoration: none; border-radius: 4px;">Verify Email</a>
                    </p>
                    <p>If you are unable to click the button, please copy and paste the following link into your browser:</p>
                    <p style="color: #0056b3;">
                        <a href="{verification_link}" style="color: #0056b3; text-decoration: underline;">{verification_link}</a>
                    </p>
                    <p>If you did not request this, please ignore this email.</p>
                    <p style="text-align: center; color: #777; font-size: 12px; margin-top: 30px;">Best Regards,<br>{company_name}</p>
                </div>
            </body>
        </html>
        "#,
            to_name = to_name,
            verification_link = verification_link,
            company_name = self.config.from_name,
        )
    }

    fn build_verification_plain_text(&self, to_name: &str, verification_link: &str) -> String {
        format!(
            "Hello {to_name},\n\n\
            Thank you for registering with us. Please verify your email address by clicking the link below:\n\n\
            {verification_link}\n\n\
            If you are unable to click the link, please copy and paste it into your browser.\n\n\
            If you did not request this, please ignore this email.\n\n\
            Best Regards,\n\
            {company_name}",
            to_name = to_name,
            verification_link = verification_link,
            company_name = self.config.from_name,
        )
    }
}
