use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use utoipa::OpenApi;

use crate::http::api_doc::API_DOC;

/// YAML
#[utoipa::path(
    get,
    path = crate::http::openapi_yaml_path(),
    responses(
    (status = 200, description = "Returns the OpenAPI YAML documentation",
    content(
    ("application/yaml" = String),
    ))
    ),
    tag = crate::http::TAG_API_DOCS
)]
pub async fn get_openapi_yaml() -> impl IntoResponse {
    let doc = API_DOC.clone();
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/yaml")
        .body(doc.to_yaml().unwrap())
        .unwrap()
}

/// JSON
#[allow(unused)]
#[utoipa::path(
    get,
    path = crate::http::openapi_json_path(),
    responses(
    (status = 200, description = "Returns the OpenAPI JSON documentation",
    content(
    ("application/json" = String),
    )
    ),
    (status = 500, description = "Internal server error", body = DevPulseError),
    ),
    tag = crate::http::TAG_API_DOCS
)]
pub async fn get_openapi_json() -> impl IntoResponse {
    let doc = API_DOC.clone();
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(doc.to_json().unwrap())
        .unwrap()
}
