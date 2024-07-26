// use reqwest::Client;
// use serde::de::DeserializeOwned;
//
// use crate::annotations::prelude::{Annotation, Annotator, AzureAnnotator};
// use crate::models::AzureReposRepository;
// use crate::prelude::*;
//
// type Result<T> = std::result::Result<T, VcsError>;
//
// /// Azure DevOps service.
// struct AzureDevOps {
//     api_token: String,
//     organization: String,
//     project: String,
//     repository: String,
// }
//
// impl AzureDevOps {
//     /// Fetches data from a given URL using the provided API token.
//     async fn fetch_data<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
//         Client::new()
//             .get(url)
//             .bearer_auth(&self.api_token)
//             .send()
//             .await
//             .map_err(|e| VcsError::Network(e.to_string()))?
//             .json::<T>()
//             .await
//             .map_err(|e| VcsError::DataParsing(e.to_string()))
//     }
// }
//
// /// Implement commit inspection for Azure DevOps.
// #[async_trait::async_trait]
// impl CommitInspection<AzureReposRepository> for AzureDevOps {
//     async fn get_commit_details(
//         &self, repository: &AzureReposRepository, commit_id: &str,
//     ) -> Result<CommitDetails> {
//         let url = format!(
//             "https://dev.azure.com/{}/{}/_apis/git/repositories/{}/commits/{}",
//             self.organization, self.project, repository.repository, commit_id
//         );
//         self.fetch_data(&url).await
//     }
//
//     async fn list_changes(
//         &self, repository: &AzureReposRepository, commit_id: &str,
//     ) -> Result<Vec<FileChange>> {
//         let url = format!(
//             "https://dev.azure.com/{}/{}/_apis/git/repositories/{}/commits/{}/changes",
//             self.organization, self.project, repository.repository, commit_id
//         );
//         self.fetch_data(&url).await
//     }
// }
//
// /// Implement annotation for Azure DevOps.
// impl Annotator for AzureDevOps {
//     fn get_annotation_string(&self, annotation: &Annotation) -> String {
//         let location_str = AzureAnnotator::construct_location_string(annotation);
//         format!("::{} {}::{}", annotation.level.to_string(), location_str, annotation.message)
//     }
// }
//
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
