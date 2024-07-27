use external_github::{apis::GithubClient, models::Commit as GitHubCommit};

use crate::annotations::prelude::{Annotation, Annotator, GitHubAnnotator};
use crate::models;
use crate::models::GitHubRepository;
use crate::prelude::*;

/// A result type for VCS operations.
type Result<T> = std::result::Result<T, VcsError>;

#[nject::provider]
struct Provider;

/// Trait for services interacting with VCS platforms.
#[async_trait::async_trait]
pub trait VcsService<T>: Annotator + CommitInspection<T> {
    // Additional methods that cross-cut the sub-traits or general utility methods.
}

/// GitHub service implementation.
#[nject::injectable]
struct GitHubService {
    client: GithubClient,
}

/// Implements commit inspection for GitHub.
#[async_trait::async_trait]
impl CommitInspection<GitHubRepository> for GitHubService {
    async fn get_commit(
        &self, repository: &GitHubRepository, commit_id: &str,
    ) -> Result<models::Commit> {
        let params = external_github::apis::repos_api::ReposSlashGetCommitParams::builder()
            .owner(repository.owner.clone())
            .repo(repository.name.clone())
            .r#ref(commit_id.to_string())
            .build()
            .map_err(|e| VcsError::DataParsing(e.to_string()))?;
        self.client
            .repos_slash_get_commit(params)
            .await
            .map_err(|e| VcsError::Network(e.to_string()))
            .map(|commit: GitHubCommit| commit.into())
    }

    async fn list_changes(
        &self, repository: &GitHubRepository, commit_id: &str,
    ) -> Result<Option<Vec<models::DiffEntry>>> {
        let commit = self.get_commit(repository, commit_id).await?;
        Ok(commit.files)
    }
}

/// Implements annotator for GitHub.
impl Annotator for GitHubService {
    /// Formats an annotation into a string suitable for GitHub Actions annotations.
    ///
    /// # Parameters
    /// - `annotation`: The annotation to be formatted.
    ///
    /// # Returns
    /// A string formatted for GitHub Actions annotations.
    fn get_annotation_string(&self, annotation: &Annotation) -> String {
        let location_str = GitHubAnnotator::construct_location_string(annotation);
        format!("::{} {}::{}", annotation.level.to_string(), location_str, annotation.message)
    }
}

// #[cfg(test)]
// mod tests {
//     use tokio;
//
//     use super::*;
//
//     #[tokio::test]
//     async fn test_get_commit_details() {
//         let github = GitHubService {
//             api_token: "your_api_token".to_string(),
//             base_url: "https://api.github.com".to_string(),
//         };
//
//         let repository = GitHubRepository {
//             owner: "octocat".to_string(),
//             name: "Hello-World".to_string(),
//             connection: "your_connection".to_string(),
//         };
//
//         let commit_id = "1e14522488cf65e0e7e9142fae7a8a395414b424";
//         match github.get_commit_details(&repository, commit_id).await {
//             Ok(details) => {
//                 println!("Commit Details: {:?}", details);
//                 assert_eq!(details.author, "geoffreygarrett");
//             }
//             Err(e) => {
//                 eprintln!("Error: {:?}", e);
//                 assert!(false, "Failed to get commit details");
//             }
//         }
//     }
//
//     #[tokio::test]
//     async fn test_list_changes() {
//         let github = GitHubService {
//             api_token: "your_api_token".to_string(),
//             base_url: "https://api.github.com".to_string(),
//         };
//
//         let repository = GitHubRepository {
//             owner: "octocat".to_string(),
//             name: "Hello-World".to_string(),
//             connection: "your_connection".to_string(),
//         };
//
//         let commit_id = "1e14522488cf65e0e7e9142fae7a8a395414b424";
//         match github.list_changes(&repository, commit_id).await {
//             Ok(changes) => {
//                 println!("File Changes: {:?}", changes);
//                 assert!(!changes.is_empty(), "No changes found");
//             }
//             Err(e) => {
//                 eprintln!("Error: {:?}", e);
//                 assert!(false, "Failed to list changes");
//             }
//         }
//     }
// }
