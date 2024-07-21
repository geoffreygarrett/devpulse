use crate::{repository::create_revwalk, repository::calculate_code_churn, models::CodeChurn};
use git2::Repository;
use async_trait::async_trait;
use std::error::Error;
use crate::utils::RepositoryManager;
use super::Analyzer;

pub struct CodeChurnAnalyzer;


#[async_trait]
impl Analyzer for CodeChurnAnalyzer {
    type Output = Vec<CodeChurn>;

    async fn analyze(&self, repo_path: &str, old_commit: &str, new_commit: &str) -> Result<Self::Output, Box<dyn Error>> {
        let repo = Repository::open(&repo_path)?;
        let revwalk = create_revwalk(&repo, &old_commit, &new_commit)?;
        let churn_data = calculate_code_churn(&repo, revwalk)?;

        Ok(churn_data)
    }
}