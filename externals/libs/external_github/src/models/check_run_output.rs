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
pub struct CheckRunOutput {
    #[serde(rename = "title", deserialize_with = "Option::deserialize")]
    pub title: Option<String>,
    #[serde(rename = "summary", deserialize_with = "Option::deserialize")]
    pub summary: Option<String>,
    #[serde(rename = "text", deserialize_with = "Option::deserialize")]
    pub text: Option<String>,
    #[serde(rename = "annotations_count")]
    pub annotations_count: i32,
    #[serde(rename = "annotations_url")]
    pub annotations_url: String,
}

impl CheckRunOutput {
    pub fn new(
        title: Option<String>, summary: Option<String>, text: Option<String>,
        annotations_count: i32, annotations_url: String,
    ) -> CheckRunOutput {
        CheckRunOutput {
            title,
            summary,
            text,
            annotations_count,
            annotations_url,
        }
    }

    pub fn builder() -> CheckRunOutputBuilder {
        CheckRunOutputBuilder::create_empty()
    }
}