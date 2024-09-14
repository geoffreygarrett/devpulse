use async_trait::async_trait;
use axum::{
    extract::{Extension, Query},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;

// Enum for well-known and generic OAuth providers
#[derive(Debug, Deserialize)]
#[serde(tag = "provider_type", content = "config")]
pub enum OAuthProviderConfig {
    Google(GoogleConfig),
    GitHub(GitHubConfig),
    Generic(GenericOAuthConfig),
}

// Overall OAuth service configuration
#[derive(Debug, Deserialize)]
pub struct OAuthServiceConfig {
    pub providers: HashMap<String, OAuthProviderConfig>,
}

impl OAuthServiceConfig {
    pub fn new(providers: HashMap<String, OAuthProviderConfig>) -> Self {
        Self { providers }
    }

    pub fn from_env() -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;
        settings.try_deserialize()
    }
}

// Configurations for specific OAuth providers
#[derive(Debug, Deserialize)]
pub struct GoogleConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

#[derive(Debug, Deserialize)]
pub struct GenericOAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
}

// Error type for OAuth operations
#[derive(Debug)]
pub struct OAuthError {
    pub message: String,
}

impl IntoResponse for OAuthError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, Json(json!({ "error": self.message }))).into_response()
    }
}

// Token structure returned by OAuth providers
#[derive(Debug, Serialize)]
pub struct OAuthToken {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<u64>,
    pub scopes: Vec<String>,
}

// Trait for OAuth providers, allowing different implementations
#[async_trait]
pub trait OAuthProvider: Send + Sync {
    async fn authorize(&self) -> Result<String, OAuthError>;
    async fn exchange_code_for_token(&self, code: &str) -> Result<OAuthToken, OAuthError>;
}

// OAuth service that manages different providers
pub struct OAuthService {
    providers: HashMap<String, Arc<dyn OAuthProvider>>,
}

impl OAuthService {
    pub fn new(config: OAuthServiceConfig) -> Self {
        let mut providers: HashMap<String, Arc<dyn OAuthProvider>> = HashMap::new();

        for (name, provider_config) in config.providers {
            match provider_config {
                OAuthProviderConfig::Google(cfg) => {
                    providers.insert(name, Arc::new(GoogleOAuth { config: cfg }));
                }
                OAuthProviderConfig::GitHub(cfg) => {
                    providers.insert(name, Arc::new(GitHubOAuth { config: cfg }));
                }
                OAuthProviderConfig::Generic(cfg) => {
                    providers.insert(name, Arc::new(GenericOAuth { config: cfg }));
                }
            }
        }

        Self { providers }
    }

    pub fn get_provider(&self, name: &str) -> Option<&Arc<dyn OAuthProvider>> {
        self.providers.get(name)
    }
}

// Implementation for Google OAuth
pub struct GoogleOAuth {
    config: GoogleConfig,
}

#[async_trait]
impl OAuthProvider for GoogleOAuth {
    async fn authorize(&self) -> Result<String, OAuthError> {
        Ok(format!(
            "https://accounts.google.com/o/oauth2/auth?client_id={}&redirect_uri={}&response_type=code&scope=email",
            self.config.client_id, self.config.redirect_uri
        ))
    }

    async fn exchange_code_for_token(&self, code: &str) -> Result<OAuthToken, OAuthError> {
        Ok(OAuthToken {
            access_token: "mock_google_access_token".to_string(),
            refresh_token: Some("mock_google_refresh_token".to_string()),
            expires_in: Some(3600),
            scopes: vec!["email".to_string()],
        })
    }
}

// Implementation for GitHub OAuth
pub struct GitHubOAuth {
    config: GitHubConfig,
}

#[async_trait]
impl OAuthProvider for GitHubOAuth {
    async fn authorize(&self) -> Result<String, OAuthError> {
        Ok(format!(
            "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope=repo",
            self.config.client_id, self.config.redirect_uri
        ))
    }

    async fn exchange_code_for_token(&self, code: &str) -> Result<OAuthToken, OAuthError> {
        Ok(OAuthToken {
            access_token: "mock_github_access_token".to_string(),
            refresh_token: None,
            expires_in: Some(3600),
            scopes: vec!["repo".to_string()],
        })
    }
}

// Example of a generic OAuth provider implementation
pub struct GenericOAuth {
    config: GenericOAuthConfig,
}

#[async_trait]
impl OAuthProvider for GenericOAuth {
    async fn authorize(&self) -> Result<String, OAuthError> {
        let scope = self.config.scopes.join(" ");
        Ok(format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope={}",
            self.config.auth_url, self.config.client_id, self.config.redirect_uri, scope
        ))
    }

    async fn exchange_code_for_token(&self, code: &str) -> Result<OAuthToken, OAuthError> {
        Ok(OAuthToken {
            access_token: "mock_generic_access_token".to_string(),
            refresh_token: Some("mock_generic_refresh_token".to_string()),
            expires_in: Some(3600),
            scopes: self.config.scopes.clone(),
        })
    }
}



// #[tokio::main]
// async fn main() {
//     let config = OAuthServiceConfig::from_env().unwrap();
//
//     let oauth_service = OAuthService::new(config);
//
//     let app = axum::Router::new()
//         .route("/authorize", axum::routing::get(authorize))
//         .route("/callback", axum::routing::get(callback))
//         .layer(Extension(Arc::new(oauth_service)));
//
//     axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }
