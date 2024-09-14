use crate::{ServiceError, SmtpSnafu};
use lettre::transport::smtp::AsyncSmtpTransportBuilder;
use lettre::{
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, Tokio1Executor,
};
use rustproof::config::SmtpConfig;
use secrecy::ExposeSecret;
use snafu::ResultExt;

pub trait SmtpTransportBuilderExt {
    fn optional_credentials(self, creds: Option<Credentials>) -> Self;
}

impl SmtpTransportBuilderExt for AsyncSmtpTransportBuilder {
    fn optional_credentials(mut self, creds: Option<Credentials>) -> Self {
        if let Some(creds) = creds {
            self = self.credentials(creds);
        }
        self
    }
}

#[tracing::instrument(skip(smtp_config))]
pub fn init_smtp_client(smtp_config: &SmtpConfig) -> Result<AsyncSmtpTransport<Tokio1Executor>, ServiceError> {
    let creds = smtp_config
        .user
        .clone()
        .zip(smtp_config.pass.clone())
        .map(|(user, pass)| Credentials::new(user, pass.expose_secret().to_string()));

    Ok(AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_config.host)
        .context(SmtpSnafu)?
        .port(smtp_config.port)
        .optional_credentials(creds)
        .build())
}