use crate::adapter::{CreateSessionParams, RefreshTokenRepository, SessionRepository, StoreRefreshTokenParams};
use crate::errors::{AuthError, InternalSnafu, PasswordHashSnafu, Result};
use crate::models::api::access_token_response::AccessTokenResponse;
use crate::models::api::signup::SignupParams;
use crate::models::api::user_update::UserUpdateParams;
use crate::models::auth::{Session, User};
use crate::repositories::{CreateUserParams, UserRecord, UserRepository};
use crate::services::password_service::{PasswordError, PasswordService};
use crate::services::token_service::AccessTokenService;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use snafu::futures::TryFutureExt;
use snafu::ResultExt;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tracing::{error, info, instrument, trace, warn};
use uuid::Uuid;

/// Configuration for the `AuthServiceImpl`.
#[derive(Debug)]
pub struct AuthConfig {
    pub require_email_verification: bool,
    pub enforce_password_strength: bool,
    pub registration_status: RegistrationStatus,
    pub verification_base_url: String,
}

impl AuthConfig {
    pub fn new(registration_status: RegistrationStatus) -> Self {
        Self {
            require_email_verification: true,
            enforce_password_strength: true,
            registration_status,
            verification_base_url: "http://localhost:3000".to_string(),
        }
    }
}

/// Enum representing different registration statuses.
#[derive(Debug, Clone)]
pub enum RegistrationStatus {
    Closed,
    InvitationOnly,
    Open,
}

pub struct AuthServiceImpl<RT, SR>
where
    RT: RefreshTokenRepository + Send + Sync,
    SR: SessionRepository + Send + Sync,
{
    user_repository: Arc<dyn UserRepository + Send + Sync>,
    token_repository: RT,
    token_service: Arc<dyn AccessTokenService + Send + Sync>,
    password_service: PasswordService,
    session_repository: SR,
    config: AuthConfig,
}

impl<RT, SR> AuthServiceImpl<RT, SR>
where
    RT: RefreshTokenRepository + Send + Sync,
    SR: SessionRepository + Send + Sync,
{
    pub fn new(
        user_repository: Arc<dyn UserRepository + Send + Sync>,
        token_repository: RT,
        token_service: Arc<dyn AccessTokenService + Send + Sync>,
        password_service: PasswordService,
        session_repository: SR,
        config: AuthConfig,
    ) -> Self {
        Self {
            user_repository,
            token_repository,
            token_service,
            password_service,
            session_repository,
            config,
        }
    }

    async fn handle_existing_user(&self, existing_user: UserRecord) -> Result<()> {
        if existing_user.confirmed_at.is_none() {
            warn!(
                "User with email {} attempted to sign up again without verifying their email.",
                existing_user.email
            );
            self.resend_verification_email(&existing_user).await?;
            Err(AuthError::EmailNotConfirmed)
        } else {
            Err(AuthError::EmailExists)
        }
    }

    async fn validate_registration(&self, input: &SignupParams) -> Result<()> {
        match &self.config.registration_status {
            RegistrationStatus::Closed => {
                warn!("Signup attempt while registration is closed.");
                Err(AuthError::SignupDisabled)
            }
            RegistrationStatus::InvitationOnly => {
                if input.invitation_token.is_none() {
                    warn!("Invitation-only signup attempt without an invitation token.");
                    return Err(AuthError::InviteNotFound);
                }
                let token = input.invitation_token.clone().unwrap();
                if !self.validate_invitation_token(&token).await? {
                    warn!("Invalid invitation token used: {}", token);
                    return Err(AuthError::InviteNotValid);
                }
                Ok(())
            }
            RegistrationStatus::Open => Ok(()),
        }
    }

    async fn check_password_strength(&self, password: &str, user_inputs_slice: &[String]) -> Result<()> {
        let user_inputs: Vec<&str> = user_inputs_slice.iter().map(|s| s.as_str()).collect();
        if self.config.enforce_password_strength {
            self.password_service
                .check_password_strength(password, user_inputs.as_slice())
                .map_err(|e| match e {
                    PasswordError::WeakPassword(e) => {
                        warn!("Weak password provided.");
                        AuthError::WeakPassword {
                            message: e.message,
                            feedback: e.feedback,
                        }
                    }
                    _ => {
                        error!("Unexpected password error: {:?}", e);
                        unreachable!("Unexpected error: {:?}", e)
                    }
                })?;
        }
        Ok(())
    }

    async fn create_and_store_user(&self, params: &CreateUserParams) -> Result<UserRecord> {
        let user = match self
            .user_repository
            .create_user(params)
            .await
        {
            Ok(user) => {
                info!("User created successfully: {}", user.id);
                user
            }
            Err(e) => {
                error!("Failed to create user: {:?}", e);
                return Err(e);
            }
        };

        if self.config.require_email_verification {
            self.send_verification_email(&user).await?;
        }

        Ok(user)
    }

    async fn resend_verification_email(&self, user: &UserRecord) -> Result<()> {
        // Logic to resend the verification email
        Ok(())
    }

    async fn send_verification_email(&self, user: &UserRecord) -> Result<()> {
        // Logic to send a new verification email
        Ok(())
    }
}

