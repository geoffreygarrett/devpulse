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
pub struct AssistantToolsFunction {
    /// The type of tool being defined: `function`
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "function")]
    pub function: Box<models::FunctionObject>,
}

impl AssistantToolsFunction {
    pub fn new(r#type: Type, function: models::FunctionObject) -> AssistantToolsFunction {
        AssistantToolsFunction {
            r#type,
            function: Box::new(function),
        }
    }

    pub fn builder() -> AssistantToolsFunctionBuilder {
        AssistantToolsFunctionBuilder::create_empty()
    }
}
/// The type of tool being defined: `function`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "function")]
    Function,
}

impl Default for Type {
    fn default() -> Type {
        Self::Function
    }
}