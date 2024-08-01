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

/// IdentityRef :
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct IdentityRef {
    /// Deprecated - Can be retrieved by querying the Graph user referenced in the \"self\" entry of the IdentityRef \"_links\" dictionary
    #[serde(rename = "directoryAlias", skip_serializing_if = "Option::is_none")]
    pub directory_alias: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Deprecated - Available in the \"avatar\" entry of the IdentityRef \"_links\" dictionary
    #[serde(rename = "imageUrl", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// Deprecated - Can be retrieved by querying the Graph membership state referenced in the \"membershipState\" entry of the GraphUser \"_links\" dictionary
    #[serde(rename = "inactive", skip_serializing_if = "Option::is_none")]
    pub inactive: Option<bool>,
    /// Deprecated - Can be inferred from the subject type of the descriptor (Descriptor.IsAadUserType/Descriptor.IsAadGroupType)
    #[serde(rename = "isAadIdentity", skip_serializing_if = "Option::is_none")]
    pub is_aad_identity: Option<bool>,
    /// Deprecated - Can be inferred from the subject type of the descriptor (Descriptor.IsGroupType)
    #[serde(rename = "isContainer", skip_serializing_if = "Option::is_none")]
    pub is_container: Option<bool>,
    #[serde(rename = "isDeletedInOrigin", skip_serializing_if = "Option::is_none")]
    pub is_deleted_in_origin: Option<bool>,
    /// Deprecated - not in use in most preexisting implementations of ToIdentityRef
    #[serde(rename = "profileUrl", skip_serializing_if = "Option::is_none")]
    pub profile_url: Option<String>,
    /// Deprecated - use Domain+PrincipalName instead
    #[serde(rename = "uniqueName", skip_serializing_if = "Option::is_none")]
    pub unique_name: Option<String>,
    #[serde(rename = "_links", skip_serializing_if = "Option::is_none")]
    pub _links: Option<Box<models::ReferenceLinks>>,
    /// The descriptor is the primary way to reference the graph subject while the system is running. This field will uniquely identify the same graph subject across both Accounts and Organizations.
    #[serde(rename = "descriptor", skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<String>,
    /// This is the non-unique display name of the graph subject. To change this field, you must alter its value in the source provider.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// This url is the full route to the source resource of this graph subject.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl IdentityRef {
    ///
    pub fn new() -> IdentityRef {
        IdentityRef {
            directory_alias: None,
            id: None,
            image_url: None,
            inactive: None,
            is_aad_identity: None,
            is_container: None,
            is_deleted_in_origin: None,
            profile_url: None,
            unique_name: None,
            _links: None,
            descriptor: None,
            display_name: None,
            url: None,
        }
    }

    pub fn builder() -> IdentityRefBuilder {
        IdentityRefBuilder::create_empty()
    }
}