#[derive(Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub ip: Option<String>,        // Optional IP address
    pub user_agent: Option<String>, // Optional User-Agent string
}

// Data structures for OAuth 2.0 grants
#[derive(Deserialize)]
pub struct AuthorizationCodeGrant {
    pub code: String,
    pub redirect_uri: String,
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Deserialize)]
pub struct RefreshTokenGrant {
    pub refresh_token: String,
    pub scope: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

#[derive(Deserialize)]
pub struct ClientCredentialsGrant {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Deserialize)]
pub struct PasswordGrant {
    pub username: String,
    pub password: Secret<String>,
    pub scope: Option<String>,
}


struct UserInputBuilder {
    inputs: Vec<String>,
}

impl UserInputBuilder {
    fn new() -> Self {
        Self {
            inputs: Vec::new(),
        }
    }

    fn process_value(&mut self, value: &Value) {
        match value {
            Value::String(s) => self.inputs.push(s.clone()),
            Value::Array(arr) => {
                for item in arr {
                    if let Value::String(s) = item {
                        self.inputs.push(s.clone());
                    }
                }
            }
            _ => {} // Ignore non-string values
        }
    }

    fn with_data(mut self, data: Option<&HashMap<String, Value>>) -> Self {
        if let Some(data_map) = data {
            for value in data_map.values() {
                self.process_value(value);
            }
        }
        self
    }

    fn with_email(mut self, email: &str) -> Self {
        let local_part = email.split('@').next().unwrap_or_default().to_string();
        self.inputs.push(local_part);
        self.inputs.push(email.to_string());
        self
    }

    fn with_phone(mut self, phone: Option<&String>) -> Self {
        if let Some(phone_number) = phone {
            self.inputs.push(phone_number.clone());
        }
        self
    }

    fn build(self) -> Vec<String> {
        self.inputs
    }
}

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn signup(&self, input: SignupParams) -> Result<SignupResponse>;
    async fn login_user(&self, login: LoginRequest) -> Result<Session>;
    async fn logout_user(&self, refresh_token: String) -> Result<()>;
    async fn update_user(&self, input: UserUpdateParams) -> Result<()>;
    async fn refresh_session(&self, input: Session) -> Result<Session>;
    async fn validate_token(&self, token: String) -> Result<bool>;
    async fn get_registration_status(&self) -> RegistrationStatus;
    async fn verify_email(&self, token: String) -> Result<()>;
    // async fn send_verification_email(&self, user: &UserRecord) -> Result<()>;

    // OAuth 2.0 Specific Methods

    /// Handles the Authorization Code Grant.
    async fn handle_authorization_code_grant(
        &self,
        payload: AuthorizationCodeGrant,
    ) -> Result<AccessTokenResponse, AuthError>;

    /// Handles the Refresh Token Grant.
    async fn handle_refresh_token_grant(
        &self,
        payload: RefreshTokenGrant,
    ) -> Result<AccessTokenResponse, AuthError>;

    /// Handles the Client Credentials Grant.
    async fn handle_client_credentials_grant(
        &self,
        payload: ClientCredentialsGrant,
    ) -> Result<AccessTokenResponse, AuthError>;

    /// Handles the Resource Owner Password Credentials Grant.
    async fn handle_password_grant(
        &self,
        payload: PasswordGrant,
    ) -> Result<AccessTokenResponse, AuthError>;
}


