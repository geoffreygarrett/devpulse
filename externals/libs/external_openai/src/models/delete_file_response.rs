/*
 * OpenAI API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct DeleteFileResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "object")]
    pub object: Object,
    #[serde(rename = "deleted")]
    pub deleted: bool,
}

impl DeleteFileResponse {
    pub fn new(id: String, object: Object, deleted: bool) -> DeleteFileResponse {
        DeleteFileResponse {
            id,
            object,
            deleted,
        }
    }

    pub fn builder() -> DeleteFileResponseBuilder {
        DeleteFileResponseBuilder::create_empty()
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
    #[serde(rename = "file")]
    File,
}

impl Default for Object {
    fn default() -> Object {
        Self::File
    }
}