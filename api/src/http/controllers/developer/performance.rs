use axum::{Json, extract::Path, response::IntoResponse};
use axum::http::StatusCode;
use crate::errors::DevPulseError;
use crate::models::{DeveloperPerformance, RepositoryContribution};

/// Retrieves performance metrics for a specified developer.
///
/// This endpoint returns a set of performance metrics for the developer identified by `username`.
/// The performance metrics include total commits, total pull requests, average time to merge PRs,
/// and a list of repository contributions.
///
/// # Parameters
/// * `username` (path parameter) - The username of the developer whose performance metrics are being requested.
///
/// # Responses
/// * `200 OK` - Returns the detailed performance metrics of the specified developer.
/// * `404 Not Found` - Indicates that the developer with the specified username does not exist.
/// * `401 Unauthorized` - Indicates that the request lacks valid authentication credentials for the requested resource.
///
/// # Examples
/// * Successful request:
///   ```bash
///   curl -X GET "http://localhost:3000/developers/johndoe/performance" -H "Authorization: Bearer {token}"
///   ```
/// * Unauthorized request:
///   ```bash
///   curl -X GET "http://localhost:3000/developers/johndoe/performance"
///   ```
#[utoipa::path(
    get,
    path = "/developers/{username}/performance",
    responses(
    (status = 200, description = "Successful retrieval of developer performance", body = DeveloperPerformance),
    (status = 404, description = "Developer not found", body = DevPulseError),
    (status = 401, description = "Unauthorized access", body = DevPulseError),
    ),
    params(
    ("username" = String, Path, description = "Username of the developer to retrieve performance for")
    ),
    tag = crate::http::TAG_DEVELOPER_ANALYSIS,
)]
pub async fn get_developer_performance(Path(username): Path<String>) -> impl IntoResponse {
    // Mock-up example of a performance object.
    // In a real application, you would fetch these details from a database or external service.
    if username != "johndoe" {
        return DevPulseError::DeveloperNotFound.into_response();
    }
    (
        StatusCode::OK,
        Json(DeveloperPerformance {
            username,
            total_commits: 120,
            total_prs: 30,
            average_time_to_merge: "48 hours".to_string(),
            repositories: vec![RepositoryContribution {
                url: "https://example.com/repo".to_string(),
                commits: 100,
            }],
        })).into_response()
}
