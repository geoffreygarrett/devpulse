use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use crate::adapters::to_core_commit_from_github::*;

// pub use commit::Commit;
// pub use crate::proto::CommitAuthor;
// pub use commit_commit::CommitCommit;
// pub use commit_parents_inner::CommitParentsInner;
// pub use commit_stats::CommitStats;
// pub use commit_status::CommitStatus;
// // mod access_token;
// pub use verification::Verification;

// pub use crate::adapters::prelude::*;
//
// // pub(crate) mod proto {
// tonic::include_proto!("commit");

// use proto::*;
tonic::include_proto!("commit");

// mod arc_client;
// mod commit_author;
// mod commit_commit;
// mod commit_commit;
// mod commit_parents_inner;
// mod commit_stats;
// mod commit_status;
// mod verification;
// // pub(crate) use arc_client::*;

/// Represents the code churn (additions and deletions) for a specific commit.
#[derive(Debug, Serialize, Deserialize)]
pub struct CodeChurn {
    commit: String,
    additions: usize,
    deletions: usize,
}

impl CodeChurn {
    /// Creates a new `CodeChurn` instance.
    pub fn new(commit: String, additions: usize, deletions: usize) -> Self {
        CodeChurn {
            commit,
            additions,
            deletions,
        }
    }

    /// Returns the commit hash.
    pub fn commit(&self) -> &str {
        &self.commit
    }

    /// Returns the number of additions.
    pub fn additions(&self) -> usize {
        self.additions
    }

    /// Returns the number of deletions.
    pub fn deletions(&self) -> usize {
        self.deletions
    }

    /// Returns the total changes (additions + deletions).
    pub fn total_changes(&self) -> usize {
        self.additions + self.deletions
    }
}

impl AsRef<CodeChurn> for CodeChurn {
    fn as_ref(&self) -> &Self {
        &self
    }
}

/// Represents an analysis of a range of commits in a repository.
#[derive(Serialize, Deserialize)]
pub struct CommitRangeAnalysis {
    pub repository: Repository,
    pub commit_range: CommitRangeDetails,
}

/// Contains details about a range of commits including total commits, additions, deletions, and top contributors.
#[derive(Serialize, Deserialize)]
pub struct CommitRangeDetails {
    pub start_commit: String,
    pub end_commit: String,
    pub total_commits: i32,
    pub total_additions: i32,
    pub total_deletions: i32,
    pub top_contributors: Vec<Contributor>,
}

/// Represents a contributor with a username and the number of commits.
#[derive(Serialize, Deserialize, Debug)]
pub struct Contributor {
    pub username: String,
    pub commits: i32,
}

impl Contributor {
    /// Creates a new `Contributor` instance.
    pub fn new(username: String, commits: i32) -> Self {
        Self { username, commits }
    }

    /// Adds a number of commits to the contributor.
    pub fn add_commits(&mut self, commits: i32) {
        self.commits += commits;
    }
}

/// A connection method to access repositories.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Connection {
    Https,
    Http,
    Ssh,
    Local,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubRepository {
    pub owner: String,
    pub name: String,
    pub connection: Connection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitLabRepository {
    pub owner: String,
    pub name: String,
    pub connection: Connection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BitbucketRepository {
    pub owner: String,
    pub name: String,
    pub connection: Connection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AzureReposRepository {
    pub organization: String,
    pub project: String,
    pub repository: String,
    pub connection: Connection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomRepository {
    pub url: String,
    pub connection: Connection,
}

/// A repository.
///
/// This enum represents a repository, which can be of various types depending on the hosting service.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Repository {
    #[serde(rename = "github")]
    GitHub(GitHubRepository),

    #[serde(rename = "gitlab")]
    GitLab(GitLabRepository),

    #[serde(rename = "bitbucket")]
    Bitbucket(BitbucketRepository),

    #[serde(rename = "azure_repos")]
    AzureRepos(AzureReposRepository),

    #[serde(rename = "custom")]
    Custom(CustomRepository),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Protocol {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "ssh")]
    Ssh,
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Unsupported protocol")]
    UnsupportedProtocol,
    #[error("Invalid URL format")]
    InvalidUrlFormat,
}

impl Repository {
    /// Returns the owner of the repository.
    pub fn owner(&self) -> &str {
        match self {
            Repository::GitHub(repository) => &repository.owner,
            Repository::GitLab(repository) => &repository.owner,
            Repository::Bitbucket(repository) => &repository.owner,
            Repository::AzureRepos(repository) => &repository.organization,
            Repository::Custom(_) => "",
        }
    }

    /// Returns the name of the repository.
    pub fn name(&self) -> &str {
        match self {
            Repository::GitHub(repository) => &repository.name,
            Repository::GitLab(repository) => &repository.name,
            Repository::Bitbucket(repository) => &repository.name,
            Repository::AzureRepos(repository) => &repository.repository,
            Repository::Custom(repository) => &repository.url,
        }
    }

    /// Returns the URL of the repository, formatted based on the given protocol (http or ssh).
    pub fn url(&self, protocol: Protocol) -> Result<String, RepositoryError> {
        match protocol {
            Protocol::Http => match self {
                Repository::GitHub(repository, ..) => {
                    Ok(format!("https://github.com/{}/{}", repository.owner, repository.name))
                }
                Repository::GitLab(repository, ..) => {
                    Ok(format!("https://gitlab.com/{}/{}", repository.owner, repository.name))
                }
                Repository::Bitbucket(repository, ..) => {
                    Ok(format!("https://bitbucket.org/{}/{}", repository.owner, repository.name))
                }
                Repository::AzureRepos(repository, ..) => Ok(format!(
                    "https://dev.azure.com/{}/{}/_git/{}",
                    repository.organization, repository.project, repository.repository
                )),
                Repository::Custom(repository, ..) => Ok(repository.url.clone()),
            },
            Protocol::Ssh => match self {
                Repository::GitHub(repository, ..) => {
                    Ok(format!("git@github.com:{}/{}.git", repository.owner, repository.name))
                }
                Repository::GitLab(repository, ..) => {
                    Ok(format!("git@gitlab.com:{}/{}.git", repository.owner, repository.name))
                }
                Repository::Bitbucket(repository, ..) => {
                    Ok(format!("git@bitbucket.org:{}/{}.git", repository.owner, repository.name))
                }
                Repository::AzureRepos(repository, ..) => Ok(format!(
                    "git@ssh.dev.azure.com:v3/{}/{}/{}",
                    repository.organization, repository.project, repository.repository
                )),
                Repository::Custom(repository, ..) => {
                    if repository.url.starts_with("git@") {
                        Ok(repository.url.clone())
                    } else {
                        Err(RepositoryError::InvalidUrlFormat)
                    }
                }
            },
        }
    }
}
