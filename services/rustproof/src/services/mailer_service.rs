// use async_trait::async_trait;
// use lettre::transport::smtp::Error as SmtpError;
//
// #[async_trait]
// pub trait Mailer: Send + Sync {
//     async fn send_verification_email(
//         &self,
//         to_name: &str,
//         to_email: &str,
//         verification_link: &str,
//     ) -> Result<(), SmtpError>;
//
//     async fn send_invite_email(
//         &self,
//         to_name: &str,
//         to_email: &str,
//         invite_link: &str,
//     ) -> Result<(), SmtpError>;
//
//     async fn send_recovery_email(
//         &self,
//         to_name: &str,
//         to_email: &str,
//         recovery_link: &str,
//     ) -> Result<(), SmtpError>;
//
//     async fn send_email_change_email(
//         &self,
//         to_name: &str,
//         to_email: &str,
//         change_link: &str,
//     ) -> Result<(), SmtpError>;
//
//     async fn send_generic_email(
//         &self,
//         to_name: &str,
//         to_email: &str,
//         subject: &str,
//         body: &str,
//     ) -> Result<(), SmtpError>;
// }
//
//
// use crate::config::SmtpConfig;
// use crate::email::{EmailTemplate, Mailer};
// use async_trait::async_trait;
// use lettre::message::header::ContentType;
// use lettre::{Message, SmtpTransport, Transport};
//
// pub struct SMTPMailer {
//     config: SmtpConfig,
//     mailer: SmtpTransport,
// }
//
// impl SMTPMailer {
//     pub fn new(config: SmtpConfig) -> Self {
//         let creds = Credentials::new(config.username.clone(), config.password.clone());
//         let mailer = SmtpTransport::relay(&config.server)
//             .unwrap()
//             .port(config.port)
//             .credentials(creds)
//             .build();
//         Self { config, mailer }
//     }
//
//     async fn send_email_template(&self, template: EmailTemplate, to_email: &str) -> Result<(), lettre::transport::smtp::Error> {
//         let subject = template.subject();
//         let html_body = template.generate_html();
//         let plain_body = template.generate_plain_text();
//
//         let email = Message::builder()
//             .from(format!("{} <{}>", self.config.from_name, self.config.from_email).parse().unwrap())
//             .to(to_email.parse().unwrap())
//             .subject(subject)
//             .multipart(lettre::message::MultiPart::alternative_plain_html(plain_body, html_body))
//             .unwrap();
//
//         self.mailer.send(&email).map(|_| ())
//     }
// }
//
// #[async_trait]
// impl Mailer for SMTPMailer {
//     async fn send_verification_email(&self, to_name: &str, to_email: &str, verification_link: &str) -> Result<(), SmtpError> {
//         let template = EmailTemplate::Verification {
//             to_name: to_name.to_string(),
//             verification_link: verification_link.to_string(),
//         };
//         self.send_email_template(template, to_email).await
//     }
//
//     async fn send_invite_email(&self, to_name: &str, to_email: &str, invite_link: &str) -> Result<(), SmtpError> {
//         let template = EmailTemplate::Invite {
//             to_name: to_name.to_string(),
//             invite_link: invite_link.to_string(),
//         };
//         self.send_email_template(template, to_email).await
//     }
//
//     async fn send_recovery_email(&self, to_name: &str, to_email: &str, recovery_link: &str) -> Result<(), SmtpError> {
//         let template = EmailTemplate::Recovery {
//             to_name: to_name.to_string(),
//             recovery_link: recovery_link.to_string(),
//         };
//         self.send_email_template(template, to_email).await
//     }
//
//     async fn send_email_change_email(&self, to_name: &str, to_email: &str, change_link: &str) -> Result<(), SmtpError> {
//         let template = EmailTemplate::EmailChange {
//             to_name: to_name.to_string(),
//             change_link: change_link.to_string(),
//         };
//         self.send_email_template(template, to_email).await
//     }
//
//     async fn send_generic_email(&self, to_name: &str, to_email: &str, subject: &str, body: &str) -> Result<(), SmtpError> {
//         let email = Message::builder()
//             .from(format!("{} <{}>", self.config.from_name, self.config.from_email).parse().unwrap())
//             .to(format!("{} <{}>", to_name, to_email).parse().unwrap())
//             .subject(subject)
//             .header(ContentType::TEXT_PLAIN)
//             .body(body.to_string())
//             .unwrap();
//
//         self.mailer.send(&email).map(|_| ())
//     }
// }
