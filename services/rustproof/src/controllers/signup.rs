use crate::models::api::signup::SignupParams;
use crate::services::AuthService;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use std::sync::Arc;

// Axum handler for the signup route
pub async fn signup(
    Extension(auth_service): Extension<Arc<dyn AuthService + Send + Sync>>,
    Json(input): Json<SignupParams>,
) -> impl IntoResponse {
    match auth_service.signup(input).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(err) => err.into_response(),
    }
}
