use axum::http::HeaderMap;
use axum::{response::IntoResponse, Extension};
use utoipa::ToResponse;

use crate::accept::serialize_response;
use crate::models::{HealthCheck, HealthCheckResponse, ServerState};

/// Health Check
///
/// This endpoint is used to check the health status of the API server.
/// It provides information about the server's current status and uptime.
/// This can be useful for monitoring the server's health and ensuring
/// that it is running as expected.
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, response = HealthCheckResponse)
    ),
    tag = "General"
)]
pub async fn health_check(
    headers: HeaderMap, Extension(state): Extension<ServerState>,
) -> impl IntoResponse {
    let uptime = state.get_uptime_as_secs().await;
    serialize_response(&HealthCheck::new("ok", uptime), &headers)
}
