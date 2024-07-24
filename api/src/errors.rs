use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Error, Debug, Serialize, ToSchema)]
#[allow(dead_code)]
pub enum DevPulseError {
    #[error("Missing credentials")]
    #[schema(example = "Missing username or password")]
    MissingCredentials,

    #[error("Wrong credentials provided")]
    #[schema(example = "Invalid username or password")]
    WrongCredentials,

    #[error("Error creating token")]
    #[schema(example = "Failed to create token")]
    TokenCreation,

    #[error("Internal server error: {0}")]
    #[schema(example = "Internal server error: Error connecting to database")]
    InternalServerError(String),

    #[error("Repository not found")]
    #[schema(example = "Repository not found")]
    RepositoryNotFound,

    #[error("Failed to process repository data: {0}")]
    #[schema(example = "Failed to process repository data: Data format error")]
    RepositoryProcessingError(String),

    #[error("Developer not found")]
    #[schema(example = "Developer not found")]
    DeveloperNotFound,

    #[error("Failed to calculate developer performance")]
    #[schema(example = "Failed to calculate developer performance due to lack of data")]
    DeveloperPerformanceError,
}

impl IntoResponse for DevPulseError {
    fn into_response(self) -> Response {
        let status = match &self {
            DevPulseError::MissingCredentials | DevPulseError::WrongCredentials => {
                StatusCode::UNAUTHORIZED
            }
            DevPulseError::TokenCreation | DevPulseError::InternalServerError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            DevPulseError::RepositoryNotFound | DevPulseError::DeveloperNotFound => {
                StatusCode::NOT_FOUND
            }
            DevPulseError::RepositoryProcessingError(_)
            | DevPulseError::DeveloperPerformanceError => StatusCode::BAD_REQUEST,
        };

        let error_message = self.to_string();
        let body = Json(json!({
            "error": error_message,
            "type": format!("{:?}", self)
        }));

        (status, body).into_response()
    }
}
