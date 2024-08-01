/*
 * OpenAI API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 2.1.0
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
macro_rules! generate_files_api_client_methods {
    () => {
        /// create_file method
        pub async fn create_file(
            &self, params: CreateFileParams,
        ) -> Result<models::OpenAiFile, Error<CreateFileError>> {
            let result = crate::apis::files_api::create_file(&self.config, params).await?;
            Ok(result)
        }
        /// delete_file method
        pub async fn delete_file(
            &self, params: DeleteFileParams,
        ) -> Result<models::DeleteFileResponse, Error<DeleteFileError>> {
            let result = crate::apis::files_api::delete_file(&self.config, params).await?;
            Ok(result)
        }
        /// download_file method
        pub async fn download_file(
            &self, params: DownloadFileParams,
        ) -> Result<String, Error<DownloadFileError>> {
            let result = crate::apis::files_api::download_file(&self.config, params).await?;
            Ok(result)
        }
        /// list_files method
        pub async fn list_files(
            &self, params: ListFilesParams,
        ) -> Result<models::ListFilesResponse, Error<ListFilesError>> {
            let result = crate::apis::files_api::list_files(&self.config, params).await?;
            Ok(result)
        }
        /// retrieve_file method
        pub async fn retrieve_file(
            &self, params: RetrieveFileParams,
        ) -> Result<models::OpenAiFile, Error<RetrieveFileError>> {
            let result = crate::apis::files_api::retrieve_file(&self.config, params).await?;
            Ok(result)
        }
    };
}

#[macro_export]
macro_rules! generate_files_api_client {
    () => {
        use cached::SizedCache;
        use reqwest::Client;
        use std::sync::Arc;

        pub struct FilesApi<'a> {
            config: &'a Arc<configuration::Configuration>,
            client: Client,
        }

        impl<'a> FilesApi<'a> {
            pub fn new(config: &'a Arc<configuration::Configuration>) -> Self {
                FilesApi {
                    config,
                    client: Client::new(),
                }
            }
            /// create_file method
            pub async fn createfile(
                &self, params: CreateFileParams,
            ) -> Result<models::OpenAiFile, Error<CreateFileError>> {
                let result = crate::apis::files_api::create_file(&self.config, params).await?;
                Ok(result)
            }
            /// delete_file method
            pub async fn deletefile(
                &self, params: DeleteFileParams,
            ) -> Result<models::DeleteFileResponse, Error<DeleteFileError>> {
                let result = crate::apis::files_api::delete_file(&self.config, params).await?;
                Ok(result)
            }
            /// download_file method
            pub async fn downloadfile(
                &self, params: DownloadFileParams,
            ) -> Result<String, Error<DownloadFileError>> {
                let result = crate::apis::files_api::download_file(&self.config, params).await?;
                Ok(result)
            }
            /// list_files method
            pub async fn listfiles(
                &self, params: ListFilesParams,
            ) -> Result<models::ListFilesResponse, Error<ListFilesError>> {
                let result = crate::apis::files_api::list_files(&self.config, params).await?;
                Ok(result)
            }
            /// retrieve_file method
            pub async fn retrievefile(
                &self, params: RetrieveFileParams,
            ) -> Result<models::OpenAiFile, Error<RetrieveFileError>> {
                let result = crate::apis::files_api::retrieve_file(&self.config, params).await?;
                Ok(result)
            }
        }
    };
}

/// struct for passing parameters to the method [`create_file`]
#[derive(Clone, Debug, Builder, Default, Serialize, PartialEq)]
#[builder(setter(strip_option, into), default)]
pub struct CreateFileParams {
    /// The File object (not file name) to be uploaded.
    pub file: std::path::PathBuf,
    /// The intended purpose of the uploaded file.  Use \\\"assistants\\\" for [Assistants](/docs/api-reference/assistants) and [Message](/docs/api-reference/messages) files, \\\"vision\\\" for Assistants image file inputs, \\\"batch\\\" for [Batch API](/docs/guides/batch), and \\\"fine-tune\\\" for [Fine-tuning](/docs/api-referen
    pub purpose: String,
}

impl CreateFileParams {
    pub fn builder() -> CreateFileParamsBuilder {
        CreateFileParamsBuilder::default()
    }
}

/// struct for passing parameters to the method [`delete_file`]
#[derive(Clone, Debug, Builder, Default, Serialize, PartialEq)]
#[builder(setter(strip_option, into), default)]
pub struct DeleteFileParams {
    /// The ID of the file to use for this request.
    pub file_id: String,
}

impl DeleteFileParams {
    pub fn builder() -> DeleteFileParamsBuilder {
        DeleteFileParamsBuilder::default()
    }
}

/// struct for passing parameters to the method [`download_file`]
#[derive(Clone, Debug, Builder, Default, Serialize, PartialEq)]
#[builder(setter(strip_option, into), default)]
pub struct DownloadFileParams {
    /// The ID of the file to use for this request.
    pub file_id: String,
}

impl DownloadFileParams {
    pub fn builder() -> DownloadFileParamsBuilder {
        DownloadFileParamsBuilder::default()
    }
}

/// struct for passing parameters to the method [`list_files`]
#[derive(Clone, Debug, Builder, Default, Serialize, PartialEq)]
#[builder(setter(strip_option, into), default)]
pub struct ListFilesParams {
    /// Only return files with the given purpose.
    pub purpose: Option<String>,
}

impl ListFilesParams {
    pub fn builder() -> ListFilesParamsBuilder {
        ListFilesParamsBuilder::default()
    }
}

/// struct for passing parameters to the method [`retrieve_file`]
#[derive(Clone, Debug, Builder, Default, Serialize, PartialEq)]
#[builder(setter(strip_option, into), default)]
pub struct RetrieveFileParams {
    /// The ID of the file to use for this request.
    pub file_id: String,
}

impl RetrieveFileParams {
    pub fn builder() -> RetrieveFileParamsBuilder {
        RetrieveFileParamsBuilder::default()
    }
}

/// struct for typed errors of method [`create_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateFileError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteFileError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`download_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DownloadFileError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_files`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListFilesError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`retrieve_file`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RetrieveFileError {
    UnknownValue(serde_json::Value),
}

#[tracing::instrument]
pub async fn create_file(
    configuration: &configuration::Configuration, params: CreateFileParams,
) -> Result<models::OpenAiFile, Error<CreateFileError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let file = params.file;
    let purpose = params.purpose;

    debug!("Calling create_file...");

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/files", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };
    let mut local_var_form = reqwest::multipart::Form::new();
    // TODO: support file upload for 'file' parameter
    local_var_form = local_var_form.text("purpose", purpose.to_string());
    local_var_req_builder = local_var_req_builder.multipart(local_var_form);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateFileError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

#[tracing::instrument]
pub async fn delete_file(
    configuration: &configuration::Configuration, params: DeleteFileParams,
) -> Result<models::DeleteFileResponse, Error<DeleteFileError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let file_id = params.file_id;

    debug!("Calling delete_file...");

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/files/{file_id}",
        local_var_configuration.base_path,
        file_id = crate::apis::urlencode(file_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteFileError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

#[tracing::instrument]
pub async fn download_file(
    configuration: &configuration::Configuration, params: DownloadFileParams,
) -> Result<String, Error<DownloadFileError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let file_id = params.file_id;

    debug!("Calling download_file...");

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/files/{file_id}/content",
        local_var_configuration.base_path,
        file_id = crate::apis::urlencode(file_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DownloadFileError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

#[tracing::instrument]
pub async fn list_files(
    configuration: &configuration::Configuration, params: ListFilesParams,
) -> Result<models::ListFilesResponse, Error<ListFilesError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let purpose = params.purpose;

    debug!("Calling list_files...");

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/files", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = purpose {
        local_var_req_builder =
            local_var_req_builder.query(&[("purpose", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListFilesError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

#[tracing::instrument]
pub async fn retrieve_file(
    configuration: &configuration::Configuration, params: RetrieveFileParams,
) -> Result<models::OpenAiFile, Error<RetrieveFileError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let file_id = params.file_id;

    debug!("Calling retrieve_file...");

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/files/{file_id}",
        local_var_configuration.base_path,
        file_id = crate::apis::urlencode(file_id)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_token) = local_var_configuration.bearer_access_token {
        local_var_req_builder = local_var_req_builder.bearer_auth(local_var_token.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RetrieveFileError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}