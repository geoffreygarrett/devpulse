use axum::response::IntoResponse;
use utoipa::ToResponse;

use crate::models::SourceVersionResponse;

/// Source Version
///
/// This endpoint provides the version of the API source code.
/// It can be useful for clients to determine the exact version
/// of the API they are interacting with, especially for debugging
/// and compatibility purposes.
#[auto_route::route(
    get,
    path = "/version",
    responses(
        (status = 200, response = SourceVersionResponse)
    ),
    tag = "General"
)]
pub async fn version() -> impl IntoResponse {
    SourceVersionResponse::new(env!("CARGO_PKG_VERSION")).into_response()
}
