use crate::ServiceError;
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use rustproof::adapter::GenericRepository;
use rustproof::config::{RustproofConfig, TokenDriver};
use rustproof::repositories::UserRepository;
use rustproof::services::token_service::jwts_service::JwtTokenService;
use rustproof::services::{
    AccessTokenService, AuthConfig, AuthService, AuthServiceImpl,
    PasswordService, RegistrationStatus,
};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::collections::HashSet;
use std::sync::Arc;

pub async fn init_auth_service(
    config: &Arc<RustproofConfig>,
    pool: PgPool,
) -> Result<Arc<dyn AuthService + Send + Sync>, ServiceError> {
    // Initialize repositories
    let user_repo: Arc<dyn UserRepository + Send + Sync> = Arc::new(GenericRepository::<PgPool>::new(pool.clone()));
    let refresh_token_repo = GenericRepository::<PgPool>::new(pool.clone());
    let session_repo = GenericRepository::<PgPool>::new(pool.clone());

    // Set up JWT validation
    let mut validation = Validation::default();
    validation.aud = Some(HashSet::from_iter(vec![config.oauth2.access_token.audience.clone()]));

    // Initialize token service
    let token_service: Arc<dyn AccessTokenService + Send + Sync> = match &config.oauth2.access_token.driver {
        TokenDriver::Jwt(jwt_config) => {
            let secret = jwt_config.jwt_secret.clone();
            Arc::new(JwtTokenService::new(
                EncodingKey::from_secret(secret.expose_secret().as_bytes()),
                DecodingKey::from_secret(secret.expose_secret().as_bytes()),
                validation,
                config.oauth2.access_token.expiration_time as usize,
                jwt_config.jwt_secret.expose_secret().clone(),
            ))
        }
        _ => panic!("Token driver not supported"),
        // _ => anyhow::bail!("Token driver not supported"),
    };

    // Initialize auth service
    let auth_service: Arc<dyn AuthService + Send + Sync> = Arc::new(
        AuthServiceImpl::new(
            user_repo.clone(),
            refresh_token_repo,
            token_service.clone(),
            PasswordService::new(),
            session_repo,
            AuthConfig::new(RegistrationStatus::Open),
        )
    );

    Ok(auth_service)
}