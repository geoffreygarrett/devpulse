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
pub struct CreateAssistantRequestToolResourcesFileSearch {
    /// The [vector store](/docs/api-reference/vector-stores/object) attached to this assistant. There can be a maximum of 1 vector store attached to the assistant.
    #[serde(rename = "vector_store_ids", skip_serializing_if = "Option::is_none")]
    pub vector_store_ids: Option<Vec<String>>,
    /// A helper to create a [vector store](/docs/api-reference/vector-stores/object) with file_ids and attach it to this assistant. There can be a maximum of 1 vector store attached to the assistant.
    #[serde(rename = "vector_stores", skip_serializing_if = "Option::is_none")]
    pub vector_stores:
        Option<Vec<models::CreateAssistantRequestToolResourcesFileSearchVectorStoresInner>>,
}

impl CreateAssistantRequestToolResourcesFileSearch {
    pub fn new() -> CreateAssistantRequestToolResourcesFileSearch {
        CreateAssistantRequestToolResourcesFileSearch {
            vector_store_ids: None,
            vector_stores: None,
        }
    }

    pub fn builder() -> CreateAssistantRequestToolResourcesFileSearchBuilder {
        CreateAssistantRequestToolResourcesFileSearchBuilder::create_empty()
    }
}