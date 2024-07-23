use axum::{Extension, response::IntoResponse};
use utoipa::ToResponse;

use crate::models::{HealthCheckResponse, ServerState};

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, response = HealthCheckResponse),
    ),
    tag = "General"
)]
pub async fn health_check(Extension(state): Extension<ServerState>) -> impl IntoResponse {
    let uptime = state.get_uptime_as_secs().await;
    HealthCheckResponse::new("ok", uptime).into_response()
}
