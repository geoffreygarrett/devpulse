use thiserror::Error;

use crate::models;

#[derive(Error, Debug)]
pub(crate) enum VcsError {
    #[error("Network error: {0}")]
    Network(String),
    #[error("Authorization error")]
    Unauthorized,
    #[error("Data parsing error: {0}")]
    DataParsing(String),
    #[error("Unexpected error")]
    Unexpected,
}

#[async_trait::async_trait]
pub trait CommitInspection<T> {
    async fn get_commit(&self, repository: &T, commit_id: &str)
        -> Result<models::Commit, VcsError>;
    async fn list_changes(
        &self, repo: &T, commit_id: &str,
    ) -> Result<Option<Vec<models::DiffEntry>>, VcsError>;
}
