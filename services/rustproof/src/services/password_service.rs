use argon2::{
    self,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use derive_more::Display;
use rand_core::OsRng;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::json;
use std::fmt;
use thiserror::Error;
use zxcvbn::feedback::{Suggestion, Warning};
use zxcvbn::zxcvbn;
use convert_case::{Casing, Case};
use tracing::instrument;

/// Verbal feedback to help choose better passwords
#[derive(Debug, Clone, Default)]
pub struct Feedback {
    /// Explains what's wrong, e.g. "This is a top-10 common password". Not always set.
    warning: Option<Warning>,
    /// A possibly empty list of suggestions to help choose a less guessable password.
    /// E.g. "Add another word or two".
    suggestions: Vec<Suggestion>,
}

impl Serialize for Feedback {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Feedback", 2)?;

        // Serialize the warning if present
        if let Some(warning) = &self.warning {
            state.serialize_field("warning", warning)?;
        }

        // Serialize the suggestions as objects with "type" and "message"
        let suggestions: Vec<serde_json::Value> = self.suggestions.iter().map(|s| {
            json!({
                "type": format!("{:?}", s).to_case(Case::Snake),
                "message": s.to_string()
            })
        }).collect();

        state.serialize_field("suggestions", &suggestions)?;
        state.end()
    }
}

impl fmt::Display for Feedback {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(warning) = &self.warning {
            write!(f, "{}", warning)?;
        }
        if !self.suggestions.is_empty() {
            write!(f, " ({})", self.suggestions.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(", "))?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct WeakPasswordError {
    pub message: String,
    pub feedback: Option<Feedback>,
}

impl Serialize for WeakPasswordError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let message = &self.message;
        let feedback = &self.feedback;
        let mut state = serializer.serialize_struct("WeakPasswordError", 2)?;
        state.serialize_field("message", message)?;
        state.serialize_field("feedback", feedback)?;
        state.end()
    }
}

#[derive(Error, Debug)]
pub enum PasswordError {
    #[error("Hashing failed: {0}")]
    HashingFailed(argon2::password_hash::Error),

    #[error("Verification failed: {0}")]
    VerificationFailed(argon2::password_hash::Error),

    #[error("Weak password: {0}")]
    WeakPassword(WeakPasswordError),
}

impl WeakPasswordError {
    pub fn new(message: String, feedback: Option<Feedback>) -> Self {
        Self { message, feedback }
    }
}
impl fmt::Display for WeakPasswordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(feedback) = &self.feedback {
            write!(f, " (Feedback: {})", feedback)?;
        }
        Ok(())
    }
}

/// A service for hashing and verifying passwords using Argon2.
#[derive(Debug)]
pub struct PasswordService {
    argon2: Argon2<'static>,
}

impl PasswordService {
    /// Creates a new `PasswordService` with the default Argon2 configuration.
    pub fn new() -> Self {
        Self {
            argon2: Argon2::default(),
        }
    }

    /// Hashes a password using Argon2.
    ///
    /// # Arguments
    /// * `password` - The plaintext password to hash.
    ///
    /// # Returns
    /// A `Result` containing the hashed password as a `String` or a `PasswordError`.
    #[instrument(level = "info", skip(self, password))]
    pub fn hash_password(&self, password: &str) -> Result<String, PasswordError> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = self
            .argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(PasswordError::HashingFailed)?;
        Ok(password_hash.to_string())
    }

    /// Verifies a password against a given hash.
    ///
    /// # Arguments
    /// * `password` - The plaintext password to verify.
    /// * `hash` - The hashed password to verify against.
    ///
    /// # Returns
    /// A `Result` indicating whether the verification succeeded (`true`) or failed (`false`).
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, PasswordError> {
        let parsed_hash = PasswordHash::new(hash).map_err(PasswordError::VerificationFailed)?;
        match self.argon2.verify_password(password.as_bytes(), &parsed_hash) {
            Ok(()) => Ok(true),
            Err(e) => Err(PasswordError::VerificationFailed(e)),
        }
    }

    /// Calculates the entropy of a password using the `zxcvbn` library.
    ///
    /// # Arguments
    /// * `password` - The plaintext password to evaluate.
    /// * `user_inputs` - Additional inputs to consider in the entropy calculation.
    ///
    /// # Returns
    /// A `Result` containing the `zxcvbn::Entropy` or a `PasswordError` if the password is too weak.
    pub fn check_password_strength(
        &self,
        password: &str,
        user_inputs: &[&str],
    ) -> Result<zxcvbn::Entropy, PasswordError> {
        let entropy = zxcvbn(password, user_inputs);
        if entropy.score() < zxcvbn::Score::Three {
            return Err(PasswordError::WeakPassword(WeakPasswordError::new(
                "Password is too weak.".to_string(),
                entropy.feedback().cloned().map(|f| Feedback {
                    warning: f.warning(),
                    suggestions: f.suggestions().to_vec(),
                }),
                // entropy.feedback().map(|f| {
                //     f.suggestions()
                //         .iter()
                //         .map(|s| s.to_string())
                //         .collect::<Vec<String>>()
                //         .join(", ")
                // }),
            )));
        }

        Ok(entropy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zxcvbn::Score;

    #[test]
    fn test_hash_password() {
        let password_service = PasswordService::new();
        let password = "my_secret_password";
        let hashed_password = password_service.hash_password(password).unwrap();

        assert!(!hashed_password.is_empty(), "Hashed password should not be empty");
    }

    #[test]
    fn test_verify_password_success() {
        let password_service = PasswordService::new();
        let password = "my_secret_password";
        let hashed_password = password_service.hash_password(password).unwrap();

        let result = password_service.verify_password(password, &hashed_password).unwrap();
        assert!(result, "Password verification should succeed");
    }

    #[test]
    fn test_verify_password_failure() {
        let password_service = PasswordService::new();
        let password = "my_secret_password";
        let wrong_password = "wrong_password";
        let hashed_password = password_service.hash_password(password).unwrap();

        let result = password_service.verify_password(wrong_password, &hashed_password);
        assert!(result.is_err(), "Password verification should fail for incorrect password");
    }

    // #[test]
    // fn test_check_password_strength_basic() {
    //     let password_service = PasswordService::new();
    //     let password = "simplepassword";
    //     let entropy = password_service.check_password_strength(password, &[]).unwrap();
    //
    //     assert!(entropy.score() > Score::Zero, "Entropy score should be greater than 0");
    // }

    #[test]
    fn test_check_password_strength_with_user_inputs() {
        let password_service = PasswordService::new();
        let password = "simplepassword";
        let user_inputs = ["simple", "password"];
        let result = password_service.check_password_strength(password, &user_inputs);

        assert!(result.is_err(), "Entropy score should be low with user inputs");
    }

    #[test]
    fn test_check_password_strength_strong_password() {
        let password_service = PasswordService::new();
        let password = "C0mpl3x!ty#2024";
        let entropy = password_service.check_password_strength(password, &[]).unwrap();

        assert!(entropy.score() >= Score::Three, "Entropy score should be high for strong password");
    }

    #[test]
    fn test_check_password_strength_edge_case_empty_password() {
        let password_service = PasswordService::new();
        let password = "";
        let result = password_service.check_password_strength(password, &[]);

        assert!(result.is_err(), "Entropy score should be zero for empty password");
        if let Err(PasswordError::WeakPassword(err)) = result {
            assert_eq!(err.message, "Password is too weak.", "Expected weak password error");
        } else {
            panic!("Expected WeakPasswordError");
        }
    }
}
