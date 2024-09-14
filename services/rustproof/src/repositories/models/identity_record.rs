use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Representation of an identity as stored in the database.
///
/// This struct includes additional metadata such as `id`, `created_at`, and `updated_at`.
#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "adapter-postgres", derive(sqlx::FromRow))]
pub struct IdentityRecord {
    /// The unique identifier for the identity.
    pub id: Uuid,

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

    /// The timestamp when the identity was created.
    pub created_at: DateTime<Utc>,

    /// The timestamp when the identity was last updated.
    pub updated_at: DateTime<Utc>,
}