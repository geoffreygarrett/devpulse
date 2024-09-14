use crate::errors::AuthError;
use crate::services::{AuthService, AuthorizationCodeGrant, ClientCredentialsGrant, PasswordGrant, RefreshTokenGrant};
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantType {
    AuthorizationCode,
    RefreshToken,
    Password,
}

#[derive(Deserialize)]
pub struct TokenParams {
    grant_type: GrantType,
}


#[derive(Deserialize)]
#[serde(untagged)]
pub enum TokenRequest {
    AuthorizationCode(AuthorizationCodeGrant),
    RefreshToken(RefreshTokenGrant),
    Password(PasswordGrant),
    ClientCredentials(ClientCredentialsGrant),

}

#[derive(Debug)]
pub enum TokenError {
    InvalidRequest(String),
    Unauthorized(String),
    InternalServerError(String),
}

impl IntoResponse for TokenError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            TokenError::InvalidRequest(message) => (
                StatusCode::BAD_REQUEST,
                json!({ "error": "invalid_request", "message": message }),
            ),
            TokenError::Unauthorized(message) => (
                StatusCode::UNAUTHORIZED,
                json!({ "error": "unauthorized", "message": message }),
            ),
            TokenError::InternalServerError(message) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "error": "internal_error", "message": message }),
            ),
        };


        let mut response = (status, Json(body)).into_response();
        response
            .headers_mut()
            .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
        response
    }
}

pub async fn token(
    Extension(auth_service): Extension<Arc<dyn AuthService + Send + Sync>>,
    Json(payload): Json<TokenRequest>,
) -> Result<Response, AuthError> {
    // Match the payload to the correct grant type
    let response = match payload {
        TokenRequest::AuthorizationCode(grant) => {
            auth_service
                .handle_authorization_code_grant(grant)
                .await?
        }
        TokenRequest::RefreshToken(grant) => {
            auth_service
                .handle_refresh_token_grant(grant)
                .await?
        }
        TokenRequest::Password(grant) => {
            auth_service
                .handle_password_grant(grant)
                .await?
        }
        TokenRequest::ClientCredentials(grant) => {
            auth_service
                .handle_client_credentials_grant(grant)
                .await?
        }
    };

    // Prepare the response and set the no-store cache control header
    let mut response = Json(response).into_response();
    response.headers_mut().insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("no-store"),
    );
    Ok(response)
}

async fn handle_authorization_code_grant(
    code: String,
    redirect_uri: String,
) -> Result<String, TokenError> {
    if code.is_empty() || redirect_uri.is_empty() {
        return Err(TokenError::InvalidRequest(
            "Code or redirect URI cannot be empty".to_string(),
        ));
    }

    Ok(format!(
        "Authorization code grant processed with code={} and redirect_uri={}",
        code, redirect_uri
    ))
}

async fn handle_refresh_token_grant(refresh_token: String) -> Result<String, TokenError> {
    if refresh_token.is_empty() {
        return Err(TokenError::InvalidRequest(
            "Refresh token cannot be empty".to_string(),
        ));
    }

    Ok(format!(
        "Refresh token grant processed with refresh_token={}",
        refresh_token
    ))
}

async fn handle_password_grant(email: String, password: String) -> Result<String, TokenError> {
    if email.is_empty() || password.is_empty() {
        return Err(TokenError::InvalidRequest(
            "Email or password cannot be empty".to_string(),
        ));
    }

    Ok(format!(
        "Password grant processed with email={} and password=*****",
        email
    ))
}
