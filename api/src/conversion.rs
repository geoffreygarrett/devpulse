// core/src/models/conversions.rs

use devpulse_core::models as core_models;

use crate::models as api_models;

// CommitRangeAnalysis: BLL -> API
impl From<core_models::CommitRangeAnalysis> for api_models::CommitRangeAnalysis {
    fn from(analysis: core_models::CommitRangeAnalysis) -> Self {
        api_models::CommitRangeAnalysis {
            repository: analysis.repository.into(),
            commit_range: api_models::CommitRangeDetails {
                start_commit: analysis.commit_range.start_commit,
                end_commit: analysis.commit_range.end_commit,
                total_commits: analysis.commit_range.total_commits,
                total_additions: analysis.commit_range.total_additions,
                total_deletions: analysis.commit_range.total_deletions,
                top_contributors: analysis
                    .commit_range
                    .top_contributors
                    .into_iter()
                    .map(api_models::Contributor::from)
                    .collect(),
            },
        }
    }
}

// CommitRangeAnalysis: API -> BLL
impl From<api_models::CommitRangeAnalysis> for core_models::CommitRangeAnalysis {
    fn from(analysis: api_models::CommitRangeAnalysis) -> Self {
        core_models::CommitRangeAnalysis {
            repository: analysis.repository.into(),
            commit_range: core_models::CommitRangeDetails {
                start_commit: analysis.commit_range.start_commit,
                end_commit: analysis.commit_range.end_commit,
                total_commits: analysis.commit_range.total_commits,
                total_additions: analysis.commit_range.total_additions,
                total_deletions: analysis.commit_range.total_deletions,
                top_contributors: analysis
                    .commit_range
                    .top_contributors
                    .into_iter()
                    .map(core_models::Contributor::from)
                    .collect(),
            },
        }
    }
}

// Contributor: BLL -> API
impl From<core_models::Contributor> for api_models::Contributor {
    fn from(contributor: core_models::Contributor) -> Self {
        api_models::Contributor {
            username: contributor.username,
            commits: contributor.commits,
        }
    }
}

// Contributor: API -> BLL
impl From<api_models::Contributor> for core_models::Contributor {
    fn from(contributor: api_models::Contributor) -> Self {
        core_models::Contributor {
            username: contributor.username,
            commits: contributor.commits,
        }
    }
}

// Repository: BLL -> API
impl From<core_models::Repository> for api_models::Repository {
    fn from(repository: core_models::Repository) -> Self {
        match repository {
            core_models::Repository::GitHub(repo) => {
                api_models::Repository::GitHub(api_models::GitHubRepository {
                    owner: repo.owner,
                    name: repo.name,
                    connection: repo.connection.into(),
                })
            }
            core_models::Repository::GitLab(repo) => {
                api_models::Repository::GitLab(api_models::GitLabRepository {
                    owner: repo.owner,
                    name: repo.name,
                    connection: repo.connection.into(),
                })
            }
            core_models::Repository::Bitbucket(repo) => {
                api_models::Repository::Bitbucket(api_models::BitbucketRepository {
                    owner: repo.owner,
                    name: repo.name,
                    connection: repo.connection.into(),
                })
            }
            core_models::Repository::AzureRepos(repo) => {
                api_models::Repository::AzureRepos(api_models::AzureReposRepository {
                    organization: repo.organization,
                    project: repo.project,
                    repository: repo.repository,
                    connection: repo.connection.into(),
                })
            }
            core_models::Repository::Custom(repo) => {
                api_models::Repository::Custom(api_models::CustomRepository {
                    url: repo.url,
                    connection: repo.connection.into(),
                })
            }
        }
    }
}

// Repository: API -> BLL
impl From<api_models::Repository> for core_models::Repository {
    fn from(repository: api_models::Repository) -> Self {
        match repository {
            api_models::Repository::GitHub(repo) => {
                core_models::Repository::GitHub(core_models::GitHubRepository {
                    owner: repo.owner,
                    name: repo.name,
                    connection: repo.connection.into(),
                })
            }
            api_models::Repository::GitLab(repo) => {
                core_models::Repository::GitLab(core_models::GitLabRepository {
                    owner: repo.owner,
                    name: repo.name,
                    connection: repo.connection.into(),
                })
            }
            api_models::Repository::Bitbucket(repo) => {
                core_models::Repository::Bitbucket(core_models::BitbucketRepository {
                    owner: repo.owner,
                    name: repo.name,
                    connection: repo.connection.into(),
                })
            }
            api_models::Repository::AzureRepos(repo) => {
                core_models::Repository::AzureRepos(core_models::AzureReposRepository {
                    organization: repo.organization,
                    project: repo.project,
                    repository: repo.repository,
                    connection: repo.connection.into(),
                })
            }
            api_models::Repository::Custom(repo) => {
                core_models::Repository::Custom(core_models::CustomRepository {
                    url: repo.url,
                    connection: repo.connection.into(),
                })
            }
        }
    }
}

// Connection: BLL -> API
impl From<core_models::Connection> for api_models::Connection {
    fn from(connection: core_models::Connection) -> Self {
        api_models::Connection {
            protocol: connection.protocol.into(),
            token: connection.token,
            url: connection.url,
        }
    }
}

// Connection: API -> BLL
impl From<api_models::Connection> for core_models::Connection {
    fn from(connection: api_models::Connection) -> Self {
        core_models::Connection {
            protocol: connection.protocol.into(),
            token: connection.token,
            url: connection.url,
        }
    }
}

// Protocol: BLL -> API
impl From<core_models::Protocol> for api_models::Protocol {
    fn from(protocol: core_models::Protocol) -> Self {
        match protocol {
            core_models::Protocol::Https => api_models::Protocol::Https,
            core_models::Protocol::Http => api_models::Protocol::Http,
            core_models::Protocol::Ssh => api_models::Protocol::Ssh,
            core_models::Protocol::Local => api_models::Protocol::Local,
        }
    }
}

// Protocol: API -> BLL
impl From<api_models::Protocol> for core_models::Protocol {
    fn from(protocol: api_models::Protocol) -> Self {
        match protocol {
            api_models::Protocol::Https => core_models::Protocol::Https,
            api_models::Protocol::Http => core_models::Protocol::Http,
            api_models::Protocol::Ssh => core_models::Protocol::Ssh,
            api_models::Protocol::Local => core_models::Protocol::Local,
        }
    }
}
