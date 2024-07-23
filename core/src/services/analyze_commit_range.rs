use std::error::Error;

use crate::analyzers::{Analyzer, CodeChurnAnalyzer, TopContributorsAnalyzer};
use crate::models::{CommitRangeAnalysis, CommitRangeDetails, Contributor, Protocol, Repository};
use crate::utils::RepositoryManager;

/// Analyzes a specified range of commits within a repository using various analyzers.
///
/// # Arguments
///
/// * `repository_url` - The URL of the repository to analyze.
/// * `start_commit` - The starting commit hash for the analysis.
/// * `end_commit` - The ending commit hash for the analysis.
///
/// # Returns
///
/// A `Result` containing `CommitRangeAnalysis` if successful, or an `Error` if an error occurred.
///
/// # Errors
///
/// Returns an error if there is an issue with repository access, cloning, or analysis.
pub async fn analyze_commit_range_service(
    repository: &Repository, start_commit: &str, end_commit: &str,
) -> Result<CommitRangeAnalysis, Box<dyn Error>> {
    let repo_manager = RepositoryManager::new(&repository.url(Protocol::Http).unwrap())?;
    let repo = repo_manager.open_or_clone().await?;
    let local_path = repo_manager.get_local_path();

    let code_churn_analyzer = CodeChurnAnalyzer;
    let top_contributors_analyzer = TopContributorsAnalyzer;

    let code_churn_results = code_churn_analyzer
        .analyze(&local_path, &start_commit, &end_commit)
        .await?;
    let mut top_contributors_results = top_contributors_analyzer
        .analyze(&local_path, &start_commit, &end_commit)
        .await?;

    top_contributors_results.sort_by(|a, b| b.commits.cmp(&a.commits));

    // Combine the results from different analyzers
    Ok(CommitRangeAnalysis {
        repository: repository.clone(),
        commit_range: CommitRangeDetails {
            start_commit: start_commit.to_string(),
            end_commit: end_commit.to_string(),
            total_commits: code_churn_results.len() as i32,
            total_additions: code_churn_results
                .iter()
                .map(|c| c.additions() as i32)
                .sum(),
            total_deletions: code_churn_results
                .iter()
                .map(|c| c.deletions() as i32)
                .sum(),
            top_contributors: top_contributors_results
                .iter()
                .map(|c| Contributor {
                    username: c.username.clone(),
                    commits: c.commits as i32,
                })
                .collect(),
        },
    })
}
