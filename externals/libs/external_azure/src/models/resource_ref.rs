/*
 * Git
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 7.1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// ResourceRef :
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct ResourceRef {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ResourceRef {
    ///
    pub fn new() -> ResourceRef {
        ResourceRef {
            id: None,
            url: None,
        }
    }

    pub fn builder() -> ResourceRefBuilder {
        ResourceRefBuilder::create_empty()
    }
}