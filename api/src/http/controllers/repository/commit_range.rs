use axum::{extract::Query, http::StatusCode, Json, response::IntoResponse};
use axum_xml_up::Xml;
use axum_yaml::Yaml;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use devpulse_core::services::analyze_commit_range_service;

use crate::errors::DevPulseError;
use crate::models::{
    BadRequest, CommitRangeRequest, InternalServerError, ResponseDetail, ResponseFormat,
    TooManyRequests, Unauthorized,
};

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[into_params(style = Form, parameter_in = Query)]
pub struct ResponseFormatQuery {
    format: Option<ResponseFormat>, // Optional. The response format: json, xml, yaml. Default is json.
    detail: Option<ResponseDetail>, // Optional. The level of detail: simple, detailed. Default is simple.
}

impl Default for ResponseFormatQuery {
    fn default() -> Self {
        Self {
            format: Some(ResponseFormat::Json),
            detail: Some(ResponseDetail::Simple),
        }
    }
}

/// Commit Range
#[utoipa::path(
    post,
    path = crate::http::COMMIT_RANGE_PATH,
    operation_id = "create_commit_range_analysis",
    params(
        ResponseFormatQuery
    ),
    responses(
        (status = 200, description = "Commit range analysis",
            content(
                ("application/vnd.devpulse.v1+json" = CommitRangeAnalysis),
                ("application/vnd.devpulse.v1+xml" = CommitRangeAnalysis),
                ("application/vnd.devpulse.v1+yaml" = CommitRangeAnalysis),
            )
        ),
        (status = 400, response = BadRequest),
        (status = 401, response = Unauthorized),
        (status = 429, response = TooManyRequests),
        (status = 500, response = InternalServerError),
    ),
    request_body(
        content = CommitRangeRequest,
        description = "The repository URL and commit range to analyze",
        content_type = "application/json"
    ),
    tag = crate::http::TAG_REPOSITORY_ANALYSIS,
)]
pub async fn create_commit_range_analysis(
    params: Option<Query<ResponseFormatQuery>>, Json(payload): Json<CommitRangeRequest>,
) -> impl IntoResponse {
    let params = params.unwrap_or_default();
    match analyze_commit_range_service(
        &payload.repository_url,
        &payload.start_commit,
        &payload.end_commit,
    )
    .await
    {
        Ok(response) => match params.format {
            Some(ResponseFormat::Xml) => Ok((StatusCode::OK, Xml(response)).into_response()),
            Some(ResponseFormat::Yaml) => Ok((StatusCode::OK, Yaml(response)).into_response()),
            _ => Ok((StatusCode::OK, Json(response)).into_response()),
        },
        Err(err) => Err(DevPulseError::InternalServerError(format!(
            "Failed to analyze commit range: {}",
            err
        ))),
    }
}
