use crate::services::{OAuthError, OAuthService};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Extension;
use std::sync::Arc;

// Axum route handler to initiate authorization
pub async fn authorize(
    Query(redirect_to): Query<Option<String>>,
    Extension(oauth_service): Extension<Arc<OAuthService>>,
) -> Result<impl IntoResponse, OAuthError> {
    let provider_name = redirect_to.unwrap_or_else(|| "google".to_string()); // Default to Google
    let provider = oauth_service
        .get_provider(&provider_name)
        .ok_or_else(|| OAuthError {
            message: format!("Provider {} not found", provider_name),
        })?;

    let auth_url = provider.authorize().await?;
    let mut response = (StatusCode::FOUND, [("Location", auth_url)]).into_response();
    response
        .headers_mut()
        .insert(axum::http::header::CACHE_CONTROL, axum::http::HeaderValue::from_static("no-store"));
    Ok(response)
}

