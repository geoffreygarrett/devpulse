// use async_trait::async_trait;
// use log::{error, info};
// use reqwest::{Client, Error as ReqwestError};
// use serde_json::json;
// use thiserror::Error;
//
// #[derive(Debug, Error)]
// pub enum CommenterError {
//     #[error("HTTP Error: {0} {1}")]
//     HttpError(String, u16),
//     #[error("Network Error: {0}")]
//     NetworkError(String),
//     #[error("Invalid Input: {0}")]
//     InvalidInput(String),
// }
//
// type Result<T> = std::result::Result<T, CommenterError>;
//
// struct GitHubCommenter {
//     client: Client,
//     config: GitHubConfig,
// }
//
// struct GitHubConfig {
//     api_token: String,
//     base_url: String,
//     owner: String,
//     repo: String,
// }
//
// impl GitHubCommenter {
//     async fn post_issue_comment(&self, issue_number: &str, body: &str) -> Result<()> {
//         let url = format!(
//             "{}/repos/{}/{}/issues/{}/comments",
//             self.config.base_url, self.config.owner, self.config.repo, issue_number
//         );
//         self.post_comment_generic(&url, body).await
//     }
//
//     async fn post_pull_request_comment(&self, pr_id: &str, body: &str) -> Result<()> {
//         let url = format!(
//             "{}/repos/{}/{}/pulls/{}/comments",
//             self.config.base_url, self.config.owner, self.config.repo, pr_id
//         );
//         self.post_comment_generic(&url, body).await
//     }
//
//     async fn post_commit_comment(
//         &self, commit_id: &str, body: &str, path: Option<&str>, line: Option<usize>,
//     ) -> Result<()> {
//         let url = format!(
//             "{}/repos/{}/{}/commits/{}/comments",
//             self.config.base_url, self.config.owner, self.config.repo, commit_id
//         );
//         let payload = json!({
//             "body": body,
//             "path": path,
//             "line": line,
//             "position": 1  // 'position' needs specific handling depending on the commit diff
//         });
//         self.post_comment_generic_with_payload(&url, &payload).await
//     }
//
//     async fn post_comment_generic_with_payload(
//         &self, url: &str, payload: &serde_json::Value,
//     ) -> Result<()> {
//         info!("Posting comment to {}", url);
//         let response = self
//             .client
//             .post(url)
//             .bearer_auth(&self.config.api_token)
//             .json(payload)
//             .send()
//             .await;
//
//         match response {
//             Ok(res) if res.status().is_success() => Ok(()),
//             Ok(res) => {
//                 error!(
//                     "HTTP Error: {}, Status Code: {}",
//                     res.status(),
//                     res.text().await.unwrap_or_default()
//                 );
//                 Err(CommenterError::HttpError(
//                     res.text().await.unwrap_or_default(),
//                     res.status().as_u16(),
//                 ))
//             }
//             Err(e) => {
//                 error!("Network Error: {}", e);
//                 Err(CommenterError::NetworkError(e.to_string()))
//             }
//         }
//     }
// }
