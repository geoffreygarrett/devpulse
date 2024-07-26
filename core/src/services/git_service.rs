// use async_trait::async_trait;
// use git2::{Oid, Repository};
// use serde::{Deserialize, Serialize};
//
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct CommitDetails {
//     author: String,
//     message: String,
//     date: String,
// }
//
// #[derive(Debug, Serialize, Deserialize, Clone)]
// pub struct FileChange {
//     file_path: String,
//     changes: usize,
//     additions: usize,
//     deletions: usize,
// }
//
// #[derive(Debug, thiserror::Error)]
// pub enum VcsError {
//     #[error("Network error: {0}")]
//     Network(String),
//     #[error("Data parsing error: {0}")]
//     DataParsing(String),
//     #[error("Unexpected error")]
//     Unexpected,
// }
//
// type Result<T> = std::result::Result<T, VcsError>;
//
// #[async_trait]
// pub trait CommitInspection<T> {
//     async fn get_commit_details(&self, repository: &T, commit_id: &str) -> Result<CommitDetails>;
//     async fn list_changes(&self, repository: &T, commit_id: &str) -> Result<Vec<FileChange>>;
// }
//
// pub struct LocalGitRepository {
//     pub path: String,
// }
//
// pub struct LocalGitService;
//
// #[async_trait]
// impl CommitInspection<LocalGitRepository> for LocalGitService {
//     async fn get_commit_details(
//         &self, repository: &LocalGitRepository, commit_id: &str,
//     ) -> Result<CommitDetails> {
//         let repo = Repository::open(&repository.path)?;
//         let oid = Oid::from_str(commit_id)?;
//         let commit = repo.find_commit(oid)?;
//         let author = commit.author();
//
//         Ok(CommitDetails {
//             author: author.name().unwrap_or("").to_string(),
//             message: commit.message().unwrap_or("").to_string(),
//             date: commit.time().seconds().to_string(),
//         })
//     }
//
//     async fn list_changes(
//         &self, repository: &LocalGitRepository, commit_id: &str,
//     ) -> Result<Vec<FileChange>> {
//         let repo = Repository::open(&repository.path)?;
//         let oid = Oid::from_str(commit_id)?;
//         let commit = repo.find_commit(oid)?;
//         let tree = commit.tree()?;
//         let parent_tree = if let Some(parent) = commit.parent(0).ok() {
//             parent.tree().ok()
//         } else {
//             None
//         };
//
//         let diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), None)?;
//         let mut changes = vec![];
//
//         diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
//             changes.push(FileChange {
//                 file_path: line.content().to_vec(),
//                 changes: 1, // Simplified, should calculate based on line context
//                 additions: if line.origin() == '+' { 1 } else { 0 },
//                 deletions: if line.origin() == '-' { 1 } else { 0 },
//             });
//             true
//         })?;
//
//         Ok(changes)
//     }
// }
