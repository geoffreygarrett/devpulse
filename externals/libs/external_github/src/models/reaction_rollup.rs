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
pub struct ReactionRollup {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "total_count")]
    pub total_count: i32,
    #[serde(rename = "+1")]
    pub plus_1: i32,
    #[serde(rename = "-1")]
    pub _1: i32,
    #[serde(rename = "laugh")]
    pub laugh: i32,
    #[serde(rename = "confused")]
    pub confused: i32,
    #[serde(rename = "heart")]
    pub heart: i32,
    #[serde(rename = "hooray")]
    pub hooray: i32,
    #[serde(rename = "eyes")]
    pub eyes: i32,
    #[serde(rename = "rocket")]
    pub rocket: i32,
}

impl ReactionRollup {
    pub fn new(
        url: String, total_count: i32, plus_1: i32, _1: i32, laugh: i32, confused: i32, heart: i32,
        hooray: i32, eyes: i32, rocket: i32,
    ) -> ReactionRollup {
        ReactionRollup {
            url,
            total_count,
            plus_1,
            _1,
            laugh,
            confused,
            heart,
            hooray,
            eyes,
            rocket,
        }
    }

    pub fn builder() -> ReactionRollupBuilder {
        ReactionRollupBuilder::create_empty()
    }
}