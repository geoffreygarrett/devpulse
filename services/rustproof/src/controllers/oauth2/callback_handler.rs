use crate::services::{OAuthError, OAuthService};
use axum::extract::Query;
use axum::response::IntoResponse;
use axum::{Extension, Json};
use std::collections::HashMap;
use std::sync::Arc;


// Axum route handler to handle callback after authorization
pub async fn callback(
    Query(params): Query<HashMap<String, String>>,
    Extension(oauth_service): Extension<Arc<OAuthService>>,
) -> Result<impl IntoResponse, OAuthError> {
    let provider_name = params.get("state").unwrap();
    let provider = oauth_service
        .get_provider(provider_name)
        .ok_or_else(|| OAuthError {
            message: format!("Provider {} not found", provider_name),
        })?;

    let code = params
        .get("code")
        .ok_or_else(|| OAuthError {
            message: "Authorization code not provided".to_string(),
        })?;

    let token = provider.exchange_code_for_token(code).await?;
    Ok(Json(token))
}