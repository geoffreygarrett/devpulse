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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder, Eq)]
#[builder(setter(strip_option, into), default)]
pub struct ListModelsResponse {
    #[serde(rename = "object")]
    pub object: Object,
    #[serde(rename = "data")]
    pub data: Vec<models::ApiModel>,
}

impl ListModelsResponse {
    pub fn new(object: Object, data: Vec<models::ApiModel>) -> ListModelsResponse {
        ListModelsResponse { object, data }
    }

    pub fn builder() -> ListModelsResponseBuilder {
        ListModelsResponseBuilder::create_empty()
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
    #[serde(rename = "list")]
    List,
}

impl Default for Object {
    fn default() -> Object {
        Self::List
    }
}