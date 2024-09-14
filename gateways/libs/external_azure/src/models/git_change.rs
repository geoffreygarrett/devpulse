/*
 * Git
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 7.1.0
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// GitChange :
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct GitChange {
    /// ID of the change within the group of changes.
    #[serde(rename = "changeId", skip_serializing_if = "Option::is_none")]
    pub change_id: Option<i32>,
    #[serde(rename = "newContentTemplate", skip_serializing_if = "Option::is_none")]
    pub new_content_template: Option<Box<models::GitTemplate>>,
    /// Original path of item if different from current path.
    #[serde(rename = "originalPath", skip_serializing_if = "Option::is_none")]
    pub original_path: Option<String>,
    /// The type of change that was made to the item.
    #[serde(rename = "changeType", skip_serializing_if = "Option::is_none")]
    pub change_type: Option<ChangeType>,
    /// Current version.
    #[serde(rename = "item", skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
    #[serde(rename = "newContent", skip_serializing_if = "Option::is_none")]
    pub new_content: Option<Box<models::ItemContent>>,
    /// Path of the item on the server.
    #[serde(rename = "sourceServerItem", skip_serializing_if = "Option::is_none")]
    pub source_server_item: Option<String>,
    /// URL to retrieve the item.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl GitChange {
    ///
    pub fn new() -> GitChange {
        GitChange {
            change_id: None,
            new_content_template: None,
            original_path: None,
            change_type: None,
            item: None,
            new_content: None,
            source_server_item: None,
            url: None,
        }
    }

    pub fn builder() -> GitChangeBuilder {
        GitChangeBuilder::create_empty()
    }
}
/// The type of change that was made to the item.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ChangeType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "edit")]
    Edit,
    #[serde(rename = "encoding")]
    Encoding,
    #[serde(rename = "rename")]
    Rename,
    #[serde(rename = "delete")]
    Delete,
    #[serde(rename = "undelete")]
    Undelete,
    #[serde(rename = "branch")]
    Branch,
    #[serde(rename = "merge")]
    Merge,
    #[serde(rename = "lock")]
    Lock,
    #[serde(rename = "rollback")]
    Rollback,
    #[serde(rename = "sourceRename")]
    SourceRename,
    #[serde(rename = "targetRename")]
    TargetRename,
    #[serde(rename = "property")]
    Property,
    #[serde(rename = "all")]
    All,
}

impl Default for ChangeType {
    fn default() -> ChangeType {
        Self::None
    }
}