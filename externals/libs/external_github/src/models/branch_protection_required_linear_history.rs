/*
 * GitHub v3 REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.1.4
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder, Eq)]
#[builder(setter(strip_option, into), default)]
pub struct BranchProtectionRequiredLinearHistory {
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl BranchProtectionRequiredLinearHistory {
    pub fn new() -> BranchProtectionRequiredLinearHistory {
        BranchProtectionRequiredLinearHistory { enabled: None }
    }

    pub fn builder() -> BranchProtectionRequiredLinearHistoryBuilder {
        BranchProtectionRequiredLinearHistoryBuilder::create_empty()
    }
}