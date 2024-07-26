use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// DiffEntry : Diff Entry
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct DiffEntry {
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "filename")]
    pub filename: String,
    #[serde(rename = "status")]
    pub status: Status,
    #[serde(rename = "additions")]
    pub additions: i32,
    #[serde(rename = "deletions")]
    pub deletions: i32,
    #[serde(rename = "changes")]
    pub changes: i32,
    #[serde(rename = "blob_url")]
    pub blob_url: String,
    #[serde(rename = "raw_url")]
    pub raw_url: String,
    #[serde(rename = "contents_url")]
    pub contents_url: String,
    #[serde(rename = "patch", skip_serializing_if = "Option::is_none")]
    pub patch: Option<String>,
    #[serde(rename = "previous_filename", skip_serializing_if = "Option::is_none")]
    pub previous_filename: Option<String>,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "added")]
    Added,
    #[serde(rename = "removed")]
    Removed,
    #[serde(rename = "modified")]
    Modified,
    #[serde(rename = "renamed")]
    Renamed,
    #[serde(rename = "copied")]
    Copied,
    #[serde(rename = "changed")]
    Changed,
    #[serde(rename = "unchanged")]
    Unchanged,
}

impl Default for Status {
    fn default() -> Status {
        Self::Added
    }
}

mod _impl_github {
    use external_github;

    impl From<external_github::models::DiffEntry> for super::DiffEntry {
        fn from(internal: external_github::models::DiffEntry) -> Self {
            super::DiffEntry {
                sha: internal.sha,
                filename: internal.filename,
                status: internal.status.into(),
                additions: internal.additions,
                deletions: internal.deletions,
                changes: internal.changes,
                blob_url: internal.blob_url,
                raw_url: internal.raw_url,
                contents_url: internal.contents_url,
                patch: internal.patch,
                previous_filename: internal.previous_filename,
            }
        }
    }

    impl From<super::DiffEntry> for external_github::models::DiffEntry {
        fn from(external: super::DiffEntry) -> Self {
            external_github::models::DiffEntry {
                sha: external.sha,
                filename: external.filename,
                status: external.status.into(),
                additions: external.additions,
                deletions: external.deletions,
                changes: external.changes,
                blob_url: external.blob_url,
                raw_url: external.raw_url,
                contents_url: external.contents_url,
                patch: external.patch,
                previous_filename: external.previous_filename,
            }
        }
    }

    impl From<external_github::models::diff_entry::Status> for super::Status {
        fn from(internal: external_github::models::diff_entry::Status) -> Self {
            match internal {
                external_github::models::diff_entry::Status::Added => Self::Added,
                external_github::models::diff_entry::Status::Removed => Self::Removed,
                external_github::models::diff_entry::Status::Modified => Self::Modified,
                external_github::models::diff_entry::Status::Renamed => Self::Renamed,
                external_github::models::diff_entry::Status::Copied => Self::Copied,
                external_github::models::diff_entry::Status::Changed => Self::Changed,
                external_github::models::diff_entry::Status::Unchanged => Self::Unchanged,
            }
        }
    }

    impl From<super::Status> for external_github::models::diff_entry::Status {
        fn from(external: super::Status) -> Self {
            match external {
                super::Status::Added => external_github::models::diff_entry::Status::Added,
                super::Status::Removed => external_github::models::diff_entry::Status::Removed,
                super::Status::Modified => external_github::models::diff_entry::Status::Modified,
                super::Status::Renamed => external_github::models::diff_entry::Status::Renamed,
                super::Status::Copied => external_github::models::diff_entry::Status::Copied,
                super::Status::Changed => external_github::models::diff_entry::Status::Changed,
                super::Status::Unchanged => external_github::models::diff_entry::Status::Unchanged,
            }
        }
    }
}
