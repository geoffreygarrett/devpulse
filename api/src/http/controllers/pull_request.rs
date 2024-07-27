use axum::response::IntoResponse;
use utoipa::OpenApi;

use crate::models::NotImplemented;
use crate::route;

/// Create Analysis
///
/// Create a new analysis for a pull request, if the analysis exists, it will be returned.
///
/// TODO: Implement this endpoint
#[route(
    put,
    path = "/pull-request",
    operation_id = "create_pull_request_analysis",
    responses(
        (status = 501, response = NotImplemented),
    ),
    tag = "Pull Request"
)]
pub async fn create_pull_request_analysis() -> impl IntoResponse {
    NotImplemented::new("The pull request analysis endpoint is not implemented yet")
}
