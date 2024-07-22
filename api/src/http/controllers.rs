use axum::http::StatusCode;
use axum::response::IntoResponse;

pub(crate) async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}