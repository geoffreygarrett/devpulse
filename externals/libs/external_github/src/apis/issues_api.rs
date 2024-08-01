/*
 * GitHub v3 REST API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.1.4
 *
 * Generated by: https://openapi-generator.tech
 */

use super::configuration;
pub use super::Error;
use crate::{apis::ResponseContent, models};
use cached::proc_macro::cached;
use cached::Cached;
use cached::SizedCache;
use derive_builder::Builder;
use reqwest;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, trace, warn};

#[macro_export]
macro_rules! generate_issues_api_client_methods {
    () => {
        /// issues_slash_create_comment method
        pub async fn issues_slash_create_comment(
            &self, params: IssuesSlashCreateCommentParams,
        ) -> Result<models::IssueComment, Error<IssuesSlashCreateCommentError>> {
            let result =
                crate::apis::issues_api::issues_slash_create_comment(&self.config, params).await?;
            Ok(result)
        }
        /// issues_slash_delete_comment method
        pub async fn issues_slash_delete_comment(
            &self, params: IssuesSlashDeleteCommentParams,
        ) -> Result<(), Error<IssuesSlashDeleteCommentError>> {
            let result =
                crate::apis::issues_api::issues_slash_delete_comment(&self.config, params).await?;
            Ok(result)
        }
        /// issues_slash_list_comments method
        pub async fn issues_slash_list_comments(
            &self, params: IssuesSlashListCommentsParams,
        ) -> Result<Vec<models::IssueComment>, Error<IssuesSlashListCommentsError>> {
            let result =
                crate::apis::issues_api::issues_slash_list_comments(&self.config, params).await?;
            Ok(result)
        }
        /// issues_slash_update_comment method
        pub async fn issues_slash_update_comment(
            &self, params: IssuesSlashUpdateCommentParams,
        ) -> Result<models::IssueComment, Error<IssuesSlashUpdateCommentError>> {
            let result =
                crate::apis::issues_api::issues_slash_update_comment(&self.config, params).await?;
            Ok(result)
        }
    };
}

#[macro_export]
macro_rules! generate_issues_api_client {
() => {
    use std::sync::Arc;
    use cached::SizedCache;
    use reqwest::Client;

    pub struct IssuesApi<'a> {
        config: &'a Arc<configuration::Configuration>,
        client: Client,
    }

    impl<'a> IssuesApi<'a> {
        pub fn new(config: &'a Arc<configuration::Configuration>) -> Self {
            IssuesApi {
                config,
                client: Client::new(),
            }
        }
        /// issues_slash_create_comment method
        pub async  fn create_comment(&self, params: IssuesSlashCreateCommentParams) ->
            Result<models::IssueComment, Error<IssuesSlashCreateCommentError>> {
            let result = crate::apis::issues_api::issues_slash_create_comment(&self.config, params).await?;
            Ok(result)
        }
        /// issues_slash_delete_comment method
        pub async  fn delete_comment(&self, params: IssuesSlashDeleteCommentParams) ->
            Result<, Error<IssuesSlashDeleteCommentError>> {
            let result = crate::apis::issues_api::issues_slash_delete_comment(&self.config, params).await?;
            Ok(result)
        }
        /// issues_slash_list_comments method
        pub async  fn list_comments(&self, params: IssuesSlashListCommentsParams) ->
            Result<Vec<models::IssueComment>, Error<IssuesSlashListCommentsError>> {
            let result = crate::apis::issues_api::issues_slash_list_comments(&self.config, params).await?;
            Ok(result)
        }
        /// issues_slash_update_comment method
        pub async  fn update_comment(&self, params: IssuesSlashUpdateCommentParams) ->
            Result<models::IssueComment, Error<IssuesSlashUpdateCommentError>> {
            let result = crate::apis::issues_api::issues_slash_update_comment(&self.config, params).await?;
            Ok(result)
        }
    }
}
}

