use tonic::{Request, Response, Status};

use crate::errors::AuthError;
use crate::services::auth_service::AuthService;
use crate::v1::{
    AuthErrorType, AuthResponse, PasswordLoginRequest, RefreshTokenRequest, RegistrationRequest,
    ValidateTokenRequest, ValidateTokenResponse,
};
use crate::v1::auth_server::Auth as GrpcAuthService;

pub struct GrpcAuthServiceImpl<S: AuthService> {
    auth_service: S,
}

impl<S: AuthService> GrpcAuthServiceImpl<S> {
    pub fn new(auth_service: S) -> Self {
        Self { auth_service }
    }
}

#[async_trait::async_trait]
impl<S: AuthService + Send + Sync + 'static> GrpcAuthService for GrpcAuthServiceImpl<S> {
    async fn register_user(
        &self, request: Request<RegistrationRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();
        self.auth_service
            .register_user(req.email, req.password, req.data.get("given_name").cloned())
            .await
            .map(|session| {
                Response::new(AuthResponse {
                    session,
                    error: None,
                })
            })
            .map_err(|e| Status::internal(e.to_string()))
    }

    async fn login_with_password(
        &self, request: Request<PasswordLoginRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();
        self.auth_service
            .login_user(req.email, req.password)
            .await
            .map(|session| {
                Response::new(AuthResponse {
                    session,
                    error: None,
                })
            })
            .map_err(|e| Status::internal(e.to_string()))
    }

    async fn refresh_token(
        &self, request: Request<RefreshTokenRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();
        self.auth_service
            .refresh_token(req.refresh_token)
            .await
            .map(|session| {
                Response::new(AuthResponse {
                    session,
                    error: None,
                })
            })
            .map_err(|e| Status::internal(e.to_string()))
    }

    async fn validate_token(
        &self, request: Request<ValidateTokenRequest>,
    ) -> Result<Response<ValidateTokenResponse>, Status> {
        let req = request.into_inner();
        self.auth_service
            .validate_token(req.token)
            .await
            .map(|is_valid| Response::new(ValidateTokenResponse { valid: is_valid }))
            .map_err(|e| Status::internal(e.to_string()))
    }
}

impl From<AuthError> for crate::v1::AuthError {
    fn from(error: AuthError) -> Self {
        use crate::v1::AuthErrorType;
        let error_type = match error {
            AuthError::BadCodeVerifier { .. } => AuthErrorType::BadCodeVerifier,
            AuthError::BadJson { .. } => AuthErrorType::BadJson,
            AuthError::BadJwt { .. } => AuthErrorType::BadJwt,
            AuthError::CaptchaFailed { .. } => AuthErrorType::CaptchaFailed,
            AuthError::EmailNotConfirmed { .. } => AuthErrorType::EmailNotConfirmed,
            AuthError::InsufficientAal { .. } => AuthErrorType::InsufficientAal,
            AuthError::InviteNotFound { .. } => AuthErrorType::InviteNotFound,
            AuthError::NotAdmin { .. } => AuthErrorType::NotAdmin,
            AuthError::ReauthenticationNeeded { .. } => AuthErrorType::ReauthenticationNeeded,
            AuthError::ReauthenticationNotValid { .. } => AuthErrorType::ReauthenticationNotValid,
            AuthError::SessionNotFound { .. } => AuthErrorType::SessionNotFound,
            AuthError::SignupDisabled { .. } => AuthErrorType::SignupDisabled,
            AuthError::UnexpectedAudience { .. } => AuthErrorType::UnexpectedAudience,
            AuthError::UnexpectedFailure { .. } => AuthErrorType::UnexpectedFailure,
            AuthError::UserAlreadyExists { .. } => AuthErrorType::UserAlreadyExists,
            AuthError::UserBanned { .. } => AuthErrorType::UserBanned,
            AuthError::UserNotFound { .. } => AuthErrorType::UserNotFound,
            AuthError::UserSsoManaged { .. } => AuthErrorType::UserSsoManaged,
            AuthError::ValidationFailed { .. } => AuthErrorType::ValidationFailed,
            AuthError::WeakPassword { .. } => AuthErrorType::WeakPassword,
            AuthError::Conflict { .. } => AuthErrorType::Conflict,
            AuthError::EmailExists { .. } => AuthErrorType::EmailExists,
            AuthError::SamePassword { .. } => AuthErrorType::SamePassword,
            _ => AuthErrorType::UnexpectedFailure,
        };
        crate::v1::AuthError {
            error_type: error_type as i32,
            message: format!("{}", error),
        }
    }
}
