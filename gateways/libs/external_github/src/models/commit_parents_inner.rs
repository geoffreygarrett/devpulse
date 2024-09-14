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