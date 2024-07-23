use serde::{Deserialize, Serialize};

use thiserror::Error;

mod base;
mod github;

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

/// A repository.
///
/// This enum represents a repository, which can be of various types depending on the hosting service.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Repository {
    #[serde(rename = "github")]
    GitHub { owner: String, name: String },
    #[serde(rename = "gitlab")]
    GitLab { owner: String, name: String },
    #[serde(rename = "bitbucket")]
    Bitbucket { owner: String, name: String },
    #[serde(rename = "azure_repos")]
    AzureRepos {
        organization: String,
        project: String,
        repository: String,
    },
    #[serde(rename = "custom")]
    Custom { url: String },
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
            Repository::GitHub { owner, .. }
            | Repository::GitLab { owner, .. }
            | Repository::Bitbucket { owner, .. } => owner,
            Repository::AzureRepos { organization, .. } => organization,
            Repository::Custom { .. } => "",
        }
    }

    /// Returns the name of the repository.
    pub fn name(&self) -> &str {
        match self {
            Repository::GitHub { name, .. }
            | Repository::GitLab { name, .. }
            | Repository::Bitbucket { name, .. } => name,
            Repository::AzureRepos { repository, .. } => repository,
            Repository::Custom { url } => url,
        }
    }

    /// Returns the URL of the repository, formatted based on the given protocol (http or ssh).
    pub fn url(&self, protocol: Protocol) -> Result<String, RepositoryError> {
        match protocol {
            Protocol::Http => match self {
                Repository::GitHub { owner, name } => {
                    Ok(format!("https://github.com/{}/{}", owner, name))
                }
                Repository::GitLab { owner, name } => {
                    Ok(format!("https://gitlab.com/{}/{}", owner, name))
                }
                Repository::Bitbucket { owner, name } => {
                    Ok(format!("https://bitbucket.org/{}/{}", owner, name))
                }
                Repository::AzureRepos {
                    organization,
                    project,
                    repository,
                } => Ok(format!(
                    "https://dev.azure.com/{}/{}/_git/{}",
                    organization, project, repository
                )),
                Repository::Custom { url } => Ok(url.clone()),
            },
            Protocol::Ssh => match self {
                Repository::GitHub { owner, name } => {
                    Ok(format!("git@github.com:{}/{}.git", owner, name))
                }
                Repository::GitLab { owner, name } => {
                    Ok(format!("git@gitlab.com:{}/{}.git", owner, name))
                }
                Repository::Bitbucket { owner, name } => {
                    Ok(format!("git@bitbucket.org:{}/{}.git", owner, name))
                }
                Repository::AzureRepos {
                    organization,
                    project,
                    repository,
                } => Ok(format!(
                    "git@ssh.dev.azure.com:v3/{}/{}/{}",
                    organization, project, repository
                )),
                Repository::Custom { url } => {
                    if url.starts_with("git@") {
                        Ok(url.clone())
                    } else {
                        Err(RepositoryError::InvalidUrlFormat)
                    }
                }
            },
        }
    }
}
