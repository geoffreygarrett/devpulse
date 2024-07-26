/*
 * Based off of GitHub's API at the time of writing.
 */

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct CommitCommit {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "author", deserialize_with = "Option::deserialize")]
    pub author: Option<Box<models::GitUser>>,
    #[serde(rename = "committer", deserialize_with = "Option::deserialize")]
    pub committer: Option<Box<models::GitUser>>,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "comment_count")]
    pub comment_count: i32,
    #[serde(rename = "tree")]
    pub tree: Box<models::CommitCommitTree>,
    #[serde(rename = "verification", skip_serializing_if = "Option::is_none")]
    pub verification: Option<Box<models::Verification>>,
}

impl CommitCommit {
    pub fn new(
        url: String, author: Option<models::GitUser>, committer: Option<models::GitUser>,
        message: String, comment_count: i32, tree: models::CommitCommitTree,
    ) -> CommitCommit {
        CommitCommit {
            url,
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
            message,
            comment_count,
            tree: Box::new(tree),
            verification: None,
        }
    }

    pub fn builder() -> CommitCommitBuilder {
        CommitCommitBuilder::create_empty()
    }
}

mod _impl_github {
    use external_github;

    use super::*;

    impl From<external_github::models::CommitCommit> for CommitCommit {
        fn from(external: external_github::models::CommitCommit) -> Self {
            Self::new(
                external.url,
                external.author.map(|x| x.into()),
                external.committer.map(|x| x.into()),
                external.message,
                external.comment_count,
                external.tree.into(),
            )
        }
    }

    impl From<CommitCommit> for external_github::models::CommitCommit {
        fn from(internal: CommitCommit) -> Self {
            external_github::models::CommitCommit::new(
                internal.url,
                internal.author.map(|x| (*x).into()),
                internal.committer.map(|x| (*x).into()),
                internal.message,
                internal.comment_count,
                (*internal.tree).into(),
            )
        }
    }
}
