use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::{Extension, Json};
use axum::extract::FromRequestParts;
use axum::http::{request::Parts, StatusCode};
use axum::response::IntoResponse;
use axum::TypedHeader;
use axum_extra::headers::{Authorization, authorization::Bearer};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::auth::jwt::{Claims, JwtService};
use crate::core::auth::user_service::UserService;
use crate::core::errors::ServiceError;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String,
}

/// Handler for user login.
pub async fn login_handler(
    Json(payload): Json<LoginRequest>, Extension(user_service): Extension<UserService>,
    Extension(jwt_service): Extension<JwtService>,
) -> Result<impl IntoResponse, ServiceError> {
    let user_id = user_service.verify_user(&payload.username, &payload.password)?;
    let token = jwt_service.generate_token(&user_id.to_string(), "key1")?;
    Ok(axum::Json(LoginResponse { token }))
}

/// Extracts JWT claims from the request.
#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = ServiceError;

    async fn from_request_parts(
        parts: &mut Parts, Extension(jwt_service): Extension<JwtService>,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| ServiceError::AuthError)?;
        let claims = jwt_service.validate_token(bearer.token())?;
        Ok(claims)
    }
}
