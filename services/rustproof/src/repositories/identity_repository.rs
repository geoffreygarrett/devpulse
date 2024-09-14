use crate::errors::Result;
use crate::repositories::models::{CreateIdentityParams, IdentityRecord};
use async_trait::async_trait;
use uuid::Uuid;

/// Trait for interacting with identities in the database.
///
/// This trait defines methods for creating, retrieving, updating, and managing identities.
#[async_trait]
pub trait IdentityRepository {
    /// Creates a new identity based on the provided parameters.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters for creating a new identity.
    ///
    /// # Returns
    ///
    /// * `Result<IdentityRecord>` - The newly created identity.
    async fn create_identity(&self, params: &CreateIdentityParams) -> Result<IdentityRecord>;

    /// Retrieves an identity by its unique ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the identity.
    ///
    /// # Returns
    ///
    /// * `Result<Option<IdentityRecord>>` - The identity if found, otherwise `None`.
    async fn get_identity_by_id(&self, id: &Uuid) -> Result<Option<IdentityRecord>>;

    /// Retrieves an identity by its type and value.
    ///
    /// # Arguments
    ///
    /// * `identity_type` - The type of the identity (e.g., "email").
    /// * `value` - The value of the identity (e.g., the email address).
    ///
    /// # Returns
    ///
    /// * `Result<Option<IdentityRecord>>` - The identity if found, otherwise `None`.
    async fn get_identity_by_type_and_value(&self, identity_type: &str, value: &str) -> Result<Option<IdentityRecord>>;

    /// Updates the verification status of an identity.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the identity.
    /// * `verified` - The new verification status.
    async fn update_identity_verification(&self, id: &Uuid, verified: bool) -> Result<()>;

    /// Checks if an identity exists by its type and value.
    ///
    /// # Arguments
    ///
    /// * `identity_type` - The type of the identity (e.g., "email").
    /// * `value` - The value of the identity (e.g., the email address).
    ///
    /// # Returns
    ///
    /// * `Result<bool>` - `true` if the identity exists, otherwise `false`.
    async fn identity_exists(&self, identity_type: &str, value: &str) -> Result<bool>;

    /// Links an identity to a user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user to whom the identity should be linked.
    /// * `params` - The parameters for creating the identity.
    async fn link_identity(&self, user_id: &Uuid, params: &CreateIdentityParams) -> Result<()>;

    /// Unlinks an identity by its unique ID.
    ///
    /// # Arguments
    ///
    /// * `identity_id` - The unique identifier of the identity to unlink.
    async fn unlink_identity(&self, identity_id: &Uuid) -> Result<()>;

    /// Retrieves all identities associated with a specific user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose identities should be retrieved.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<IdentityRecord>>` - A list of identities associated with the user.
    async fn get_user_identities(&self, user_id: &Uuid) -> Result<Vec<IdentityRecord>>;

    /// Removes all unverified identities associated with a specific user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user whose unverified identities should be removed.
    async fn remove_unverified_identities(&self, user_id: &Uuid) -> Result<()>;
}
