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

/// RunObjectLastError : The last error associated with this run. Will be `null` if there are no errors.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder, Eq)]
#[builder(setter(strip_option, into), default)]
pub struct RunObjectLastError {
    /// One of `server_error`, `rate_limit_exceeded`, or `invalid_prompt`.
    #[serde(rename = "code")]
    pub code: Code,
    /// A human-readable description of the error.
    #[serde(rename = "message")]
    pub message: String,
}

impl RunObjectLastError {
    /// The last error associated with this run. Will be `null` if there are no errors.
    pub fn new(code: Code, message: String) -> RunObjectLastError {
        RunObjectLastError { code, message }
    }

    pub fn builder() -> RunObjectLastErrorBuilder {
        RunObjectLastErrorBuilder::create_empty()
    }
}
/// One of `server_error`, `rate_limit_exceeded`, or `invalid_prompt`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Code {
    #[serde(rename = "server_error")]
    ServerError,
    #[serde(rename = "rate_limit_exceeded")]
    RateLimitExceeded,
    #[serde(rename = "invalid_prompt")]
    InvalidPrompt,
}

impl Default for Code {
    fn default() -> Code {
        Self::ServerError
    }
}