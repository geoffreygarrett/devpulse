use crate::config::AppConfig;
use crate::errors::ServiceError;
use axum::extract::{Extension, Json};
use axum::response::IntoResponse;
use bcrypt::verify;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String,
}

pub async fn login_handler(
    Json(payload): Json<LoginRequest>,
    Extension(config): Extension<Arc<AppConfig>>,
) -> Result<impl IntoResponse, ServiceError> {
    if payload.username == config.admin_username && verify(&payload.password, &config.admin_password).unwrap() {
        let expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize
            + 60 * 60; // 1 hour

        let claims = Claims {
            sub: payload.username,
            exp: expiration,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(config.jwt_secret.as_ref()),
        )
            .unwrap();

        Ok(axum::Json(LoginResponse { token }))
    } else {
        Err(ServiceError::AuthError)
    }
}
