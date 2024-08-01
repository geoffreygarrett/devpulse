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
pub struct AssistantToolsFileSearchTypeOnly {
    /// The type of tool being defined: `file_search`
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl AssistantToolsFileSearchTypeOnly {
    pub fn new(r#type: Type) -> AssistantToolsFileSearchTypeOnly {
        AssistantToolsFileSearchTypeOnly { r#type }
    }

    pub fn builder() -> AssistantToolsFileSearchTypeOnlyBuilder {
        AssistantToolsFileSearchTypeOnlyBuilder::create_empty()
    }
}
/// The type of tool being defined: `file_search`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "file_search")]
    FileSearch,
}

impl Default for Type {
    fn default() -> Type {
        Self::FileSearch
    }
}