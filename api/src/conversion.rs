// core/src/models/conversions.rs
use crate::models::{CommitRangeAnalysis, CommitRangeDetails, Contributor};

impl From<devpulse_core::models::CommitRangeAnalysis> for CommitRangeAnalysis {
    fn from(analysis: devpulse_core::models::CommitRangeAnalysis) -> Self {
        CommitRangeAnalysis {
            repository: analysis.repository,
            commit_range: CommitRangeDetails {
                start_commit: analysis.commit_range.start_commit,
                end_commit: analysis.commit_range.end_commit,
                total_commits: analysis.commit_range.total_commits,
                total_additions: analysis.commit_range.total_additions,
                total_deletions: analysis.commit_range.total_deletions,
                top_contributors: analysis
                    .commit_range
                    .top_contributors
                    .into_iter()
                    .map(Contributor::from)
                    .collect(),
            },
        }
    }
}

impl From<devpulse_core::models::Contributor> for Contributor {
    fn from(contributor: devpulse_core::models::Contributor) -> Self {
        Contributor {
            username: contributor.username,
            commits: contributor.commits,
        }
    }
}
