use externals::external_azure as azure;

use crate::annotations::prelude::{Annotation, Annotator, AzureAnnotator};
use crate::models;
use crate::models::AzureReposRepository;
use crate::prelude::*;

use super::Result;

/// Azure DevOps service.
struct AzureService {
    client: azure::apis::AzureClient,
}

/// Implement commit inspection for Azure DevOps.
#[async_trait::async_trait]
impl CommitInspection<AzureReposRepository> for AzureService {
    async fn get_commit(
        &self, repository: &AzureReposRepository, commit_id: &str,
    ) -> Result<models::Commit> {
        let params = azure::apis::commits_api::CommitsGetParams::builder()
            .organization(repository.organization.clone())
            .project(repository.project.clone())
            .repository_id(repository.repository.clone())
            .commit_id(commit_id.to_string())
            .api_version("6.0".to_string())
            .build()
            .map_err(|e| VcsError::DataParsing(e.to_string()))?;
        self.client
            .commits_get(params)
            .await
            .map_err(|e| VcsError::Network(e.to_string()))
            .map(Into::into)
    }
    async fn list_changes(
        &self, repository: &AzureReposRepository, commit_id: &str,
    ) -> Result<Option<Vec<models::FileChange>>> {
        let params = azure::apis::commits_api::CommitsGetChangesParams::builder()
            .organization(repository.organization.clone())
            .project(repository.project.clone())
            .repository_id(repository.repository.clone())
            .commit_id(commit_id.to_string())
            .api_version("6.0".to_string())
            .build()
            .map_err(|e| VcsError::DataParsing(e.to_string()))?;

        let changes_result = self
            .client
            .commits_get_changes(params)
            .await
            .map_err(|e| VcsError::Network(e.to_string()));

        changes_result.map(|changes| {
            changes.changes.map_or(None, |git_changes| {
                let file_changes: Vec<_> = git_changes.into_iter()
                    .map(|x| x.into()) // Assuming you have implemented From<GitChange> for FileChange
                    .collect();
                if file_changes.is_empty() {
                    None
                } else {
                    Some(file_changes)
                }
            })
        })
    }
}

/// Implement annotation for Azure DevOps.
impl Annotator for AzureService {
    fn get_annotation_string(&self, annotation: &Annotation) -> String {
        let location_str = AzureAnnotator::construct_location_string(annotation);
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
//         let azure = AzureDevOps {
//             api_token: "your_api_token".to_string(),
//             organization: "your_organization".to_string(),
//             project: "your_project".to_string(),
//             repository: "your_repository".to_string(),
//         };
//
//         let repository = AzureReposRepository {
//             organization: "your_organization".to_string(),
//             project: "your_project".to_string(),
//             repository: "your_repository".to_string(),
//             connection: "your_connection".to_string(),
//         };
//
//         let commit_id = "your_commit_id";
//         match azure.get_commit_details(&repository, commit_id).await {
//             Ok(details) => {
//                 println!("Commit Details: {:?}", details);
//                 assert_eq!(details.author, "expected_author");
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
//         let azure = AzureDevOps {
//             api_token: "your_api_token".to_string(),
//             organization: "your_organization".to_string(),
//             project: "your_project".to_string(),
//             repository: "your_repository".to_string(),
//         };
//
//         let repository = AzureReposRepository {
//             organization: "your_organization".to_string(),
//             project: "your_project".to_string(),
//             repository: "your_repository".to_string(),
//             connection: "your_connection".to_string(),
//         };
//
//         let commit_id = "your_commit_id";
//         match azure.list_changes(&repository, commit_id).await {
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
