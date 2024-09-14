use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Parameters for creating a new identity.
///
/// This struct contains the necessary fields for creating an identity in the database.
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateIdentityParams {
    /// The ID of the user to whom the identity belongs.
    pub user_id: Uuid,

    /// The type of identity (e.g., "email", "phone", "oauth").
    pub identity_type: String,

    /// The value of the identity (e.g., the email address, phone number, or OAuth identifier).
    pub value: String,

    /// Optional metadata associated with the identity (e.g., OAuth claims).
    pub data: Option<serde_json::Value>,

    /// Indicates whether the identity has been verified.
    pub verified: bool,

    /// The provider of the identity, if applicable (e.g., "google", "facebook").
    pub provider: Option<String>,
}