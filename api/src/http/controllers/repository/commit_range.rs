use axum::{Json};
use crate::errors::DevPulseError;
use crate::models::{CommitRangeRequest, CommitRangeResponse};
use devpulse_core::services::analyze_commit_range_service;

/// Analyzes a specified range of commits within a repository.
///
/// # Request Body
/// * `CommitRangeRequest` - Contains the repository URL and the specific start and end commits to analyze.
///
/// # Responses
/// * `200 OK` - Returns the results of the commit range analysis.
/// * `400 Bad Request` - Occurs if the request parameters are invalid.
/// * `401 Unauthorized` - Occurs if the API access is unauthorized.
///
/// # Examples
/// * Successful request:
///   ```bash
///   curl -X POST "http://localhost:3000/repositories/commit-range" \
///   -H "Content-Type: application/json" \
///   -d '{
///         "repository_url": "https://github.com/example/repo",
///         "start_commit": "abc123",
///         "end_commit": "def456"
///       }'
///   ```
/// * Unauthorized request:
///   ```bash
///   curl -X POST "http://localhost:3000/repositories/commit-range" \
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
    responses(
    (status = 200, description = "Successful analysis of commit range", body = CommitRangeResponse),
    (status = 400, description = "Bad request parameters"),
    (status = 401, description = "Unauthorized access")
    ),
    request_body = CommitRangeRequest,
    tag = crate::http::TAG_REPOSITORY_ANALYSIS,
)]
pub async fn create_commit_range_analysis(Json(payload): Json<CommitRangeRequest>) -> Result<Json<CommitRangeResponse>, DevPulseError> {
    match analyze_commit_range_service(&payload.repository_url, &payload.start_commit, &payload.end_commit).await {
        Ok(response) => Ok(Json(response.into())),
        Err(err) => Err(DevPulseError::InternalServerError(format!("Failed to analyze commit range: {}", err))),
    }
}
