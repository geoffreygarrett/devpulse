use axum::{extract::Path, response::IntoResponse};
use utoipa::ToResponse;

use crate::models::NotImplemented;

/// Performance Metrics
///
/// This endpoint returns a set of performance metrics for the developer identified by `username`.
/// The performance metrics include total commits, total pull requests, average time to merge PRs,
/// and a list of repository contributions.
///
/// TODO: Implement this endpoint
#[utoipa::path(
    get,
    path = "/developers/{username}/performance",
    responses(
        // (status = 200, response = DeveloperPerformanceAnalysisResponse),
        // (status = 404, description = "Developer not found", body = DevPulseError),
        // (status = 401, response = Unauthorized),
        // (status = 429, response = TooManyRequests),
        (status = 500, response = NotImplemented)
    ),
    params(
    ("username" = String, Path, description = "Username of the developer to retrieve performance for")
    ),
    tag = crate::http::TAG_DEVELOPER_ANALYSIS,
)]
pub async fn get_developer_performance(Path(username): Path<String>) -> impl IntoResponse {
    NotImplemented::new("The developer performance analysis endpoint is not implemented yet")
}
