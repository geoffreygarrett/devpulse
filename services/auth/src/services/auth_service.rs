use async_trait::async_trait;
use tonic::Status;

use crate::errors::Result;
// use crate::models::{User, Tokens};
use crate::repositories::{TokenRepository, UserRepository};
use crate::services::jwts_service::JwtsService;
use crate::services::pass_service::PasswordService;

trait UserRepository {}
trait TokenRepository {}

#[nject::injectable]
pub struct AuthServiceImpl<R: UserRepository, T: TokenRepository> {
    user_repository: R,
    token_repository: T,
    jwts_service: JwtsService,
    password_service: PasswordService,
}

#[async_trait]
pub trait AuthService {
    async fn register_user(
        &self, email: String, password: String, given_name: String,
    ) -> Result<Tokens>;
    async fn login_user(&self, email: String, password: String) -> Result<Tokens, Status>;
    async fn refresh_token(&self, refresh_token: String) -> Result<Tokens, Status>;
    async fn validate_token(&self, token: String) -> Result<bool, Status>;
}

#[async_trait]
impl<R: UserRepository, T: TokenRepository> AuthService for AuthServiceImpl<R, T> {
    async fn register_user(
        &self, email: String, password: String, given_name: String,
    ) -> Result<Tokens> {
        let hashed_password = self.password_service.hash_password(&password)?;
        let user = self
            .user_repository
            .create_user(email.clone(), hashed_password, given_name)
            .await?;
        let tokens = self.jwts_service.generate_tokens(user.id).await?;
        self.token_repository
            .store_refresh_token(user.id, &tokens.refresh_token)
            .await?;
        Ok(tokens)
    }

    async fn login_user(&self, email: String, password: String) -> Result<Tokens> {
        let user = self.user_repository.get_user_by_email(&email).await?;
        self.password_service
            .verify_password(&password, &user.password_hash)?;
        let tokens = self.jwts_service.generate_tokens(user.id).await?;
        self.token_repository
            .store_refresh_token(user.id, &tokens.refresh_token)
            .await?;
        Ok(tokens)
    }

    async fn refresh_token(&self, refresh_token: String) -> Result<Tokens> {
        let user_id = self
            .token_repository
            .validate_refresh_token(&refresh_token)
            .await?;
        let tokens = self.jwts_service.generate_tokens(user_id).await?;
        self.token_repository
            .store_refresh_token(user_id, &tokens.refresh_token)
            .await?;
        Ok(tokens)
    }

    async fn validate_token(&self, token: String) -> Result<bool> {
        self.jwts_service.validate_token(&token).await
    }
}
