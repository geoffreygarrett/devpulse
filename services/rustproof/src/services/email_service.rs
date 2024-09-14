// use std::sync::Arc;
// // use crate::email::mailer::Mailer;
// use lettre::transport::smtp::Error as SmtpError;
// use crate::services::mailer_service::Mailer;
//
// pub struct EmailService {
//     mailer: Arc<dyn Mailer>,
// }
//
// impl EmailService {
//     pub fn new(mailer: Arc<dyn Mailer>) -> Self {
//         Self { mailer }
//     }
//
//     pub async fn send_verification_email(
//         &self,
//         to_name: &str,
//         to_email: &str,
//         verification_link: &str,
//     ) -> Result<(), SmtpError> {
//         self.mailer
//             .send_verification_email(to_name, to_email, verification_link)
//             .await
//     }
//
//     pub async fn send_invite_email(
//         &self,
//         to_name: &str,
//         to_email: &str,
//         invite_link: &str,
//     ) -> Result<(), SmtpError> {
//         self.mailer
//             .send_invite_email(to_name, to_email, invite_link)
//             .await
//     }
//
//     pub async fn send_recovery_email(
//         &self,
//         to_name: &str,
//         to_email: &str,
//         recovery_link: &str,
//     ) -> Result<(), SmtpError> {
//         self.mailer
//             .send_recovery_email(to_name, to_email, recovery_link)
//             .await
//     }
//
//     pub async fn send_email_change_email(
//         &self,
//         to_name: &str,
//         to_email: &str,
//         change_link: &str,
//     ) -> Result<(), SmtpError> {
//         self.mailer
//             .send_email_change_email(to_name, to_email, change_link)
//             .await
//     }
//
//     pub async fn send_generic_email(
//         &self,
//         to_name: &str,
//         to_email: &str,
//         subject: &str,
//         body: &str,
//     ) -> Result<(), SmtpError> {
//         self.mailer
//             .send_generic_email(to_name, to_email, subject, body)
//             .await
//     }
// }
