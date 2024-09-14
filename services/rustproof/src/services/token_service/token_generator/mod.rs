pub mod jwt;

use async_trait::async_trait;
use crate::errors::Result;
use serde_json::Value;

#[async_trait]
pub trait TokenGenerator: Send + Sync {
    async fn generate_token(&self, claims: &Value) -> Result<String>;
    async fn validate_token(&self, token: &str) -> Result<bool>;
}
