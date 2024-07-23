// core/src/models/conversions.rs

// CommitRangeAnalysis: BLL -> API
impl From<devpulse_core::models::CommitRangeAnalysis> for crate::models::CommitRangeAnalysis {
    fn from(analysis: devpulse_core::models::CommitRangeAnalysis) -> Self {
        crate::models::CommitRangeAnalysis {
            repository: analysis.repository.into(),
            commit_range: crate::models::CommitRangeDetails {
                start_commit: analysis.commit_range.start_commit,
                end_commit: analysis.commit_range.end_commit,
                total_commits: analysis.commit_range.total_commits,
                total_additions: analysis.commit_range.total_additions,
                total_deletions: analysis.commit_range.total_deletions,
                top_contributors: analysis
                    .commit_range
                    .top_contributors
                    .into_iter()
                    .map(crate::models::Contributor::from)
                    .collect(),
            },
        }
    }
}

// CommitRangeAnalysis: API -> BLL
impl From<crate::models::CommitRangeAnalysis> for devpulse_core::models::CommitRangeAnalysis {
    fn from(analysis: crate::models::CommitRangeAnalysis) -> Self {
        devpulse_core::models::CommitRangeAnalysis {
            repository: analysis.repository.into(),
            commit_range: devpulse_core::models::CommitRangeDetails {
                start_commit: analysis.commit_range.start_commit,
                end_commit: analysis.commit_range.end_commit,
                total_commits: analysis.commit_range.total_commits,
                total_additions: analysis.commit_range.total_additions,
                total_deletions: analysis.commit_range.total_deletions,
                top_contributors: analysis
                    .commit_range
                    .top_contributors
                    .into_iter()
                    .map(devpulse_core::models::Contributor::from)
                    .collect(),
            },
        }
    }
}

// Contributor: BLL -> API
impl From<devpulse_core::models::Contributor> for crate::models::Contributor {
    fn from(contributor: devpulse_core::models::Contributor) -> Self {
        crate::models::Contributor {
            username: contributor.username,
            commits: contributor.commits,
        }
    }
}

// Contributor: API -> BLL
impl From<crate::models::Contributor> for devpulse_core::models::Contributor {
    fn from(contributor: crate::models::Contributor) -> Self {
        devpulse_core::models::Contributor {
            username: contributor.username,
            commits: contributor.commits,
        }
    }
}

// Repository: BLL -> API
impl From<devpulse_core::models::Repository> for crate::models::Repository {
    fn from(repository: devpulse_core::models::Repository) -> Self {
        match repository {
            devpulse_core::models::Repository::GitHub { owner, name } => {
                crate::models::Repository::GitHub { owner, name }
            }
            devpulse_core::models::Repository::GitLab { owner, name } => {
                crate::models::Repository::GitLab { owner, name }
            }
            devpulse_core::models::Repository::Bitbucket { owner, name } => {
                crate::models::Repository::Bitbucket { owner, name }
            }
            devpulse_core::models::Repository::Custom { url } => {
                crate::models::Repository::Custom { url }
            }
            devpulse_core::models::Repository::AzureRepos {
                organization,
                project,
                repository,
            } => crate::models::Repository::AzureRepos {
                organization,
                project,
                repository,
            },
        }
    }
}

// Repository: API -> BLL
impl From<crate::models::Repository> for devpulse_core::models::Repository {
    fn from(repository: crate::models::Repository) -> Self {
        match repository {
            crate::models::Repository::GitHub { owner, name } => {
                devpulse_core::models::Repository::GitHub { owner, name }
            }
            crate::models::Repository::GitLab { owner, name } => {
                devpulse_core::models::Repository::GitLab { owner, name }
            }
            crate::models::Repository::Bitbucket { owner, name } => {
                devpulse_core::models::Repository::Bitbucket { owner, name }
            }
            crate::models::Repository::Custom { url } => {
                devpulse_core::models::Repository::Custom { url }
            }
            crate::models::Repository::AzureRepos {
                organization,
                project,
                repository,
            } => devpulse_core::models::Repository::AzureRepos {
                organization,
                project,
                repository,
            },
        }
    }
}
