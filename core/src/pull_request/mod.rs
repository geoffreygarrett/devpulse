pub use error::*;

use crate::annotations::prelude::*;

pub(crate) mod error;
pub(crate) mod models;
pub(crate) mod service;

/// Trait for services that process pull requests and generate summaries.
#[allow(dead_code)]
pub trait PullRequestSummaryService {
    /// Processes a pull request by its ID and collects summaries from its segments.
    fn generate_summary(&self, pr_id: u64) -> Result<String, PullRequestError>;
}

/// Trait for services that process pull requests and generate detailed code annotations.
#[allow(dead_code)]
pub trait PullRequestAnnotationService {
    /// Processes a pull request by its ID and generates annotations for code ranges.
    fn generate_annotations(&self, pr_id: u64) -> Result<Vec<Annotation>, PullRequestError>;
}

/// Enum representing various supported version control services.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum VersionControlService {
    GitHub,
    AzureRepos,
    Bitbucket,
    GitLab,
}

#[allow(dead_code)]
impl VersionControlService {
    /// Returns the base API URL for the service.
    pub fn api_base_url(&self) -> &str {
        match self {
            VersionControlService::GitHub => "https://api.github.com",
            VersionControlService::AzureRepos => "https://dev.azure.com",
            VersionControlService::Bitbucket => "https://api.bitbucket.org",
            VersionControlService::GitLab => "https://gitlab.com/api/v4",
        }
    }

    /// Returns a human-readable name of the service.
    pub fn display_name(&self) -> &str {
        match self {
            VersionControlService::GitHub => "GitHub",
            VersionControlService::AzureRepos => "Azure Repos",
            VersionControlService::Bitbucket => "Bitbucket",
            VersionControlService::GitLab => "GitLab",
        }
    }

    /// Constructs a specific API endpoint for fetching pull request data.
    pub fn pull_request_endpoint(&self, repo_identifier: &str, pr_id: u64) -> String {
        match self {
            VersionControlService::GitHub => {
                format!("{}/repos/{}/pulls/{}", self.api_base_url(), repo_identifier, pr_id)
            }
            VersionControlService::AzureRepos => format!(
                "{}/_apis/git/repositories/{}/pullRequests/{}",
                self.api_base_url(),
                repo_identifier,
                pr_id
            ),
            VersionControlService::Bitbucket => format!(
                "{}/repositories/{}/pullrequests/{}",
                self.api_base_url(),
                repo_identifier,
                pr_id
            ),
            VersionControlService::GitLab => format!(
                "{}/projects/{}/merge_requests/{}",
                self.api_base_url(),
                repo_identifier.replace('/', "%2F"),
                pr_id
            ),
        }
    }
}
