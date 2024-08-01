use std::error::Error;

use async_trait::async_trait;

/// Defines the Data Access Object (DAO) interface for interacting with the database.
#[async_trait]
pub trait Dao<Entity, InsertModel, UpdateModel> {
    /// Defines the error type that methods of this trait will return.
    type Error: Error;

    /// Defines the type for any additional query parameters.
    type QueryParams;

    /// Inserts a new entity into the database.
    ///
    /// # Parameters
    /// - `item`: The data model for insertion.
    /// - `params`: Optional parameters for the insertion operation.
    ///
    /// # Returns
    /// A result containing either the newly created entity or an error.
    async fn insert(
        &self, item: &InsertModel, params: Option<Self::QueryParams>,
    ) -> Result<Option<Entity>, Self::Error>;

    /// Upserts an entity into the database, with conflict resolution.
    ///
    /// # Parameters
    /// - `item`: The data model for insertion.
    /// - `conflict_resolution`: SQL conflict resolution clause.
    /// - `params`: Optional parameters for the upsert operation.
    ///
    /// # Returns
    /// A result containing either the newly created or updated entity or an error.
    async fn upsert(
        &self, item: &InsertModel, conflict_resolution: &str, params: Option<Self::QueryParams>,
    ) -> Result<Option<Entity>, Self::Error>;

    /// Deletes entities based on specified parameters.
    ///
    /// # Parameters
    /// - `params`: Parameters that specify which entities to delete.
    ///
    /// # Returns
    /// A result containing an optional entity that was deleted or an error.
    async fn delete(&self, params: Self::QueryParams) -> Result<Option<Entity>, Self::Error>;

    /// Updates an existing entity based on specified parameters.
    ///
    /// # Parameters
    /// - `item`: The update model containing the new data.
    /// - `params`: Parameters for the update operation.
    ///
    /// # Returns
    /// A result containing an optional updated entity or an error.
    async fn update(
        &self, item: &UpdateModel, params: Self::QueryParams,
    ) -> Result<Option<Entity>, Self::Error>;

    /// Selects entities based on a query string.
    ///
    /// # Parameters
    /// - `query`: The query string to execute.
    /// - `params`: Optional parameters for the query.
    ///
    /// # Returns
    /// A result containing a vector of entities or an error.
    async fn select<U>(
        &self, query: U, params: Option<Self::QueryParams>,
    ) -> Result<Vec<Entity>, Self::Error>
    where
        U: AsRef<str> + Send + Sync;
}
