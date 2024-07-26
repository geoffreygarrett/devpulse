use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::models;

/// CommitStatus : The CommitStatus of a commit.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct CommitStatus {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "avatar_url", deserialize_with = "Option::deserialize")]
    pub avatar_url: Option<String>,
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "state")]
    pub state: String,
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    #[serde(rename = "target_url", deserialize_with = "Option::deserialize")]
    pub target_url: Option<String>,
    #[serde(rename = "context")]
    pub context: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "creator", deserialize_with = "Option::deserialize")]
    pub creator: Option<Box<models::SimpleUser>>,
}

impl CommitStatus {
    /// The CommitStatus of a commit.
    pub fn new(
        url: String, avatar_url: Option<String>, id: i32, node_id: String, state: String,
        description: Option<String>, target_url: Option<String>, context: String,
        created_at: String, updated_at: String, creator: Option<models::SimpleUser>,
    ) -> CommitStatus {
        CommitStatus {
            url,
            avatar_url,
            id,
            node_id,
            state,
            description,
            target_url,
            context,
            created_at,
            updated_at,
            creator: if let Some(x) = creator {
                Some(Box::new(x))
            } else {
                None
            },
        }
    }

    pub fn builder() -> CommitStatusBuilder {
        CommitStatusBuilder::create_empty()
    }
}

mod _impl_github {
    use external_github;

    impl From<external_github::models::Status> for super::CommitStatus {
        fn from(internal: external_github::models::Status) -> Self {
            super::CommitStatus {
                url: internal.url,
                avatar_url: internal.avatar_url,
                id: internal.id,
                node_id: internal.node_id,
                state: internal.state,
                description: internal.description,
                target_url: internal.target_url,
                context: internal.context,
                created_at: internal.created_at,
                updated_at: internal.updated_at,
                creator: internal.creator.map(|x| Box::new((*x).into())),
            }
        }
    }

    impl From<super::CommitStatus> for external_github::models::Status {
        fn from(external: super::CommitStatus) -> Self {
            external_github::models::Status {
                url: external.url,
                avatar_url: external.avatar_url,
                id: external.id,
                node_id: external.node_id,
                state: external.state,
                description: external.description,
                target_url: external.target_url,
                context: external.context,
                created_at: external.created_at,
                updated_at: external.updated_at,
                creator: external.creator.map(|x| Box::new((*x).into())),
            }
        }
    }
}
