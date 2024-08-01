use std::error::Error;

use async_trait::async_trait;

/// Defines the Database Pool trait to be implemented by specific database types.
#[async_trait]
pub trait DatabasePool {
    type Pool;
    type QueryType;

    /// Creates a new database pool.
    ///
    /// # Parameters
    /// - `database_url`: The URL of the database to connect to.
    ///
    /// # Returns
    /// A result containing either the created pool or an error.
    async fn new_pool(database_url: &str) -> Result<Self::Pool, Box<dyn Error>>;
}
