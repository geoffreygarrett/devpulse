use std::fmt::Debug;
use std::fmt::Display;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait::async_trait]
pub trait DataAccess<T, I, U> {
    type Error: Debug + Display;

    async fn create(&self, item: &I) -> Result<T, Self::Error>;
    async fn read(&self, id: u64) -> Result<Option<T>, Self::Error>;
    async fn update(&self, item: &U) -> Result<T, Self::Error>;
    async fn delete(&self, id: u64) -> Result<(), Self::Error>;
    async fn list(&self) -> Result<Vec<T>, Self::Error>;
}

#[async_trait]
pub trait DbPool {
    type Pool;

    // Ensure this matches the trait signature
    async fn new_pool(database_url: &str) -> Result<Self::Pool>;
}
