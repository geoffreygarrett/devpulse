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

/// CreateAssistantRequestApiModel : ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models/overview) for descriptions of them.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder, Eq)]
#[builder(setter(strip_option, into), default)]
pub struct CreateAssistantRequestApiModel {}

impl CreateAssistantRequestApiModel {
    /// ID of the model to use. You can use the [List models](/docs/api-reference/models/list) API to see all of your available models, or see our [Model overview](/docs/models/overview) for descriptions of them.
    pub fn new() -> CreateAssistantRequestApiModel {
        CreateAssistantRequestApiModel {}
    }

    pub fn builder() -> CreateAssistantRequestApiModelBuilder {
        CreateAssistantRequestApiModelBuilder::create_empty()
    }
}