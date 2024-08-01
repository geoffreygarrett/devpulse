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

/// VectorStoreFileObjectLastError : The last error associated with this vector store file. Will be `null` if there are no errors.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder, Eq)]
#[builder(setter(strip_option, into), default)]
pub struct VectorStoreFileObjectLastError {
    /// One of `server_error` or `rate_limit_exceeded`.
    #[serde(rename = "code")]
    pub code: Code,
    /// A human-readable description of the error.
    #[serde(rename = "message")]
    pub message: String,
}

impl VectorStoreFileObjectLastError {
    /// The last error associated with this vector store file. Will be `null` if there are no errors.
    pub fn new(code: Code, message: String) -> VectorStoreFileObjectLastError {
        VectorStoreFileObjectLastError { code, message }
    }

    pub fn builder() -> VectorStoreFileObjectLastErrorBuilder {
        VectorStoreFileObjectLastErrorBuilder::create_empty()
    }
}
/// One of `server_error` or `rate_limit_exceeded`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Code {
    #[serde(rename = "internal_error")]
    InternalError,
    #[serde(rename = "file_not_found")]
    FileNotFound,
    #[serde(rename = "parsing_error")]
    ParsingError,
    #[serde(rename = "unhandled_mime_type")]
    UnhandledMimeType,
}

impl Default for Code {
    fn default() -> Code {
        Self::InternalError
    }
}