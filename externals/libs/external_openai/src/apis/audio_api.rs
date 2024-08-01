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
macro_rules! generate_audio_api_client_methods {
    () => {
        /// create_translation method
        pub async fn create_translation(
            &self, params: CreateTranslationParams,
        ) -> Result<models::CreateTranslation200Response, Error<CreateTranslationError>> {
            let result = crate::apis::audio_api::create_translation(&self.config, params).await?;
            Ok(result)
        }
    };
}

#[macro_export]
macro_rules! generate_audio_api_client {
    () => {
        use cached::SizedCache;
        use reqwest::Client;
        use std::sync::Arc;

        pub struct AudioApi<'a> {
            config: &'a Arc<configuration::Configuration>,
            client: Client,
        }

        impl<'a> AudioApi<'a> {
            pub fn new(config: &'a Arc<configuration::Configuration>) -> Self {
                AudioApi {
                    config,
                    client: Client::new(),
                }
            }
            /// create_translation method
            pub async fn createtranslation(
                &self, params: CreateTranslationParams,
            ) -> Result<models::CreateTranslation200Response, Error<CreateTranslationError>> {
                let result =
                    crate::apis::audio_api::create_translation(&self.config, params).await?;
                Ok(result)
            }
        }
    };
}

/// struct for passing parameters to the method [`create_translation`]
#[derive(Clone, Debug, Builder, Default, Serialize, PartialEq, Eq)]
#[builder(setter(strip_option, into), default)]
pub struct CreateTranslationParams {
    /// The audio file object (not file name) translate, in one of these formats: flac, mp3, mp4, mpeg, mpga, m4a, ogg, wav, or webm.
    pub file: std::path::PathBuf,
    pub model: models::CreateTranslationRequestModel,
    /// An optional text to guide the model's style or continue a previous audio segment. The [prompt](/docs/guides/speech-to-text/prompting) should be in English.
    pub prompt: Option<String>,
    /// The format of the transcript output, in one of these options: `json`, `text`, `srt`, `verbose_json`, or `vtt`.
    pub response_format: Option<String>,
    /// The sampling temperature, between 0 and 1. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. If set to 0, the model will use [log probability](https://en.wikipedia.org/wiki/Log_probability) to automatically increase the
    pub temperature: Option<f64>,
}

impl CreateTranslationParams {
    pub fn builder() -> CreateTranslationParamsBuilder {
        CreateTranslationParamsBuilder::default()
    }
}

/// struct for typed errors of method [`create_translation`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateTranslationError {
    UnknownValue(serde_json::Value),
}

#[tracing::instrument]
pub async fn create_translation(
    configuration: &configuration::Configuration, params: CreateTranslationParams,
) -> Result<models::CreateTranslation200Response, Error<CreateTranslationError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let file = params.file;
    let model = params.model;
    let prompt = params.prompt;
    let response_format = params.response_format;
    let temperature = params.temperature;

    debug!("Calling create_translation...");

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/audio/translations", local_var_configuration.base_path);
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
    local_var_form = local_var_form.text("model", model.to_string());
    if let Some(local_var_param_value) = prompt {
        local_var_form = local_var_form.text("prompt", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = response_format {
        local_var_form = local_var_form.text("response_format", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = temperature {
        local_var_form = local_var_form.text("temperature", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.multipart(local_var_form);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateTranslationError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}