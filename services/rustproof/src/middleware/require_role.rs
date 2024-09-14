use crate::prelude::RustProofClaims;
use crate::services::AccessTokenService;
use axum::http::request::Parts;
use axum::http::{Request, StatusCode};
use axum::{
    async_trait,
    extract::{Extension, FromRequestParts},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use std::sync::Arc;
use thiserror::Error;
use tracing::{debug, error, info, trace, warn, instrument};

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = match self {
            AuthError::Unauthorized => StatusCode::UNAUTHORIZED,
            AuthError::Forbidden => StatusCode::FORBIDDEN,
        };
        (status, Json(self.to_string())).into_response()
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for RustProofClaims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        trace!("Attempting to extract token service from request parts.");

        // Attempt to extract the token service from the request extensions
        let token_service = Extension::<Arc<dyn AccessTokenService + Send + Sync>>::from_request_parts(parts, state)
            .await
            .map_err(|_| {
                error!("Failed to extract token service from request parts.");
                AuthError::Unauthorized
            })?;

        trace!("Token service successfully extracted. Extracting token from request headers.");

        // Attempt to extract the token from the request headers
        let token = extract_token(parts).ok_or_else(|| {
            warn!("Authorization header missing or malformed.");
            AuthError::Unauthorized
        })?;

        debug!("Token successfully extracted: {:?}", token);

        // Attempt to validate the token and extract claims
        let claims = token_service.validate_access_token(&token).await.map_err(|err| {
            error!("Token validation failed: {:?}", err);
            AuthError::Unauthorized
        })?;

        debug!("Claims successfully extracted: {:?}", claims);

        Ok(claims)
    }
}

fn extract_token(parts: &axum::http::request::Parts) -> Option<String> {
    parts
        .headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| {
            if value.starts_with("Bearer ") {
                Some(value[7..].to_string())
            } else {
                Some(value.to_string())
            }
        })
}

#[instrument(skip(req, next))]
pub async fn extract_claims(
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let request_path = req.uri().path().to_string();
    info!(%request_path, "Starting to extract claims from the request.");

    let (mut parts, body) = req.into_parts();

    // Attempt to extract claims from the request parts
    let claims = match RustProofClaims::from_request_parts(&mut parts, &()).await {
        Ok(claims) => {
            info!(
                sub = %claims.subject.as_deref().unwrap_or("unknown"),
                "Successfully extracted claims from request."
            );
            debug!("Claims: {:?}", claims);
            Some(Arc::new(claims))
        }
        Err(_) => {
            warn!(request_path = %request_path, "Failed to extract claims from request. Proceeding without claims.");
            None
        }
    };

    // Reconstruct the request and store the claims in the extensions
    let mut req = Request::from_parts(parts, body);
    req.extensions_mut().insert(claims);

    // Proceed with the next middleware or handler
    next.run(req).await
}

#[instrument(skip(req, next, claim_check))]
pub async fn require_claim(
    mut req: Request<axum::body::Body>,
    next: Next,
    claim_check: impl Fn(&RustProofClaims) -> bool + Send + Sync + 'static,
) -> Result<Response, AuthError> {
    let (mut parts, body) = req.into_parts();

    let token_service = Extension::<Arc<dyn AccessTokenService + Send + Sync>>::from_request_parts(&mut parts, &())
        .await
        .map_err(|err| {
            error!("Failed to extract token service from request: {:?}", err);
            AuthError::Forbidden
        })?;

    let token = extract_token(&parts).ok_or_else(|| {
        error!("Token not found in request headers.");
        AuthError::Unauthorized
    })?;

    let claims = token_service.validate_access_token(&token).await.map_err(|err| {
        error!("Failed to validate token or extract claims: {:?}", err);
        AuthError::Unauthorized
    })?;

    let req = Request::from_parts(parts, body);

    if claim_check(&claims) {
        info!("Claim check passed.");
        Ok(next.run(req).await)
    } else {
        error!("Claim check failed.");
        Err(AuthError::Forbidden)
    }
}

#[instrument(skip(req, next))]
pub async fn require_role(
    req: Request<axum::body::Body>,
    next: Next,
    required_role: String,
) -> Result<Response, AuthError> {
    require_claim(req, next, move |claims| {
        claims.roles.as_ref().map_or(false, |roles| roles.contains(&required_role))
    }).await
}
