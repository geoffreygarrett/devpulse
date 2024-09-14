// use axum::{
//     extract::{Extension, Json, Path},
//     http::StatusCode,
//     response::IntoResponse,
//     routing::post,
//     Router,
// };
// use serde::{Deserialize, Serialize};
// use std::sync::Arc;
// use uuid::Uuid;
// use webauthn_rs::prelude::*;
//
// // Assuming you have these modules
// use crate::services::fido_service::FidoService;
// use crate::startup::AppState;
// use crate::repositories::{SessionRepository, UserRepository};
//
// #[derive(Deserialize)]
// pub struct StartEnrollmentRequest {
//     pub username: String,
// }
//
// #[derive(Serialize)]
// pub struct StartEnrollmentResponse {
//     pub challenge: String,
//     pub rp_id: String,
// }
//
// #[derive(Deserialize)]
// pub struct CompleteEnrollmentRequest {
//     pub credential: RegisterPublicKeyCredential,
// }
//
// #[derive(Serialize)]
// pub struct CompleteEnrollmentResponse {
//     pub status: String,
// }
//
// #[derive(Deserialize)]
// pub struct StartAuthenticationRequest {
//     pub username: String,
// }
//
// #[derive(Serialize)]
// pub struct StartAuthenticationResponse {
//     pub challenge: String,
//     pub rp_id: String,
// }
//
// #[derive(Deserialize)]
// pub struct CompleteAuthenticationRequest {
//     pub credential: PublicKeyCredential,
// }
//
// #[derive(Serialize)]
// pub struct CompleteAuthenticationResponse {
//     pub status: String,
// }
//
// // Handler for starting the enrollment process
// pub async fn start_enrollment(
//     Extension(state): Extension<AppState>,
//     Extension(session_repo): Extension<Arc<dyn SessionRepository + Send + Sync>>,
//     Json(payload): Json<StartEnrollmentRequest>,
// ) -> Result<impl IntoResponse, WebauthnError> {
//     let username = payload.username;
//     let user_id = {
//         let users_guard = state.users.lock().await;
//         users_guard
//             .name_to_id
//             .get(&username)
//             .copied()
//             .unwrap_or_else(Uuid::new_v4)
//     };
//
//     session_repo.remove_value("reg_state").await?;
//
//     let exclude_credentials = {
//         let users_guard = state.users.lock().await;
//         users_guard
//             .keys
//             .get(&user_id)
//             .map(|keys| keys.iter().map(|sk| sk.cred_id().clone()).collect())
//     };
//
//     let (challenge, reg_state) = state
//         .webauthn
//         .start_passkey_registration(user_id, &username, &username, exclude_credentials)
//         .map_err(|_| WebauthnError::ChallengePersistenceError)?;
//
//     session_repo
//         .insert("reg_state", (username, user_id, reg_state))
//         .await?;
//
//     Ok(Json(StartEnrollmentResponse {
//         challenge: challenge.to_string(),
//         rp_id: state.webauthn.rp_id().to_string(),
//     }))
// }
//
// // Handler for completing the enrollment process
// pub async fn complete_enrollment(
//     Extension(state): Extension<AppState>,
//     Extension(session_repo): Extension<Arc<dyn SessionRepository + Send + Sync>>,
//     Json(payload): Json<CompleteEnrollmentRequest>,
// ) -> Result<impl IntoResponse, WebauthnError> {
//     let (username, user_id, reg_state) = session_repo
//         .get::<(String, Uuid, PasskeyRegistration)>("reg_state")
//         .await?
//         .ok_or(WebauthnError::ChallengeNotFound)?;
//
//     session_repo.remove_value("reg_state").await?;
//
//     let credential = state
//         .webauthn
//         .finish_passkey_registration(&payload.credential, &reg_state)
//         .map_err(|_| WebauthnError::CredentialPersistenceError)?;
//
//     let mut users_guard = state.users.lock().await;
//     users_guard
//         .keys
//         .entry(user_id)
//         .and_modify(|keys| keys.push(credential.clone()))
//         .or_insert_with(|| vec![credential.clone()]);
//
//     users_guard.name_to_id.insert(username, user_id);
//
//     Ok(Json(CompleteEnrollmentResponse {
//         status: "ok".to_string(),
//     }))
// }
//
// // Handler for starting the authentication process
// pub async fn start_authentication(
//     Extension(state): Extension<AppState>,
//     Extension(session_repo): Extension<Arc<dyn SessionRepository + Send + Sync>>,
//     Json(payload): Json<StartAuthenticationRequest>,
// ) -> Result<impl IntoResponse, WebauthnError> {
//     let username = payload.username;
//     let user_id = {
//         let users_guard = state.users.lock().await;
//         users_guard
//             .name_to_id
//             .get(&username)
//             .copied()
//             .ok_or(WebauthnError::InvalidUsername)?
//     };
//
//     session_repo.remove_value("auth_state").await?;
//
//     let allow_credentials = {
//         let users_guard = state.users.lock().await;
//         users_guard
//             .keys
//             .get(&user_id)
//             .ok_or(WebauthnError::CredentialNotFound)?
//     };
//
//     let (challenge, auth_state) = state
//         .webauthn
//         .start_passkey_authentication(allow_credentials)
//         .map_err(|_| WebauthnError::ChallengePersistenceError)?;
//
//     session_repo
//         .insert("auth_state", (user_id, auth_state))
//         .await?;
//
//     Ok(Json(StartAuthenticationResponse {
//         challenge: challenge.to_string(),
//         rp_id: state.webauthn.rp_id().to_string(),
//     }))
// }
//
// // Handler for completing the authentication process
// pub async fn complete_authentication(
//     Extension(state): Extension<AppState>,
//     Extension(session_repo): Extension<Arc<dyn SessionRepository + Send + Sync>>,
//     Json(payload): Json<CompleteAuthenticationRequest>,
// ) -> Result<impl IntoResponse, WebauthnError> {
//     let (user_id, auth_state) = session_repo
//         .get::<(Uuid, PasskeyAuthentication)>("auth_state")
//         .await?
//         .ok_or(WebauthnError::ChallengeNotFound)?;
//
//     session_repo.remove_value("auth_state").await?;
//
//     let auth_result = state
//         .webauthn
//         .finish_passkey_authentication(&payload.credential, &auth_state)
//         .map_err(|_| WebauthnError::AuthenticationFailure)?;
//
//     let mut users_guard = state.users.lock().await;
//     users_guard
//         .keys
//         .get_mut(&user_id)
//         .map(|keys| {
//             keys.iter_mut().for_each(|sk| {
//                 sk.update_credential(&auth_result);
//             })
//         })
//         .ok_or(WebauthnError::CredentialNotFound)?;
//
//     Ok(Json(CompleteAuthenticationResponse {
//         status: "ok".to_string(),
//     }))
// }
