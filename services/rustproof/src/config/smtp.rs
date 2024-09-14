use secrecy::Secret;
use serde::{Deserialize, Deserializer, Serialize};
use validator::{Validate, ValidationError, ValidationErrors, ValidationErrorsKind};

/// Configuration for the SMTP (Simple Mail Transfer Protocol) service.
///
/// This structure contains all the necessary settings for connecting to an SMTP server
/// and sending emails from your application.
#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct SmtpConfig {
    /// The "From" email address for all emails sent. (**SMTP_ADMIN_EMAIL**)
    ///
    /// This field is **required** and must be a valid email address.
    #[validate(email(message = "Invalid email address"))]
    pub admin_email: String,

    /// The mail server hostname to send emails through. (**SMTP_HOST**)
    ///
    /// This field is **required** and must be a valid hostname or IP address.
    #[validate(length(min = 1, message = "SMTP host cannot be empty"))]
    pub host: String,

    /// The port number to connect to the mail server on. (**SMTP_PORT**)
    ///
    /// This field is **required** and must be a valid port number (1-65535).
    #[validate(range(min = 1, max = 65535))]
    pub port: u16,

    /// The username to use if the mail server requires authentication. (**SMTP_USER**)
    ///
    /// This field is **optional** and is used in conjunction with `smtp_pass` for authentication.
    #[validate(length(min = 1, message = "SMTP username cannot be empty"))]
    pub user: Option<String>,

    /// The password to use if the mail server requires authentication. (**SMTP_PASS**)
    ///
    /// This field is **optional** and is used in conjunction with `smtp_user` for authentication.
    #[serde(serialize_with = "crate::utils::serde::serialize_option_secret_redacted")]
    pub pass: Option<Secret<String>>,

    /// Controls the minimum amount of time that must pass before sending another signup confirmation or password reset email. (**SMTP_MAX_FREQUENCY**)
    ///
    /// The value is the number of seconds. Defaults to 900 seconds (15 minutes).
    #[validate(range(min = 1, message = "SMTP max frequency must be at least 1 second"))]
    #[serde(
        default = "SmtpConfig::default_smtp_max_frequency",
    )]
    pub max_frequency: u64,

    /// Sets the name of the sender. (**SMTP_SENDER_NAME**)
    ///
    /// If not provided, defaults to the value of `smtp_admin_email`.
    #[validate(length(min = 1, message = "SMTP sender name cannot be empty"))]
    pub sender_name: Option<String>,
}

impl SmtpConfig {
    /// Default value for SMTP max frequency (15 minutes)
    pub const fn default_smtp_max_frequency() -> u64 {
        900
    }

    /// Validates the configuration
    pub fn validate_config(&self) -> Result<(), ValidationErrors> {
        self.validate()?;

        // Additional custom validations
        if self.user.is_some() != self.pass.is_some() {
            let error = ValidationError {
                code: "smtp_user".into(),
                message: Some("SMTP user and pass must be both set or both unset".into()),
                params: Default::default(),
            };
            return Err(ValidationErrors(
                [("smtp_user", ValidationErrorsKind::Field(vec![error]))]
                    .iter()
                    .cloned()
                    .collect(),
            ));
        }
        Ok(())
    }
}
