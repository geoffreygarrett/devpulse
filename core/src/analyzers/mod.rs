mod code_churn;
mod top_contributor;

pub use code_churn::CodeChurnAnalyzer;
pub use top_contributor::TopContributorsAnalyzer;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait Analyzer {
    type Output;
    async fn analyze(&self, repo_path: &str, old_commit: &str, new_commit: &str) -> Result<Self::Output, Box<dyn Error>>;
}
