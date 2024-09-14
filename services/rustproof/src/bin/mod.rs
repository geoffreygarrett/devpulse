use axum::body::Body;
use axum::extract::{Path, Query};
use axum::http::header::{ALLOW, CONTENT_TYPE};
use axum::http::{HeaderMap, Method, Response};
use axum::middleware::from_fn;
use axum::response::IntoResponse;
use axum::routing::{options, MethodRouter};
use axum::{http::StatusCode, routing::{get, post}, Extension, Json, Router};
use clap::builder::StyledStr;
use clap::{Parser, Subcommand};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use lettre::message::MultiPart;
use rustproof::adapter::GenericRepository;
use rustproof::config::{DatabaseConfig, LoggingConfig, ObservabilityConfig, RustproofConfig, SmtpConfig, TracingConfig};
use rustproof::config::{TemplateType, TokenDriver};
use rustproof::controllers::admin::UpdateUserPayload;
use rustproof::helper::OptionsBuilder;
use rustproof::middleware::extract_claims;
use rustproof::prelude::RustProofClaims;
use rustproof::repositories::UserRepository;
use rustproof::services::token_service::jwts_service::JwtTokenService;
use rustproof::services::AccessTokenService;
use rustproof::services::{AuthConfig, AuthService, AuthServiceImpl, PasswordService, RegistrationStatus};
use secrecy::ExposeSecret;
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Executor, PgConnection, PgPool};
use std::collections::HashSet;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tonic::IntoRequest;
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;
use validator::ValidationErrors;
mod cli;
#[derive(Serialize, Deserialize)]

struct User {
    id: String,
    name: String,
    email: String,
}

mod run;



#[derive(Debug, snafu::Snafu)]
enum ServiceError {
    #[snafu(display("Configuration error: {:#?}", source))]
    ConfigError { source: ConfigError },

    #[snafu(display("Database connection error: {}", source))]
    DatabaseConnectionError { source: sqlx::Error },

    #[snafu(display("Failed to set global default subscriber: {}", source))]
    TracingSetGlobalDefaultError { source: tracing::subscriber::SetGlobalDefaultError },

    #[snafu(display("Configuration validation error for {}: {}", config_name, source))]
    ConfigValidationError {
        config_name: String,
        source: ValidationErrors,
    },

    #[snafu(display("SMTP error: {}", source))]
    SmtpError { source: lettre::transport::smtp::Error },

    #[snafu(display("SMTP configuration error: {}", message))]
    SmtpConfigError { message: String },
}


const RUSTPROOF_LOG: &str = "RUSTPROOF_LOG_";
const RUSTPROOF_DB: &str = "RUSTPROOF_DB_";
const RUSTPROOF_SMTP: &str = "RUSTPROOF_SMTP_";
const RUSTPROOF: &str = "RUSTPROOF_";

use crate::cli::Cli;
use config::{Config, ConfigError, Environment, File};
use shaku::module;
use rustproof::utils::flatten_json;




// Define the module
module! {
    AppModule {
        components = [DbPoolImpl, SmtpClientImpl, AuthServiceImpl],
        providers = []
    }
}


#[tokio::main]
async fn main() -> Result<(), ServiceError> {
    // Parse command line arguments
    // let cli = Cli::parse();
    // tracing::info!("CLI arguments: {:?}", cli);

    // Log the start of the service
    tracing::info!("Starting Rustproof service");

    // Display the current working directory
    if let Ok(dir) = std::env::current_dir() {
        tracing::info!("Current directory: {:?}", dir);
    } else {
        tracing::warn!("Could not determine current directory");
    }

    // Load environment variables from .env file if available
    if dotenv::dotenv().is_ok() {
        tracing::info!("Loaded environment variables from .env file");
    } else {
        tracing::warn!("No .env file found or could not be loaded");
    }

    //////////////////////
    // CONFIGURATION
    //////////////////////
    let config = RustproofConfig::new_arc(Some(".")).context(ConfigSnafu)?;

    //////////////////////
    // LOGGING
    //////////////////////
    let _ = run::init::init_logging(&config.logging)?;
    let value = serde_json::to_value(&config).unwrap();
    let flattened = flatten_json(&value);
    let pretty_json = serde_json::to_string_pretty(&flattened).unwrap();
    tracing::info!("Flattened configuration:\n{}", pretty_json);

    //////////////////////
    // DATABASE
    //////////////////////
    let pool = run::init::init_db_pool(config.database.clone()).await?;

    //////////////////////
    // SMTP
    //////////////////////
    let smtp = config.smtp.as_ref()
        .map(|smtp_config| run::init::init_smtp_client(smtp_config))
        .transpose()?;

    if smtp.is_some() {
        tracing::info!("SMTP client initialized successfully");
    } else {
        tracing::info!("No SMTP configuration provided, skipping SMTP initialization");
    }

    //////////////////////
    // AUTHENTICATION
    //////////////////////
    let auth_service = run::init::init_auth_service(&config, pool.clone()).await?;

    run::server::run_server(config.server, pool, smtp).await?;

    Ok(())
}
// //////////////////////
// // SMTP
// //////////////////////
// // let smtp = rustproof::config::env::prefixed(RUSTPROOF_SMTP)
// //     .from_env::<SmtpConfig>()
// //     .context(ConfigValidationError { config_name: "SMTP" })
// //     .and_then(|config| config.validate_config().map(|_| config))
// //     .map(run::init::init_smtp_client)
// //     .map_err(|e| anyhow::anyhow!("SMTP configuration error: {}", e))?
// //     .await
// //     .map_err(|e| anyhow::anyhow!("Failed to initialize SMTP client: {}", e))?;
//
// // Load configuration from environment variables
// let config = match envy::prefixed("RUSTPROOF_").from_env::<RustproofConfig>()
// {
//     Ok(config) => {
//         println!("{}", serde_json::to_string_pretty(&config).unwrap());
//         Arc::new(config)
//     }
//     Err(err) => {
//         panic!("Configuration could not be loaded from environment variables: {}", err)
//     }
// };
//
// // Instantiate service
// // Instantiate repositories
// let user_repo: Arc<dyn UserRepository + Send + Sync> = Arc::new(GenericRepository::<PgPool>::new(pool.clone()));
// let refresh_token_repo = GenericRepository::<PgPool>::new(pool.clone());
// let session_repo = GenericRepository::<PgPool>::new(pool.clone());
//
// let mut validation = Validation::default();
// validation.aud = Some(HashSet::from_iter(vec![config.token.audience.clone()]));
//
//
// // let mailer = SMTPMailer::new(config.mailer.smtp.clone());
// let token_service: Arc<dyn AccessTokenService + Send + Sync> = match &config.as_ref().token.driver {
//     TokenDriver::Jwt(jwt_config) => {
//         let secret = jwt_config.jwt_secret.clone();
//         Arc::new(JwtTokenService::new(
//             EncodingKey::from_secret(secret.expose_secret().as_bytes()),
//             DecodingKey::from_secret(secret.expose_secret().as_bytes()),
//             validation,
//             config.as_ref().token.expiration_time as usize,
//             jwt_config.jwt_secret.expose_secret().clone(),
//         ))
//     }
//     _ => panic!("Token driver not supported"),
// };
//
// let auth_service: Arc<dyn AuthService + Send + Sync> = Arc::new(
//     AuthServiceImpl::new(
//         user_repo.clone(),
//         refresh_token_repo,
//         token_service.clone(),
//         PasswordService::new(),
//         session_repo,
//         AuthConfig::new(RegistrationStatus::Open),
//     )
// );

