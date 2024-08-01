use tonic::{Request, Response, Status};

use crate::auth::{
    AuthResponse, LoginRequest, RegisterRequest, TokenRequest, TokenResponse, ValidateTokenRequest,
    ValidateTokenResponse,
};
use crate::auth::auth_service_server::AuthService as GrpcAuthService;
use crate::auth_service::AuthService;

pub struct GrpcAuthServiceImpl<S: AuthService> {
    auth_service: S,
}

impl<S: AuthService> GrpcAuthServiceImpl<S> {
    pub fn new(auth_service: S) -> Self {
        Self { auth_service }
    }
}

#[tonic::async_trait]
impl<S: AuthService + Send + Sync + 'static> GrpcAuthService for GrpcAuthServiceImpl<S> {
    async fn register(
        &self, request: Request<RegisterRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();
        let tokens = self
            .auth_service
            .register_user(req.email, req.password, req.given_name)
            .await?;
        Ok(Response::new(AuthResponse {
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
        }))
    }

    async fn login(
        &self, request: Request<LoginRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();
        let tokens = self
            .auth_service
            .login_user(req.email, req.password)
            .await?;
        Ok(Response::new(AuthResponse {
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
        }))
    }

    async fn refresh_token(
        &self, request: Request<TokenRequest>,
    ) -> Result<Response<TokenResponse>, Status> {
        let req = request.into_inner();
        let tokens = self.auth_service.refresh_token(req.refresh_token).await?;
        Ok(Response::new(TokenResponse {
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
            access_token_expiry: Some(tokens.access_token_expiry.into()),
            refresh_token_expiry: Some(tokens.refresh_token_expiry.into()),
        }))
    }

    async fn validate_token(
        &self, request: Request<ValidateTokenRequest>,
    ) -> Result<Response<ValidateTokenResponse>, Status> {
        let req = request.into_inner();
        let is_valid = self.auth_service.validate_token(req.token).await?;
        Ok(Response::new(ValidateTokenResponse { is_valid }))
    }
}
