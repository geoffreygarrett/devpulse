use axum::{
    Json, extract::Query, response::IntoResponse,
    http::StatusCode,
};
use axum::response::Response;
use axum_xml_up::Xml;
use axum_yaml::Yaml;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use crate::errors::DevPulseError;
use devpulse_core::services::analyze_commit_range_service;
use crate::models::{ResponseFormat, ResponseDetail, CommitRangeRequest, TooManyRequests};

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[into_params(style = Form, parameter_in = Query)]
pub struct ResponseFormatQuery {
    format: Option<ResponseFormat>,  // Optional. The response format: json, xml, yaml. Default is json.
    detail: Option<ResponseDetail>,  // Optional. The level of detail: simple, detailed. Default is simple.
}

impl Default for ResponseFormatQuery {
    fn default() -> Self {
        Self {
            format: Some(ResponseFormat::Json),
            detail: Some(ResponseDetail::Simple),
        }
    }
}

/// Repository Commit Range Analysis
///
/// # Request Body
/// * `CommitRangeRequest` - Contains the repository URL and the specific start and end commits to analyze.
///
/// # Query Parameters
/// * `format` - Optional. The response format (json, xml, yaml). Default is json.
/// * `detail` - Optional. The level of detail (simple, detailed). Default is simple.
///
/// # Responses
/// * `200 OK` - Returns the results of the commit range analysis in the specified format.
/// * `400 Bad Request` - Occurs if the request parameters are invalid.
/// * `401 Unauthorized` - Occurs if the API access is unauthorized.
/// * `429 Too Many Requests` - Occurs if the rate limit is exceeded.
///
/// # Examples
/// * Successful request in XML format:
///   ```bash
///   curl -X POST "http://localhost:3000/repositories/commit-range?format=xml" \
///   -H "Content-Type: application/json" \
///   -d '{
///         "repository_url": "https://github.com/example/repo",
///         "start_commit": "abc123",
///         "end_commit": "def456"
///       }'
///   ```
#[utoipa::path(
    post,
    path = crate::http::COMMIT_RANGE_PATH,
    operation_id = "create_commit_range_analysis",
    params(
    ResponseFormatQuery,
    ),
    responses(
    (status = 200, description = "Successful analysis of commit range",
    content(
    ("application/json" = CommitRangeResponse),
    ("application/xml" = CommitRangeResponse),
    ("application/yaml" = CommitRangeResponse)
    )
    ),
    (status = 400, description = "Bad request parameters"),
    (status = 401, description = "Unauthorized access"),
    (status = 429, response = TooManyRequests),
    ),
    request_body(content = CommitRangeRequest, description = "The repository URL and commit range to analyze", content_type = "application/json"),
    tag = crate::http::TAG_REPOSITORY_ANALYSIS,
)]
pub async fn create_commit_range_analysis(
    params: Option<Query<ResponseFormatQuery>>,
    Json(payload): Json<CommitRangeRequest>,
) -> Result<Response, DevPulseError> {
    let params = params.unwrap_or_default();
    match analyze_commit_range_service(
        &payload.repository_url,
        &payload.start_commit,
        &payload.end_commit).await {
        Ok(response) => match params.format {
            Some(ResponseFormat::Xml) => Ok(
                (StatusCode::OK, Xml(response)).into_response(),
            ),
            Some(ResponseFormat::Yaml) => Ok(
                (StatusCode::OK, Yaml(response)).into_response(),
            ),
            _ => Ok(
                (StatusCode::OK, Json(response)).into_response(),
            ),
        },
        Err(err) => Err(DevPulseError::InternalServerError(format!("Failed to analyze commit range: {}", err))),
    }
}
