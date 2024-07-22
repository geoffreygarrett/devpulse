use axum::{response::IntoResponse};
use axum::http::StatusCode;
use axum::response::Response;
use utoipa::{OpenApi};
use super::super::*;

/// Provides the OpenAPI documentation for the API in YAML format.
///
/// # Responses
/// * `200 OK` - Returns the OpenAPI YAML documentation.
/// * `500 Internal Server Error` - Occurs if there is an issue generating the documentation.
#[utoipa::path(
    get,
    path = openapi_yaml_path(),
    responses(
    (status = 200, description = "Returns the OpenAPI YAML documentation",
    content(
    ("application/yaml" = String),
    ))
    ),
    tag = TAG_API_DOCS
)]
pub async fn get_openapi_yaml() -> impl IntoResponse {
    let doc = API_DOC.clone();
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/yaml")
        .body(doc.to_yaml().unwrap())
        .unwrap()
}

/// Provides the OpenAPI documentation for the API in JSON format.
///
/// # Responses
/// * `200 OK` - Returns the OpenAPI JSON documentation.
/// * `500 Internal Server Error` - Occurs if there is an issue generating the documentation.
#[allow(unused)]
#[utoipa::path(
    get,
    path = openapi_json_path(),
    responses(
    (status = 200, description = "Returns the OpenAPI JSON documentation",
    content(
    ("application/json" = String),
    )
    ),
    (status = 500, description = "Internal server error", body = DevPulseError),
    ),
    tag = TAG_API_DOCS
)]
pub async fn get_openapi_json() -> impl IntoResponse {
    let doc = API_DOC.clone();
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(doc.to_json().unwrap())
        .unwrap()
}