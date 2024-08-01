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

/// UploadPart : The upload Part represents a chunk of bytes we can add to an Upload object.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder, Eq)]
#[builder(setter(strip_option, into), default)]
pub struct UploadPart {
    /// The upload Part unique identifier, which can be referenced in API endpoints.
    #[serde(rename = "id")]
    pub id: String,
    /// The Unix timestamp (in seconds) for when the Part was created.
    #[serde(rename = "created_at")]
    pub created_at: i32,
    /// The ID of the Upload object that this Part was added to.
    #[serde(rename = "upload_id")]
    pub upload_id: String,
    /// The object type, which is always `upload.part`.
    #[serde(rename = "object")]
    pub object: Object,
}

impl UploadPart {
    /// The upload Part represents a chunk of bytes we can add to an Upload object.
    pub fn new(id: String, created_at: i32, upload_id: String, object: Object) -> UploadPart {
        UploadPart {
            id,
            created_at,
            upload_id,
            object,
        }
    }

    pub fn builder() -> UploadPartBuilder {
        UploadPartBuilder::create_empty()
    }
}
/// The object type, which is always `upload.part`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
    #[serde(rename = "upload.part")]
    UploadPeriodPart,
}

impl Default for Object {
    fn default() -> Object {
        Self::UploadPeriodPart
    }
}