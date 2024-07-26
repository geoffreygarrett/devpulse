/*
 *
 */

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct CommitParentsInner {
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "html_url", skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}

impl CommitParentsInner {
    pub fn new(sha: String, url: String) -> CommitParentsInner {
        CommitParentsInner {
            sha,
            url,
            html_url: None,
        }
    }

    pub fn builder() -> CommitParentsInnerBuilder {
        CommitParentsInnerBuilder::create_empty()
    }
}

mod _impl_github {
    use external_github;

    use super::*;

    impl From<external_github::models::CommitParentsInner> for CommitParentsInner {
        fn from(internal: external_github::models::CommitParentsInner) -> Self {
            Self {
                sha: internal.sha,
                url: internal.url,
                html_url: internal.html_url,
            }
        }
    }

    impl From<CommitParentsInner> for external_github::models::CommitParentsInner {
        fn from(internal: CommitParentsInner) -> Self {
            Self {
                sha: internal.sha,
                url: internal.url,
                html_url: internal.html_url,
            }
        }
    }
}
