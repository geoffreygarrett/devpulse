/*
 * Based off of GitHub's API at the time of writing.
 */

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// NullableGitUser : Metaproperties for Git author/committer information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct GitUser {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

impl GitUser {
    /// Metaproperties for Git author/committer information.
    pub fn new() -> GitUser {
        GitUser {
            name: None,
            email: None,
            date: None,
        }
    }

    pub fn builder() -> GitUserBuilder {
        GitUserBuilder::create_empty()
    }
}

mod _impl_github {
    use external_github;

    impl From<external_github::models::NullableGitUser> for super::GitUser {
        fn from(x: external_github::models::NullableGitUser) -> Self {
            Self {
                name: x.name,
                email: x.email,
                date: x.date,
            }
        }
    }
    impl From<super::GitUser> for external_github::models::NullableGitUser {
        fn from(x: super::GitUser) -> Self {
            Self {
                name: x.name,
                email: x.email,
                date: x.date,
            }
        }
    }

    impl From<Box<external_github::models::NullableGitUser>> for super::GitUser {
        fn from(x: Box<external_github::models::NullableGitUser>) -> Self {
            Self {
                name: x.name,
                email: x.email,
                date: x.date,
            }
        }
    }
}
