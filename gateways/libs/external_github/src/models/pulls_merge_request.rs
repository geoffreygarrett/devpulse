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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder, Eq)]
#[builder(setter(strip_option, into), default)]
pub struct PullsMergeRequest {
    /// Title for the automatic commit message.
    #[serde(rename = "commit_title", skip_serializing_if = "Option::is_none")]
    pub commit_title: Option<String>,
    /// Extra detail to append to automatic commit message.
    #[serde(rename = "commit_message", skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    /// SHA that pull request head must match to allow merge.
    #[serde(rename = "sha", skip_serializing_if = "Option::is_none")]
    pub sha: Option<String>,
    /// The merge method to use.
    #[serde(rename = "merge_method", skip_serializing_if = "Option::is_none")]
    pub merge_method: Option<MergeMethod>,
}

impl PullsMergeRequest {
    pub fn new() -> PullsMergeRequest {
        PullsMergeRequest {
            commit_title: None,
            commit_message: None,
            sha: None,
            merge_method: None,
        }
    }

    pub fn builder() -> PullsMergeRequestBuilder {
        PullsMergeRequestBuilder::create_empty()
    }
}
/// The merge method to use.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MergeMethod {
    #[serde(rename = "merge")]
    Merge,
    #[serde(rename = "squash")]
    Squash,
    #[serde(rename = "rebase")]
    Rebase,
}

impl Default for MergeMethod {
    fn default() -> MergeMethod {
        Self::Merge
    }
}