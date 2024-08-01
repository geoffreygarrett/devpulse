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

/// MessageContentTextAnnotationsFileCitationObject : A citation within the message that points to a specific quote from a specific File associated with the assistant or the message. Generated when the assistant uses the \"file_search\" tool to search files.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder, Eq)]
#[builder(setter(strip_option, into), default)]
pub struct MessageContentTextAnnotationsFileCitationObject {
    /// Always `file_citation`.
    #[serde(rename = "type")]
    pub r#type: Type,
    /// The text in the message content that needs to be replaced.
    #[serde(rename = "text")]
    pub text: String,
    #[serde(rename = "file_citation")]
    pub file_citation: Box<models::MessageContentTextAnnotationsFileCitationObjectFileCitation>,
    #[serde(rename = "start_index")]
    pub start_index: i32,
    #[serde(rename = "end_index")]
    pub end_index: i32,
}

impl MessageContentTextAnnotationsFileCitationObject {
    /// A citation within the message that points to a specific quote from a specific File associated with the assistant or the message. Generated when the assistant uses the \"file_search\" tool to search files.
    pub fn new(
        r#type: Type, text: String,
        file_citation: models::MessageContentTextAnnotationsFileCitationObjectFileCitation,
        start_index: i32, end_index: i32,
    ) -> MessageContentTextAnnotationsFileCitationObject {
        MessageContentTextAnnotationsFileCitationObject {
            r#type,
            text,
            file_citation: Box::new(file_citation),
            start_index,
            end_index,
        }
    }

    pub fn builder() -> MessageContentTextAnnotationsFileCitationObjectBuilder {
        MessageContentTextAnnotationsFileCitationObjectBuilder::create_empty()
    }
}
/// Always `file_citation`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "file_citation")]
    FileCitation,
}

impl Default for Type {
    fn default() -> Type {
        Self::FileCitation
    }
}