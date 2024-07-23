use axum::response::IntoResponse;
use utoipa::OpenApi;

use crate::models::NotImplemented;

/// Create Analysis
///
/// TODO: Implement this endpoint
#[utoipa::path(
    post,
    path = "/pull-request",
    responses(
        (status = 501, response = NotImplemented),
    ),
    tag = "Pull Request"
)]
pub async fn create_pull_request_analysis() -> impl IntoResponse {
    NotImplemented::new("The pull request analysis endpoint is not implemented yet")
}
