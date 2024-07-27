/*
 * Based on GitHub's API at the time of writing.
 */

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::models;

/// Commit : Commit
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct Commit {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "commit")]
    pub commit: Box<models::CommitCommit>,
    #[serde(rename = "author", deserialize_with = "Option::deserialize")]
    pub author: Option<Box<models::CommitAuthor>>,
    #[serde(rename = "committer", deserialize_with = "Option::deserialize")]
    pub committer: Option<Box<models::CommitAuthor>>,
    #[serde(rename = "parents")]
    pub parents: Vec<models::CommitParentsInner>,
    #[serde(rename = "stats", skip_serializing_if = "Option::is_none")]
    pub stats: Option<Box<models::CommitStats>>,
    #[serde(rename = "files", skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<models::DiffEntry>>,
}

impl Commit {
    /// Commit
    pub fn new(
        url: String, sha: String, node_id: String, html_url: String, comments_url: String,
        commit: models::CommitCommit, author: Option<models::CommitAuthor>,
        committer: Option<models::CommitAuthor>, parents: Vec<models::CommitParentsInner>,
    ) -> Commit {
        Commit {
            url,
            sha,
            node_id,
            html_url,
            comments_url,
            commit: Box::new(commit),
            author: if let Some(x) = author {
                Some(Box::new(x))
            } else {
                None
            },
            committer: if let Some(x) = committer {
                Some(Box::new(x))
            } else {
                None
            },
            parents,
            stats: None,
            files: None,
        }
    }

    pub fn builder() -> CommitBuilder {
        CommitBuilder::create_empty()
    }
}

mod _impl_azure {

    // impl From<external_azure::models::GitCommit> for models::Commit {
    //     fn from(commit: external_azure::models::GitCommit) -> Self {
    //         models::Commit {
    //             url: commit.remote_url.unwrap_or_default(),
    //             sha: commit.commit_id.unwrap_or_default(),
    //             node_id: commit.commit_id.clone().unwrap_or_default(), // Placeholder
    //             html_url: commit.remote_url.clone().unwrap_or_default(), // Placeholder
    //             comments_url: String::new(),                           // Default empty string
    //             commit: Box::new(models::CommitCommit {
    //                 // Map fields from Azure's commit structure to our internal CommitCommit structure
    //                 // Provide default values if necessary
    //                 author: commit
    //                     .author
    //                     .as_ref()
    //                     .map(|x| x.name.clone().unwrap_or_default()),
    //                 message: commit.comment.clone().unwrap_or_default(),
    //                 date: commit
    //                     .author
    //                     .as_ref()
    //                     .map(|x| x.date.clone().unwrap_or_default())
    //                     .unwrap_or_default(),
    //                 tree: commit.tree_id.clone().unwrap_or_default(),
    //                 parents: commit.parents.clone().unwrap_or_default(),
    //             }),
    //             author: commit.author.map(|x| {
    //                 Box::new(models::CommitAuthor {
    //                     name: x.name,
    //                     email: x.email,
    //                     date: x.date,
    //                 })
    //             }),
    //             committer: commit.committer.map(|x| {
    //                 Box::new(models::CommitAuthor {
    //                     name: x.name,
    //                     email: x.email,
    //                     date: x.date,
    //                 })
    //             }),
    //             parents: commit
    //                 .parents
    //                 .unwrap_or_default()
    //                 .into_iter()
    //                 .map(models::CommitParentsInner::from)
    //                 .collect(),
    //             stats: None, // Default to None
    //             files: None, // Default to None
    //         }
    //     }
    // }
}

mod _impl_github {
    use external_github;

    impl From<external_github::models::Commit> for super::Commit {
        fn from(commit: external_github::models::Commit) -> Self {
            Self::new(
                commit.url,
                commit.sha,
                commit.node_id,
                commit.html_url,
                commit.comments_url,
                (*commit.commit).into(),
                commit.author.map(|x| (*x).into()),
                commit.committer.map(|x| (*x).into()),
                commit.parents.into_iter().map(Into::into).collect(),
            )
        }
    }

    impl From<super::Commit> for external_github::models::Commit {
        fn from(commit: super::Commit) -> Self {
            external_github::models::Commit::new(
                commit.url,
                commit.sha,
                commit.node_id,
                commit.html_url,
                commit.comments_url,
                (*commit.commit).into(),
                commit.author.map(|x| (*x).into()),
                commit.committer.map(|x| (*x).into()),
                commit.parents.into_iter().map(Into::into).collect(),
            )
        }
    }
}
