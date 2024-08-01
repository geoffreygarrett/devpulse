// src/lib.rs

use std::convert::From;
use std::str::FromStr;

use externals::external_azure::models as azure;

use crate::models as core;

/// Conversion from Azure's GitCommit to Core's Commit
impl From<azure::GitCommit> for core::Commit {
    fn from(item: azure::GitCommit) -> Self {
        core::Commit {
            sha: item.commit_id.unwrap_or_default(),
            url: item.url.unwrap_or_default(),
            commit_date: item
                .committer
                .as_ref()
                .and_then(|committer| committer.date.as_ref())
                .and_then(|date| prost_types::Timestamp::from_str(date).ok())
                .or_else(|| Some(prost_types::Timestamp::default())),
            message: item.comment.unwrap_or_default(),
            file_changes: item
                .changes
                .unwrap_or_default()
                .iter()
                .map(|change| ((change).clone()).into())
                .collect(),
            parent_shas: item.parents.unwrap_or_default(),
            author: item.author.map(|author| (*author).into()),
            committer: item.committer.map(|committer| (*committer).into()),
            associated_work_items: item
                .work_items
                .unwrap_or_default()
                .iter()
                .map(|wi| ((wi).clone()).into())
                .collect(),
            push_refs: Vec::new(), // Additional push reference conversions if necessary
            statuses: item
                .statuses
                .unwrap_or_default()
                .iter()
                .map(|status| ((status).clone()).into())
                .collect(),
            change_counts: None, // Populate this field if applicable
        }
    }
}

/// Conversion helper from Azure's GitUserDate to Core's GitUser
impl From<azure::GitUserDate> for core::GitUser {
    fn from(user: azure::GitUserDate) -> Self {
        core::GitUser {
            date: user
                .date
                .map(|x| prost_types::Timestamp::from_str(&x).unwrap_or_default()),
            email: Some(user.email.unwrap_or_default()),
            name: Some(user.name.unwrap_or_else(|| "Unknown".to_string())),
            login: None,       // Assuming no corresponding data is available
            id: None,          // Assuming no corresponding data is available
            node_id: None,     // Assuming no corresponding data is available
            avatar_url: None,  // Assuming no corresponding data is available
            url: None,         // Assuming no corresponding data is available
            html_url: None,    // Assuming no corresponding data is available
            site_admin: false, // Assuming no corresponding data is available or default to false
        }
    }
}

/// Conversion from Azure's ResourceRef to a core model Resource
impl From<azure::ResourceRef> for core::ResourceRef {
    fn from(item: azure::ResourceRef) -> Self {
        core::ResourceRef {
            name: None,
            r#type: None,
            id: item.id,
            url: item.url,
        }
    }
}

impl From<azure::GitStatusContext> for core::GitStatusContext {
    fn from(item: azure::GitStatusContext) -> Self {
        core::GitStatusContext {
            genre: item.genre.unwrap_or_default(), // Convert Option<String> to String, defaulting to empty
            name: item.name.unwrap_or_default(), // Ensure name has a default to prevent null issues
        }
    }
}

impl From<azure::IdentityRef> for core::GitUser {
    fn from(item: azure::IdentityRef) -> Self {
        core::GitUser {
            // Convert optional date from string to Timestamp
            date: None, // Assuming IdentityRef does not contain date information

            // Directly use available email, provide default if not present
            email: None,

            // Use display name or default if not available
            name: Some(item.display_name.unwrap_or_else(|| "Unknown".to_string())),

            // Use unique name or provide None if not available
            login: item.unique_name,

            // Convert ID to i64, provide None if not available
            id: item.id.map(|id| id.parse::<i64>().unwrap_or_default()),

            // No node_id available in IdentityRef
            node_id: None,

            // No avatar URL available in IdentityRef
            avatar_url: None,

            // URL is used if available
            url: item.url,

            // No HTML URL available in IdentityRef
            html_url: None,

            // Assuming the user is not a site admin by default
            site_admin: false,
            // TODO: Add an extra metadata catch later.
        }
    }
}

impl From<azure::GitStatus> for core::GitStatus {
    fn from(item: azure::GitStatus) -> Self {
        core::GitStatus {
            context: item.context.map(|x| (*x).into()),
            created_by: item.created_by.map(|u| (*u).into()),
            creation_date: item
                .creation_date
                .map(|d| prost_types::Timestamp::from_str(&d).unwrap_or_default()),
            description: item.description.unwrap_or_default(),
            id: item.id.unwrap_or(0),
            state: match item.state {
                Some(azure::git_status::State::Pending) => core::GitStatusState::Pending as i32,
                Some(azure::git_status::State::Succeeded) => core::GitStatusState::Succeeded as i32,
                Some(azure::git_status::State::Failed) => core::GitStatusState::Failed as i32,
                Some(azure::git_status::State::Error) => core::GitStatusState::Error as i32,
                Some(azure::git_status::State::NotApplicable) => {
                    core::GitStatusState::NotApplicable as i32
                }
                Some(azure::git_status::State::NotSet) => core::GitStatusState::NotSet as i32,
                None => core::GitStatusState::NotSet as i32,
            },
            target_url: item.target_url.unwrap_or_default(),
            updated_date: item
                .updated_date
                .map(|d| prost_types::Timestamp::from_str(&d).unwrap_or_default()),
        }
    }
}

impl From<azure::GitChange> for core::FileChange {
    fn from(item: azure::GitChange) -> Self {
        core::FileChange {
            filename: item.original_path.unwrap_or_default(),
            content_type: "None".to_string(), // TODO: Determine content type
            change_type: item
                .change_type
                .map_or(core::ChangeType::None as i32, |ct| ct as i32),
            additions: 0, // TODO: Determine how to calculate additions
            // item
            // .new_content
            // .as_ref()
            // .map_or(0, |nc| nc.additions.unwrap_or(0)),
            deletions: 0, // TODO: Determine how to calculate deletions
            // item
            //     .new_content
            //     .as_ref()
            //     .map_or(0, |nc| nc.deletions.unwrap_or(0)),
            patch: "None".to_string(), // TODO: Determine how to calculate patch
            // item
            //     .new_content
            //     .as_ref()
            //     .and_then(|nc| nc.patch.clone())
            //     .unwrap_or_default(),
            previous_filename: item.source_server_item.unwrap_or_default(),
        }
    }
}

// impl From<azure::GitCommitChanges> for Vec<models::FileChange> {
//     fn from(item: azure::GitCommitChanges) -> Self {
//         item.changes
//             .unwrap_or_default()
//             .iter()
//             .map(|x| (*x).into())
//             .collect()
//     }
// }
// pub enum State {
//     #[serde(rename = "notSet")]
//     NotSet,
//     #[serde(rename = "pending")]
//     Pending,
//     #[serde(rename = "succeeded")]
//     Succeeded,
//     #[serde(rename = "failed")]
//     Failed,
//     #[serde(rename = "error")]
//     Error,
//     #[serde(rename = "notApplicable")]
//     NotApplicable,
// }
