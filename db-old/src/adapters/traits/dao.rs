use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait Dao<T> {
    type Error: Error;
    type QueryParams;

    async fn insert(&self, item: &T, params: Option<Self::QueryParams>) -> Result<Option<T>, Self::Error>;
    async fn upsert(&self, item: &T, conflict_resolution: &str, params: Option<Self::QueryParams>) -> Result<Option<T>, Self::Error>;
    async fn delete(&self, params: Self::QueryParams) -> Result<Option<T>, Self::Error>;
    async fn update(&self, item: &T, params: Self::QueryParams) -> Result<Option<T>, Self::Error>;
    async fn select<U>(&self, query: U, params: Option<Self::QueryParams>) -> Result<Vec<T>, Self::Error> where U: AsRef<str> + Send + Sync;
}
