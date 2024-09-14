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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateMessageRequestAttachmentsInnerToolsInner {
    AssistantToolsCode(Box<models::AssistantToolsCode>),
    AssistantToolsFileSearchTypeOnly(Box<models::AssistantToolsFileSearchTypeOnly>),
}

impl Default for CreateMessageRequestAttachmentsInnerToolsInner {
    fn default() -> Self {
        Self::AssistantToolsCode(Default::default())
    }
}
/// The type of tool being defined: `code_interpreter`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "code_interpreter")]
    CodeInterpreter,
    #[serde(rename = "file_search")]
    FileSearch,
}

impl Default for Type {
    fn default() -> Type {
        Self::CodeInterpreter
    }
}