use async_trait::async_trait;
use crate::errors::Result;
use crate::repositories::{CreateUserParams, UserRecord};
use uuid::Uuid;

/// A repository for managing user data in the database.
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Creates a new user in the database.
    ///
    /// # Arguments
    ///
    /// * `params` - The parameters required to create a new user.
    ///
    /// # Returns
    ///
    /// A `Result` containing the created `UserRecord`.
    async fn create_user(&self, params: &CreateUserParams) -> Result<UserRecord>;

    /// Retrieves a user by their email address.
    ///
    /// # Arguments
    ///
    /// * `email` - The email address of the user.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option<UserRecord>`. If the user is found, it is returned; otherwise, `None`.
    async fn get_user_by_email(&self, email: &str) -> Result<Option<UserRecord>>;

    /// Retrieves a user by their unique identifier.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The unique identifier of the user.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `UserRecord` for the specified user ID.
    async fn get_user_by_id(&self, user_id: &Uuid) -> Result<UserRecord>;

    /// Updates the password hash for a user identified by their unique identifier.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The unique identifier of the user.
    /// * `password_hash` - The new password hash to set for the user.
    ///
    /// # Returns
    ///
    /// A `Result` indicating the success or failure of the operation.
    async fn update_user(&self, user_id: &Uuid, password_hash: String) -> Result<()>;

    /// Checks if a user exists by their email address.
    ///
    /// # Arguments
    ///
    /// * `email` - The email address to check.
    ///
    /// # Returns
    ///
    /// A `Result` containing a boolean indicating whether the user exists.
    async fn user_exists_by_email(&self, email: &str) -> Result<bool>;

    /// Checks if a user exists by their unique identifier.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The unique identifier to check.
    ///
    /// # Returns
    ///
    /// A `Result` containing a boolean indicating whether the user exists.
    async fn user_exists_by_id(&self, user_id: &Uuid) -> Result<bool>;
    /// Deletes a user by their unique identifier.
    /// Deletes a user by their unique identifier.
    async fn delete_user(&self, user_id: &Uuid) -> Result<()>;

    /// Lists users with pagination using offset and limit.
    ///
    /// Returns a tuple of `Vec<UserRecord>` and the total number of users.
    async fn list_users(&self, offset: i32, limit: i32) -> Result<(Vec<UserRecord>, u64)>;

}
