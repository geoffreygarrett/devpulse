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
pub struct PullRequestLabelsInner {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", deserialize_with = "Option::deserialize")]
    pub description: Option<String>,
    #[serde(rename = "color")]
    pub color: String,
    #[serde(rename = "default")]
    pub default: bool,
}

impl PullRequestLabelsInner {
    pub fn new(
        id: i64, node_id: String, url: String, name: String, description: Option<String>,
        color: String, default: bool,
    ) -> PullRequestLabelsInner {
        PullRequestLabelsInner {
            id,
            node_id,
            url,
            name,
            description,
            color,
            default,
        }
    }

    pub fn builder() -> PullRequestLabelsInnerBuilder {
        PullRequestLabelsInnerBuilder::create_empty()
    }
}