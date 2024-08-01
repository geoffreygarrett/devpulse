pub use analyze_commit_range::*;

use crate::commit_inspection::VcsError;

mod analyze_commit_range;
mod azure_service;
mod git_service;
mod github_service;
mod vcs_service;

/// A result type for VCS operations.
pub type Result<T> = std::result::Result<T, VcsError>;
