#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Internal server error")]
    InternalError(String),
}

impl From<ServiceError> for tonic::Status {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::DatabaseError(db_err) => match db_err {
                sqlx::Error::RowNotFound => tonic::Status::not_found("Row not found"),
                sqlx::Error::PoolTimedOut => {
                    tonic::Status::unavailable("Database connection pool timed out")
                }
                _ => tonic::Status::internal(format!("Database error: {:?}", db_err)),
            },
            ServiceError::InternalError(msg) => tonic::Status::internal(msg),
        }
    }
}
