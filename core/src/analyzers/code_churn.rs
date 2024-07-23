use std::error::Error;

use async_trait::async_trait;
use git2::Repository;

use crate::{models::CodeChurn, repository::calculate_code_churn, repository::create_revwalk};

use super::Analyzer;

pub struct CodeChurnAnalyzer;

#[async_trait]
impl Analyzer for CodeChurnAnalyzer {
    type Output = Vec<CodeChurn>;

    async fn analyze(
        &self, repo_path: &str, old_commit: &str, new_commit: &str,
    ) -> Result<Self::Output, Box<dyn Error>> {
        let repo = Repository::open(&repo_path)?;
        let revwalk = create_revwalk(&repo, &old_commit, &new_commit)?;
        let churn_data = calculate_code_churn(&repo, revwalk)?;

        Ok(churn_data)
    }
}
