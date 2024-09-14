use crate::errors::Result;
use crate::repositories::{CreateFidoCredentialsParams, FidoCredentialsRecord};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait FidoCredentialsRepository: Send + Sync {
    /// Creates a new FIDO credential.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters required to create a FIDO credential.
    ///
    /// # Returns
    ///
    /// * `Result<FidoCredentialsRecord>` - The created FIDO credential.
    async fn create_credential(&self, params: CreateFidoCredentialsParams) -> Result<FidoCredentialsRecord>;

    /// Retrieves a FIDO credential by its credential ID.
    ///
    /// # Arguments
    ///
    /// * `credential_id` - The unique ID of the credential.
    ///
    /// # Returns
    ///
    /// * `Result<Option<FidoCredentialsRecord>>` - The FIDO credential if found, otherwise `None`.
    async fn get_credential_by_id(&self, credential_id: &[u8]) -> Result<Option<FidoCredentialsRecord>>;

    /// Retrieves all FIDO credentials associated with a user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The ID of the user.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<FidoCredentialsRecord>>` - A list of the user's FIDO credentials.
    async fn get_credentials_by_user_id(&self, user_id: Uuid) -> Result<Vec<FidoCredentialsRecord>>;

    /// Updates the sign count of a FIDO credential.
    ///
    /// # Arguments
    ///
    /// * `credential_id` - The unique ID of the credential.
    /// * `sign_count` - The new sign count.
    async fn update_sign_count(&self, credential_id: &[u8], sign_count: i64) -> Result<()>;

    /// Deletes a FIDO credential by its credential ID.
    ///
    /// # Arguments
    ///
    /// * `credential_id` - The unique ID of the credential to delete.
    async fn delete_credential(&self, credential_id: &[u8]) -> Result<()>;
}
