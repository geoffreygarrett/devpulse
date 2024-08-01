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

/// Commit : Commit
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, Builder)]
#[builder(setter(strip_option, into), default)]
pub struct Commit {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "sha")]
    pub sha: String,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "commit")]
    pub commit: Box<models::CommitCommit>,
    #[serde(rename = "author", deserialize_with = "Option::deserialize")]
    pub author: Option<Box<models::CommitAuthor>>,
    #[serde(rename = "committer", deserialize_with = "Option::deserialize")]
    pub committer: Option<Box<models::CommitAuthor>>,
    #[serde(rename = "parents")]
    pub parents: Vec<models::CommitParentsInner>,
    #[serde(rename = "stats", skip_serializing_if = "Option::is_none")]
    pub stats: Option<Box<models::CommitStats>>,
    #[serde(rename = "files", skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<models::DiffEntry>>,
}

impl Commit {
    /// Commit
    pub fn new(
        url: String, sha: String, node_id: String, html_url: String, comments_url: String,
        commit: models::CommitCommit, author: Option<models::CommitAuthor>,
        committer: Option<models::CommitAuthor>, parents: Vec<models::CommitParentsInner>,
    ) -> Commit {
        Commit {
            url,
            sha,
            node_id,
            html_url,
            comments_url,
            commit: Box::new(commit),
            author: if let Some(x) = author {
                Some(Box::new(x))
            } else {
                None
            },
            committer: if let Some(x) = committer {
                Some(Box::new(x))
            } else {
                None
            },
            parents,
            stats: None,
            files: None,
        }
    }

    pub fn builder() -> CommitBuilder {
        CommitBuilder::create_empty()
    }
}