// Handler functions
async fn generate_link() -> impl axum::response::IntoResponse {
    StatusCode::OK
}

// async fn get_user(Path(user_id): Path<String>) -> impl axum::response::IntoResponse {
//     StatusCode::OK
// }
//
// async fn update_user(Path(user_id): Path<String>) -> impl axum::response::IntoResponse {
//     StatusCode::OK
// }
//
// async fn delete_user(Path(user_id): Path<String>) -> impl axum::response::IntoResponse {
//     StatusCode::OK
// }
//
// async fn list_users() -> impl axum::response::IntoResponse {
//     StatusCode::OK
// }

async fn create_user() -> impl axum::response::IntoResponse {
    StatusCode::OK
}


// #[derive(Serialize, Deserialize)]
// struct Root {
//     pub version: String,
//     pub name: String,
//     pub description: String,
// }
// {
// "version": "v2.40.1",
// "name": "GoTrue",
// "description": "GoTrue is a user registration and authentication API"
// }

async fn send_invite() -> impl axum::response::IntoResponse {
    StatusCode::OK
}

async fn logout(
    scope: Option<Query<String>>,
    Extension(claims): Extension<Option<Arc<RustProofClaims>>>,
) -> impl IntoResponse {
    match claims {
        Some(claims) => {
            StatusCode::NO_CONTENT
        }
        None => {
            StatusCode::UNAUTHORIZED
        }
    }
}

async fn otp() -> impl axum::response::IntoResponse {
    StatusCode::OK
}

async fn recover_password() -> impl IntoResponse {
    StatusCode::OK
}

async fn get_settings() -> impl IntoResponse {
    // (StatusCode::OK, Json(Settings::default())).into_response()
    (StatusCode::OK, Json(())).into_response()
}

async fn verify_signup() -> impl IntoResponse {
    StatusCode::OK
}

async fn email_template_handler(
    Path((locale, template_type, format)): Path<(String, String, String)>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let cache_control = headers.get("Cache-Control").map(|v| v.to_str().unwrap_or("")).unwrap_or("");

    // Example response based on the format requested
    let content = match format.as_str() {
        "html" => format!("<html><body><h1>{} Template in {}</h1></body></html>", template_type, locale),
        "plain" => format!("{} Template in {}", template_type, locale),
        _ => format!("{} in {}", template_type, locale),
    };

    // Respond with appropriate content type and cache control headers
    let mut response = (headers.clone(), content).into_response();
    response.headers_mut().insert("Content-Type", format!("text/{}", format).parse().unwrap());

    if !cache_control.is_empty() {
        response.headers_mut().insert("Cache-Control", cache_control.parse().unwrap());
    }

    response
}

// async fn email_template_handler_mp(
//     Path((locale, template_type)): Path<(String, TemplateType)>,
//     headers: HeaderMap,
// ) -> impl IntoResponse {
//     let cache_control = headers.get("Cache-Control").map(|v| v.to_str().unwrap_or("")).unwrap_or("");
//
//     // Example response for plain text and HTML formats
//     let plain_text = format!("{:?} Template in {}", template_type, locale);
//     let html_content = format!("<html><body><h1>{:?} Template in {}</h1></body></html>", template_type, locale);
//
//     // Use MultipartBuilder to create the multipart response
//     let (content_type, body) = MultipartBuilder::new("boundary42")
//         .add_part("text/plain; charset=utf-8", &plain_text)
//         .add_part("text/html; charset=utf-8", &html_content)
//         .build();
//
//     // Build the final response
//     let mut response = Response::builder()
//         .header(CONTENT_TYPE, content_type)
//         .body(body)
//         .unwrap();
//
//     // Add Cache-Control header if present
//     if !cache_control.is_empty() {
//         response.headers_mut().insert("Cache-Control", cache_control.parse().unwrap());
//     }
//
//     response
// }
// // }