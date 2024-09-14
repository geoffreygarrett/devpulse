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
pub struct PullRequestMinimal {
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "number")]
    pub number: i32,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "head")]
    pub head: Box<models::PullRequestMinimalHead>,
    #[serde(rename = "base")]
    pub base: Box<models::PullRequestMinimalHead>,
}

impl PullRequestMinimal {
    pub fn new(
        id: i64, number: i32, url: String, head: models::PullRequestMinimalHead,
        base: models::PullRequestMinimalHead,
    ) -> PullRequestMinimal {
        PullRequestMinimal {
            id,
            number,
            url,
            head: Box::new(head),
            base: Box::new(base),
        }
    }

    pub fn builder() -> PullRequestMinimalBuilder {
        PullRequestMinimalBuilder::create_empty()
    }
}