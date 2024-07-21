use git2::{Repository, Error as GitError, Oid};
use async_trait::async_trait;
use std::collections::HashMap;
use std::error::Error;
use crate::models::Contributor;
use crate::repository::{calculate_code_churn, create_revwalk};
use super::Analyzer;

pub struct TopContributorsAnalyzer;

#[async_trait]
impl Analyzer for TopContributorsAnalyzer {
    type Output = Vec<Contributor>;

    async fn analyze(&self, repo_path: &str, old_commit: &str, new_commit: &str) -> Result<Self::Output, Box<dyn Error>> {
        let repo = Repository::open(repo_path)?;
        let revwalk = create_revwalk(&repo, old_commit, new_commit)?;
        let churn_data = calculate_code_churn(&repo, revwalk)?;

        let mut contributors = HashMap::new();

        for churn in churn_data {
            let commit_id = churn.commit().to_string();
            let commit = repo.find_commit(Oid::from_str(&commit_id)?)?;
            let author_name = commit.author().name().unwrap_or("Unknown").to_string();

            let entry = contributors.entry(author_name.clone()).or_insert_with(|| Contributor::new(author_name.clone(), 0));
            entry.add_commits(churn.additions() as i32 + churn.deletions() as i32);
        }

        let top_contributors: Vec<_> = contributors.into_iter()
            .map(|(_, contributor)| contributor)
            .collect();

        Ok(top_contributors)
    }
}
