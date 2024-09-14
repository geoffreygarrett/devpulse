/*
 * GitHub v3 REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.1.4
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct PullRequestSimpleHead {
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "ref")]
    pub r#ref: String,
    #[serde(rename = "repo")]
    pub repo: Box<models::Repository>,
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "user", deserialize_with = "Option::deserialize")]
    pub user: Option<Box<models::NullableSimpleUser>>,
}

impl PullRequestSimpleHead {
    pub fn new(
        label: String, r#ref: String, repo: models::Repository, sha: String,
        user: Option<models::NullableSimpleUser>,
    ) -> PullRequestSimpleHead {
        PullRequestSimpleHead {
            label,
            r#ref,
            repo: Box::new(repo),
            sha,
            user: if let Some(x) = user {
                Some(Box::new(x))
            } else {
                None
            },
        }
    }

    pub fn builder() -> PullRequestSimpleHeadBuilder {
        PullRequestSimpleHeadBuilder::create_empty()
    }
}