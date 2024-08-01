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

/// NullableLicenseSimple : License Simple
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct NullableLicenseSimple {
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "url", deserialize_with = "Option::deserialize")]
    pub url: Option<String>,
    #[serde(rename = "spdx_id", deserialize_with = "Option::deserialize")]
    pub spdx_id: Option<String>,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "html_url", skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
}

impl NullableLicenseSimple {
    /// License Simple
    pub fn new(
        key: String, name: String, url: Option<String>, spdx_id: Option<String>, node_id: String,
    ) -> NullableLicenseSimple {
        NullableLicenseSimple {
            key,
            name,
            url,
            spdx_id,
            node_id,
            html_url: None,
        }
    }

    pub fn builder() -> NullableLicenseSimpleBuilder {
        NullableLicenseSimpleBuilder::create_empty()
    }
}