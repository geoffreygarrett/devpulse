use serde::{Deserialize, Serialize};

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
    pub repository: String,
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
