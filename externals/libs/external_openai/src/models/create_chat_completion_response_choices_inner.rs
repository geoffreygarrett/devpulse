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
pub struct CreateChatCompletionResponseChoicesInner {
    /// The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence, `length` if the maximum number of tokens specified in the request was reached, `content_filter` if content was omitted due to a flag from our content filters, `tool_
    #[serde(rename = "finish_reason")]
    pub finish_reason: FinishReason,
    /// The index of the choice in the list of choices.
    #[serde(rename = "index")]
    pub index: i32,
    #[serde(rename = "message")]
    pub message: Box<models::ChatCompletionResponseMessage>,
    #[serde(rename = "logprobs", deserialize_with = "Option::deserialize")]
    pub logprobs: Option<Box<models::CreateChatCompletionResponseChoicesInnerLogprobs>>,
}

impl CreateChatCompletionResponseChoicesInner {
    pub fn new(
        finish_reason: FinishReason, index: i32, message: models::ChatCompletionResponseMessage,
        logprobs: Option<models::CreateChatCompletionResponseChoicesInnerLogprobs>,
    ) -> CreateChatCompletionResponseChoicesInner {
        CreateChatCompletionResponseChoicesInner {
            finish_reason,
            index,
            message: Box::new(message),
            logprobs: if let Some(x) = logprobs {
                Some(Box::new(x))
            } else {
                None
            },
        }
    }

    pub fn builder() -> CreateChatCompletionResponseChoicesInnerBuilder {
        CreateChatCompletionResponseChoicesInnerBuilder::create_empty()
    }
}
/// The reason the model stopped generating tokens. This will be `stop` if the model hit a natural stop point or a provided stop sequence, `length` if the maximum number of tokens specified in the request was reached, `content_filter` if content was omitted due to a flag from our content filters, `tool_
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FinishReason {
    #[serde(rename = "stop")]
    Stop,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "tool_calls")]
    ToolCalls,
    #[serde(rename = "content_filter")]
    ContentFilter,
    #[serde(rename = "function_call")]
    FunctionCall,
}

impl Default for FinishReason {
    fn default() -> FinishReason {
        Self::Stop
    }
}