// use std::sync::Arc;
//
// use axum::Extension;
// use axum::extract::FromRequestParts;
// use axum::http::request::Parts;
// use axum_core::RequestPartsExt;
// use serde::{Deserialize, Serialize};
// use tower::buffer::error::ServiceError;
//
// use devpulse_core::auth::jwt::JwtService;
//
// use super::error::ServiceError;
//
// #[derive(Debug, Deserialize, Serialize)]
// pub struct Claims {
//     pub sub: String,
//     pub exp: usize,
//     pub org: Option<String>,
//     pub iat: Option<usize>,
//     pub iss: Option<String>,
//     pub aud: Option<String>,
//     pub jti: Option<String>,
// }
//
// pub struct AuthenticatedUser(pub Claims);
//
// #[async_trait::async_trait]
// impl<S> FromRequestParts<S> for AuthenticatedUser
// where
//     Arc<JwtService>: Send + Sync,
// {
//     type Rejection = ServiceError;
//
//     async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         let Extension(jwt_service) = parts
//             .extract::<Extension<Arc<JwtService>>>()
//             .await
//             .map_err(|_| ServiceError::AuthError)?;
//
//         let token = parts
//             .headers
//             .get("Authorization")
//             .and_then(|h| h.to_str().ok())
//             .and_then(|h| {
//                 if h.starts_with("Bearer ") {
//                     Some(&h[7..])
//                 } else {
//                     None
//                 }
//             })
//             .ok_or(ServiceError::AuthError)?;
//
//         let claims = jwt_service.validate_token(token)?;
//
//         Ok(AuthenticatedUser(claims))
//     }
// }