/// struct for passing parameters to the method [`issues_slash_create_comment`]
#[derive(Clone, Debug, Builder, Default, Serialize, PartialEq, Eq)]
#[builder(setter(strip_option, into), default)]
pub struct IssuesSlashCreateCommentParams {
    /// The account owner of the repository. The name is not case sensitive.
    pub owner: String,
    /// The name of the repository without the `.git` extension. The name is not case sensitive.
    pub repo: String,
    /// The number that identifies the issue.
    pub issue_number: i32,
    pub issues_create_comment_request: models::IssuesCreateCommentRequest,
}

impl IssuesSlashCreateCommentParams {
    pub fn builder() -> IssuesSlashCreateCommentParamsBuilder {
        IssuesSlashCreateCommentParamsBuilder::default()
    }
}

/// struct for passing parameters to the method [`issues_slash_delete_comment`]
#[derive(Clone, Debug, Builder, Default, Serialize, PartialEq, Eq)]
#[builder(setter(strip_option, into), default)]
pub struct IssuesSlashDeleteCommentParams {
    /// The account owner of the repository. The name is not case sensitive.
    pub owner: String,
    /// The name of the repository without the `.git` extension. The name is not case sensitive.
    pub repo: String,
    /// The unique identifier of the comment.
    pub comment_id: i32,
}

impl IssuesSlashDeleteCommentParams {
    pub fn builder() -> IssuesSlashDeleteCommentParamsBuilder {
        IssuesSlashDeleteCommentParamsBuilder::default()
    }
}

/// struct for passing parameters to the method [`issues_slash_list_comments`]
#[derive(Clone, Debug, Builder, Default, Serialize, PartialEq, Eq)]
#[builder(setter(strip_option, into), default)]
pub struct IssuesSlashListCommentsParams {
    /// The account owner of the repository. The name is not case sensitive.
    pub owner: String,
    /// The name of the repository without the `.git` extension. The name is not case sensitive.
    pub repo: String,
    /// The number that identifies the issue.
    pub issue_number: i32,
    /// Only show results that were last updated after the given time. This is a timestamp in [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format: `YYYY-MM-DDTHH:MM:SSZ`.
    pub since: Option<String>,
    /// The number of results per page (max 100). For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\"
    pub per_page: Option<i32>,
    /// The page number of the results to fetch. For more information, see \"[Using pagination in the REST API](https://docs.github.com/rest/using-the-rest-api/using-pagination-in-the-rest-api).\"
    pub page: Option<i32>,
}

impl IssuesSlashListCommentsParams {
    pub fn builder() -> IssuesSlashListCommentsParamsBuilder {
        IssuesSlashListCommentsParamsBuilder::default()
    }
}

/// struct for passing parameters to the method [`issues_slash_update_comment`]
#[derive(Clone, Debug, Builder, Default, Serialize, PartialEq, Eq)]
#[builder(setter(strip_option, into), default)]
pub struct IssuesSlashUpdateCommentParams {
    /// The account owner of the repository. The name is not case sensitive.
    pub owner: String,
    /// The name of the repository without the `.git` extension. The name is not case sensitive.
    pub repo: String,
    /// The unique identifier of the comment.
    pub comment_id: i32,
    pub issues_create_comment_request: models::IssuesCreateCommentRequest,
}

impl IssuesSlashUpdateCommentParams {
    pub fn builder() -> IssuesSlashUpdateCommentParamsBuilder {
        IssuesSlashUpdateCommentParamsBuilder::default()
    }
}

