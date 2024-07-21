use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde_json::json;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Error, Debug, ToSchema)]
pub enum DevPulseError {
    #[error("Missing credentials")]
    // #[schema(example = "Missing username or password.")]
    MissingCredentials,

    #[error("Wrong credentials")]
    // #[schema(example = "Invalid username or password.")]
    WrongCredentials,

    #[error("Error creating token")]
    // #[schema(example = "Failed to create token.")]
    TokenCreation,

    #[error("Internal server error {0}")]
    // #[schema(example = "Internal server error.")]
    InternalServerError(String),

    // Repository-specific errors
    #[error("Repository not found")]
    // #[schema(example = "Repository not found.")]
    RepositoryNotFound,

    #[error("Failed to process repository data")]
    // #[schema(example = "Failed to process repository data.")]
    RepositoryProcessingError,

    // Developer-specific errors
    #[error("Developer not found")]
    // #[schema(example = "Developer not found.")]
    DeveloperNotFound,

    #[error("Failed to calculate developer performance")]
    // #[schema(example = "Failed to calculate developer performance.")]
    DeveloperPerformanceError,
}

impl IntoResponse for DevPulseError {
    fn into_response(self) -> Response {
        let (status, error_message) = match &self {
            DevPulseError::MissingCredentials => (StatusCode::BAD_REQUEST, self.to_string()),
            DevPulseError::WrongCredentials => (StatusCode::UNAUTHORIZED, self.to_string()),
            DevPulseError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            DevPulseError::InternalServerError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            DevPulseError::RepositoryNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            DevPulseError::RepositoryProcessingError => (StatusCode::BAD_REQUEST, self.to_string()),
            DevPulseError::DeveloperNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            DevPulseError::DeveloperPerformanceError => (StatusCode::BAD_REQUEST, self.to_string()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
