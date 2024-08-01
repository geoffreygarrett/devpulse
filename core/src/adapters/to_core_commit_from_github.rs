// src/lib.rs

// Standard library imports
use std::convert::From;
use std::str::FromStr;

// External GitHub API models
use externals::external_github::models as github;

// Internal core models
use crate::models as core;

/// Conversion from GitHub's NullableGitUser to Core's GitUser
impl From<github::nullable_git_user::NullableGitUser> for core::GitUser {
    fn from(user: github::nullable_git_user::NullableGitUser) -> Self {
        core::GitUser {
            date: user
                .date
                .map(|x| prost_types::Timestamp::from_str(&x).unwrap_or_default()),
            email: user.email,
            name: Some(user.name.unwrap_or_else(|| "Unknown".to_string())),
            login: None,
            id: None,
            node_id: None,
            avatar_url: None,
            url: None,
            html_url: None,
            site_admin: false,
        }
    }
}

/// Conversion from GitHub's Commit model to Core's Commit model
impl From<github::commit::Commit> for core::Commit {
    fn from(item: github::commit::Commit) -> Self {
        core::Commit {
            sha: item.sha.clone(),
            url: item.url.clone(),
            commit_date: item
                .commit
                .author
                .as_ref()
                .and_then(|author| author.date.as_ref())
                .and_then(|date| prost_types::Timestamp::from_str(date).ok())
                .or_else(|| Some(prost_types::Timestamp::default())),
            message: item.commit.message.clone(),
            file_changes: item
                .files
                .map(|x| x.iter().map(|file| (file.clone()).into()).collect())
                .unwrap_or_default(),
            parent_shas: item.parents.iter().map(|p| p.sha.clone()).collect(),
            author: Some(
                item.commit
                    .author
                    .as_ref()
                    .map(|author| (*author.clone()).into())
                    .unwrap_or_default(),
            ),
            committer: Some(
                item.commit
                    .committer
                    .as_ref()
                    .map(|committer| (*committer.clone()).into())
                    .unwrap_or_default(),
            ),
            associated_work_items: Vec::new(),
            push_refs: Vec::new(),
            statuses: Vec::new(),
            change_counts: None,
        }
    }
}

/// Conversion from GitHub's DiffEntry model to Core's FileChange model
impl From<github::diff_entry::DiffEntry> for core::FileChange {
    fn from(item: github::diff_entry::DiffEntry) -> Self {
        core::FileChange {
            filename: item.filename,
            content_type: "None".to_string(), // TODO: Determine content type
            change_type: match item.status {
                github::diff_entry::Status::Added => core::ChangeType::Add as i32,
                github::diff_entry::Status::Modified => core::ChangeType::Edit as i32,
                github::diff_entry::Status::Removed => core::ChangeType::Delete as i32,
                github::diff_entry::Status::Renamed => core::ChangeType::Rename as i32,
                github::diff_entry::Status::Copied => core::ChangeType::Copy as i32,
                _ => core::ChangeType::None as i32,
            },
            additions: item.additions,
            deletions: item.deletions,
            patch: item.patch.unwrap_or_default(),
            previous_filename: item.previous_filename.unwrap_or_default(),
        }
    }
}