/// struct for typed errors of method [`issues_slash_create_comment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IssuesSlashCreateCommentError {
    Status403(models::BasicError),
    Status404(models::BasicError),
    Status410(models::BasicError),
    Status422(models::ValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`issues_slash_delete_comment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IssuesSlashDeleteCommentError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`issues_slash_list_comments`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IssuesSlashListCommentsError {
    Status404(models::BasicError),
    Status410(models::BasicError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`issues_slash_update_comment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IssuesSlashUpdateCommentError {
    Status422(models::ValidationError),
    UnknownValue(serde_json::Value),
}

/// You can use the REST API to create comments on issues and pull requests. Every pull request is an issue, but not every issue is a pull request.  This endpoint triggers [notifications](https://docs.github.com/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating con
#[tracing::instrument]
pub async fn issues_slash_create_comment(
    configuration: &configuration::Configuration, params: IssuesSlashCreateCommentParams,
) -> Result<models::IssueComment, Error<IssuesSlashCreateCommentError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let owner = params.owner;
    let repo = params.repo;
    let issue_number = params.issue_number;
    let issues_create_comment_request = params.issues_create_comment_request;

    debug!("Calling issues_slash_create_comment...");

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/repos/{owner}/{repo}/issues/{issue_number}/comments",
        local_var_configuration.base_path,
        owner = crate::apis::urlencode(owner),
        repo = crate::apis::urlencode(repo),
        issue_number = issue_number
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&issues_create_comment_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IssuesSlashCreateCommentError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// You can use the REST API to delete comments on issues and pull requests. Every pull request is an issue, but not every issue is a pull request.
#[tracing::instrument]
pub async fn issues_slash_delete_comment(
    configuration: &configuration::Configuration, params: IssuesSlashDeleteCommentParams,
) -> Result<(), Error<IssuesSlashDeleteCommentError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let owner = params.owner;
    let repo = params.repo;
    let comment_id = params.comment_id;

    debug!("Calling issues_slash_delete_comment...");

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/repos/{owner}/{repo}/issues/comments/{comment_id}",
        local_var_configuration.base_path,
        owner = crate::apis::urlencode(owner),
        repo = crate::apis::urlencode(repo),
        comment_id = comment_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<IssuesSlashDeleteCommentError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// You can use the REST API to list comments on issues and pull requests. Every pull request is an issue, but not every issue is a pull request.  Issue comments are ordered by ascending ID.  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.g
#[tracing::instrument]
pub async fn issues_slash_list_comments(
    configuration: &configuration::Configuration, params: IssuesSlashListCommentsParams,
) -> Result<Vec<models::IssueComment>, Error<IssuesSlashListCommentsError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let owner = params.owner;
    let repo = params.repo;
    let issue_number = params.issue_number;
    let since = params.since;
    let per_page = params.per_page;
    let page = params.page;

    debug!("Calling issues_slash_list_comments...");

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/repos/{owner}/{repo}/issues/{issue_number}/comments",
        local_var_configuration.base_path,
        owner = crate::apis::urlencode(owner),
        repo = crate::apis::urlencode(repo),
        issue_number = issue_number
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = since {
        local_var_req_builder =
            local_var_req_builder.query(&[("since", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = per_page {
        local_var_req_builder =
            local_var_req_builder.query(&[("per_page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder =
            local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IssuesSlashListCommentsError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// You can use the REST API to update comments on issues and pull requests. Every pull request is an issue, but not every issue is a pull request.  This endpoint supports the following custom media types. For more information, see \"[Media types](https://docs.github.com/rest/using-the-rest-api/getting-s
#[tracing::instrument]
pub async fn issues_slash_update_comment(
    configuration: &configuration::Configuration, params: IssuesSlashUpdateCommentParams,
) -> Result<models::IssueComment, Error<IssuesSlashUpdateCommentError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let owner = params.owner;
    let repo = params.repo;
    let comment_id = params.comment_id;
    let issues_create_comment_request = params.issues_create_comment_request;

    debug!("Calling issues_slash_update_comment...");

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/repos/{owner}/{repo}/issues/comments/{comment_id}",
        local_var_configuration.base_path,
        owner = crate::apis::urlencode(owner),
        repo = crate::apis::urlencode(repo),
        comment_id = comment_id
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&issues_create_comment_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<IssuesSlashUpdateCommentError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}