#[derive(Serialize)]
pub struct SignupResponse {
    id: Uuid,
    email: String,
    confirmation_sent_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<UserRecord> for SignupResponse {
    fn from(user: UserRecord) -> Self {
        Self {
            id: user.id,
            email: user.email,
            confirmation_sent_at: user.confirmation_sent_at,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[async_trait]
impl<RT, SR> AuthService for AuthServiceImpl<RT, SR>
where
    RT: RefreshTokenRepository + Send + Sync,
    SR: SessionRepository + Send + Sync,
{
    async fn handle_authorization_code_grant(
        &self,
        payload: AuthorizationCodeGrant,
    ) -> Result<AccessTokenResponse, AuthError> {
        // // Validate the authorization code and client credentials
        // let user = self
        //     .token_service
        //     .validate_authorization_code(&payload.code, &payload.client_id, &payload.client_secret)
        //     .await?;
        //
        // // Issue access and refresh tokens
        // let tokens = self
        //     .token_service
        //     .issue_tokens(&user, &Uuid::new_v4(), None)
        //     .await?;

        Ok(AccessTokenResponse {
            // access_token: tokens.token,
            token: "tokens.token".to_string(),
            token_type: "bearer".to_string(),
            // expires_in: tokens.expires_in as u64,
            expires_in: Some(3600),
            // refresh_token: Some(tokens.refresh_token),
            refresh_token: None,
            scope: None,
        })
    }

    async fn handle_refresh_token_grant(
        &self,
        payload: RefreshTokenGrant,
    ) -> Result<AccessTokenResponse, AuthError> {
        // Validate the refresh token and issue a new access token
        let user = self
            .token_service
            .validate_refresh_token(&payload.refresh_token)
            .await?;

        let tokens = self
            .token_service
            .issue_tokens(&user, &Uuid::new_v4(), None)
            .await?;

        Ok(AccessTokenResponse {
            token: tokens.token,
            token_type: "bearer".to_string(),
            expires_in: tokens.expires_in,
            refresh_token: tokens.refresh_token,
            scope: payload.scope.clone(),
        })
    }

    async fn handle_client_credentials_grant(
        &self,
        payload: ClientCredentialsGrant,
    ) -> Result<AccessTokenResponse, AuthError> {
        // Validate the client credentials and issue an access token
        // let user = self
        //     .user_repository
        //     .get_user_by_client_credentials(&payload.client_id, &payload.client_secret)
        //     .await?;

        // let tokens = self
        //     .token_service
        //     .issue_tokens(&user, &Uuid::new_v4(), None)
        //     .await?;

        Ok(AccessTokenResponse {
            token: "tokens.token".to_string(), // tokens.token,
            token_type: "bearer".to_string(),
            // expires_in: tokens.expires_in as u64,
            expires_in: None,
            refresh_token: None,
            scope: None,
        })
    }

    #[instrument(skip(self, payload), fields(email = %payload.username))]
    async fn handle_password_grant(
        &self,
        payload: PasswordGrant,
    ) -> Result<AccessTokenResponse, AuthError> {
        // Validate the username and password, then issue an access token
        // let user = self
        //     .user_repository
        //     .validate_user_credentials(&payload.username, &payload.password)
        //     .await?;
        //
        // let tokens = self
        //     .token_service
        //     .issue_tokens(&user, &Uuid::new_v4(), None)
        //     .await?;

        // Retrieve the user by email
        let user = match self.user_repository.get_user_by_email(&payload.username).await? {
            Some(user) => user, // If a user is found, continue
            None => return Err(AuthError::InvalidCredentials) // If no user is found, return an error
        };
        // Verify the provided password against the stored encrypted password
        self.password_service
            .verify_password(&payload.password.expose_secret(), &user.encrypted_password)
            .map_err(|_| AuthError::InvalidCredentials)?;

        // Generate a new session ID
        let session_id = Uuid::new_v4();

        // Create a new session for the user
        let session_record = self
            .session_repository
            .create_session(CreateSessionParams {
                user_id: user.id,
                factor_id: None,
                aal: None,
                not_after: None,
                refreshed_at: None,
                user_agent: None,
                ip: None,
                tag: None,
            })
            .await?;

        // Generate access and refresh tokens
        let tokens = self
            .token_service
            .issue_tokens(&user, &session_id, None)
            .await?;

        Ok(AccessTokenResponse {
            token: tokens.token,
            token_type: "bearer".to_string(),
            expires_in: tokens.expires_in,
            refresh_token: tokens.refresh_token,
            scope: payload.scope.clone(),
        })
    }

    async fn verify_email(&self, token: String) -> Result<()> {
        // // 1. Fetch user based on the token
        // let user = self.user_repository.get_user_by_verification_token(&token).await?;
        //
        // // 2. Validate token (check expiration, usage, etc.)
        // if user.verification_token != token {
        //     return Err(AuthError::InvalidCredentials);
        // }
        //
        // // 3. Mark the email as confirmed
        // self.user_repository.confirm_user_email(user.id).await?;
        //
        // // 4. Optionally, delete or invalidate the token
        // self.user_repository.invalidate_verification_token(user.id).await?;

        Ok(())
    }

    // async fn send_verification_email(&self, user: &UserRecord) -> Result<()> {
    //     let token = self.generate_verification_token(user)?;
    //     let verification_link = format!(
    //         "{}/verify?token={}",
    //         self.config.verification_base_url, token
    //     );
    //
    //     self.email_service
    //         .send_email(
    //             &self.config.verification_email_sender,
    //             &user.email,
    //             "Verify your email",
    //             &format!("Please click on the following link to verify your email: {}", verification_link),
    //         )
    //         .await
    //         .context(InternalSnafu { code: 500_u16 })?;
    //
    //     Ok(())
    // }

    /// Handles user signup by validating input, checking password strength, and attempting user creation.
    ///
    /// # Arguments
    ///
    /// * `input` - Contains the email, password, and optional additional data for the signup process.
    ///
    /// # Workflow
    ///
    /// 1. **Registration Validation**: Ensures that the signup is allowed under the current
    ///     registration settings, such as open registration or invitation-only.
    /// 2. **Password Strength Check**: Collects various user inputs, including email and phone,
    ///     and ensures the password meets strength requirements.
    /// 3. **Password Hashing**: Securely hashes the password to prepare it for storage.
    /// 4. **User Creation**: Attempts to store the new user in the database.
    ///     If a conflict occurs (e.g., the email already exists), the process generates faux
    ///     user data to mitigate timing attacks.
    ///
    /// # Instrumentation
    ///
    /// This method is instrumented with `tracing` to log key steps and errors at different levels:
    /// - **INFO**: General progress of the signup process, such as start and successful completion.
    /// - **TRACE**: Detailed steps like validation, password strength checking, and password hashing.
    /// - **ERROR**: Failures at critical stages such as registration validation, password strength checking, or
    /// password hashing.
    /// - **WARN**: Issues during user creation, such as a duplicate email, handled by processing faux data.
    ///
    /// # Errors
    ///
    /// Returns appropriate error types if any stage of the signup process fails, including validation, password
    /// strength checks, and database operations.
    ///
    /// # Returns
    ///
    /// On success, returns a `SignupResponse` containing the user ID, email, and timestamps. If the user creation fails
    /// due to a duplicate email, returns a faux `SignupResponse` with a new user ID and the same email to maintain
    /// constant timing.
    #[instrument(level = "info", skip(self, input), fields(email = %input.email))]
    async fn signup(&self, input: SignupParams) -> Result<SignupResponse> {
        info!(email = %input.email, "Starting signup process");

        // Step 1: Validate registration status and invitation token
        if let Err(e) = self.validate_registration(&input).await {
            error!(email = %input.email, error = ?e, "Registration validation failed");
            return Err(e);
        }
        trace!(email = %input.email, "Registration validation passed");

        // Step 2: Prepare user inputs for password strength check
        let user_inputs = UserInputBuilder::new()
            .with_data(input.data.as_ref())
            .with_email(&input.email)
            .with_phone(input.phone.as_ref())
            .build();

        // Step 3: Check password strength
        trace!(email = %input.email, "Checking password strength");
        if let Err(e) = self.check_password_strength(&input.password, user_inputs.as_slice()).await {
            error!(email = %input.email, error = ?e, "Password strength check failed");
            return Err(e);
        }
        trace!(email = %input.email, "Password strength check passed");

        // Step 4: Hash the password
        trace!(email = %input.email, "Hashing password");
        let hashed_password = match self.password_service.hash_password(&input.password)
            .context(PasswordHashSnafu)
            .context(InternalSnafu { code: 500_u16 }) {
            Ok(hash) => hash,
            Err(e) => {
                error!(email = %input.email, error = ?e, "Password hashing failed");
                return Err(e);
            }
        };

        // Step 5: Prepare and attempt user creation
        let now = Utc::now();
        let user_id = Uuid::new_v4();
        let params = CreateUserParams {
            id: Some(user_id),
            email: input.email.clone(),
            encrypted_password: hashed_password,
            created_at: Some(now),
            updated_at: Some(now),
            confirmation_sent_at: Some(now),
        };

        match self.create_and_store_user(&params).await {
            Ok(user) => {
                info!(user_id = %user.id, email = %user.email, "User created successfully");
                Ok(SignupResponse::from(user))
            }
            Err(e) => {
                warn!(email = %input.email, error = ?e, "User creation failed, assuming user already exists and processing faux data");
                Ok(SignupResponse {
                    id: user_id,
                    email: input.email,
                    confirmation_sent_at: Some(now),
                    created_at: now,
                    updated_at: now,
                })
            }
        }
    }


    async fn login_user(&self, login: LoginRequest) -> Result<Session> {
        // Retrieve the user by email
        let user = match self.user_repository.get_user_by_email(&login.email).await? {
            Some(user) => user, // If user is found, continue
            None => return Err(AuthError::InvalidCredentials) // If no user is found, return an error
        };
        // Verify the provided password against the stored encrypted password
        self.password_service
            .verify_password(&login.password, &user.encrypted_password)
            .map_err(|_| AuthError::InvalidCredentials)?;

        // Generate a new session ID
        let session_id = Uuid::new_v4();

        // Create a new session for the user
        let session_record = self
            .session_repository
            .create_session(CreateSessionParams {
                user_id: user.id,
                factor_id: None,
                aal: None,
                not_after: None,
                refreshed_at: None,
                user_agent: None,
                ip: None,
                tag: None,
            })
            .await?;

        // Generate access and refresh tokens
        let tokens = self
            .token_service
            .issue_tokens(&user, &session_id, None)
            .await?;

        // Return the session containing access token, refresh token, and other details
        Ok(Session {
            access_token: tokens.token,
            refresh_token: tokens.refresh_token.expect("TODO: panic message"),
            token_type: "bearer".to_string(),
            expires_in: tokens.expires_in.expect("TODO: panic message") as i64,
            user_id: user.id,
            expires_at: Utc::now() + chrono::Duration::seconds(tokens.expires_in.expect("TODO: panic message") as i64),
        })
    }

    async fn logout_user(&self, refresh_token: String) -> Result<()> {
        self.token_repository
            .revoke_refresh_token(&refresh_token)
            .await?;
        Ok(())
    }

    async fn update_user(&self, params: UserUpdateParams) -> Result<()> {
        // if let Some(password) = params.password {
        //     let hashed_password = self
        //         .password_service
        //         .hash_password(&password)
        //         .context(InternalSnafu { code: 500_u16 })?;
        //     self.user_repository
        //         .update_password(&params.user_id, hashed_password)
        //         .await?;
        // }
        //
        // if let Some(email) = &params.email {
        //     self.user_repository
        //         .update_email(&params.user_id, email)
        //         .await?;
        // }
        //
        // if let Some(email_change_token) = &params.email_change_token {
        //     self.user_repository
        //         .update_email_change_token(&params.user_id, email_change_token)
        //         .await?;
        // }
        //
        // if let Some(phone_change_token) = &params.phone_change_token {
        //     self.user_repository
        //         .update_phone_change_token(&params.user_id, phone_change_token)
        //         .await?;
        // }
        //
        // if let Some(data) = &params.data {
        //     self.user_repository
        //         .update_user_data(&params.user_id, data)
        //         .await?;
        // }
        //
        // if let Some(app_data) = &params.app_data {
        //     self.user_repository
        //         .update_app_metadata(&params.user_id, app_data)
        //         .await?;
        // }

        Ok(())
    }

    async fn refresh_session(&self, input: Session) -> Result<Session> {
        // Validate the refresh token and retrieve the associated user ID
        let refresh_token_record = self
            .token_repository
            .validate_refresh_token(&input.refresh_token)
            .await?;

        // Retrieve the user by ID
        let user = self
            .user_repository
            .get_user_by_id(&refresh_token_record.user_id)
            .await?;

        // Generate a new session ID
        let session_id = Uuid::new_v4();

        // Create a new session for the user
        let session_record = self
            .session_repository
            .create_session(CreateSessionParams {
                user_id: user.id,
                factor_id: None,
                aal: None,
                not_after: None,
                refreshed_at: None,
                user_agent: None,
                ip: None,
                tag: None,
            })
            .await?;

        // Generate new access and refresh tokens
        let tokens = self
            .token_service
            .issue_tokens(&user, &session_id, Some(&refresh_token_record.id))
            .await?;

        // Store the new refresh token with the new session ID
        self.token_repository
            .store_refresh_token(StoreRefreshTokenParams {
                user_id: &user.id,
                token: &tokens.refresh_token.clone().expect("TODO: panic message"),
                parent_token_id: Some(&refresh_token_record.id),
                session_id: Some(&session_id),
                instance_id: &Default::default(),
            })
            .await?;

        // Return the new session with the updated tokens
        Ok(Session {
            access_token: tokens.token,
            refresh_token: tokens.refresh_token.expect("TODO: panic message"),
            token_type: "bearer".to_string(),
            expires_in: tokens.expires_in.expect("TODO: panic message") as i64,
            user_id: user.id,
            expires_at: chrono::Utc::now() + chrono::Duration::seconds(tokens.expires_in.expect("TODO: panic message") as i64),
        })
    }

    async fn validate_token(&self, token: String) -> Result<bool> {
        // self.token_service.validate_token(&token).await
        Ok(true)
    }

    async fn get_registration_status(&self) -> RegistrationStatus {
        todo!()
    }
}

impl<RT, SR> AuthServiceImpl<RT, SR>
where
    RT: RefreshTokenRepository + Send + Sync,
    SR: SessionRepository + Send + Sync,
{
    async fn validate_invitation_token(&self, token: &str) -> Result<bool> {
        // Implement validation logic here
        Ok(true)
    }
}
