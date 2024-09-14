use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use serde::Serialize;
pub async fn health_check() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(HealthResponse {
            version: env!("CARGO_PKG_VERSION").to_string(),
            name: env!("CARGO_PKG_NAME").to_string(),
            description: env!("CARGO_PKG_DESCRIPTION").to_string(),
        }),
    )
}

#[derive(Serialize)]
struct HealthResponse {
    version: String,
    name: String,
    description: String,
}

