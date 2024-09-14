use axum::{Router, Extension, Json};
use axum::routing::{get, post};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use std::sync::Arc;
use axum::extract::{Path, Query};
use axum::http::{HeaderMap, Response, StatusCode};
use axum::http::header::CONTENT_TYPE;
use axum::response::IntoResponse;
use serde::Serialize;
use tracing::info;
use rustproof::config::{ServerConfig, TemplateType};
use rustproof::controllers;
use rustproof::prelude::RustProofClaims;
use rustproof::services::{AccessTokenService, AuthService};

pub async fn run_http_server(config: Arc<ServerConfig>, auth_service: Arc<dyn AuthService + Send + Sync>, token_service: Arc<dyn AccessTokenService + Send + Sync>) -> Result<(), Box<dyn std::error::Error>> {
    let app = build_app(config.clone(), auth_service, token_service);

    let addr = SocketAddr::from(([127, 0, 0, 1], config.http_port));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("HTTP server running at http://{}", addr);
    // axum_server::bind(addr)
    //     .handle(handle)
    //     .serve(app.into_make_service())
    //     .await

    // axum::from_tcp(listener)
    //     .unwrap()
    //     .serve(app.into_make_service())
    //     .await?;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Server running at http://{}", addr);
    axum::serve(listener, app.into_make_service()).await.unwrap();
    Ok(())
}

fn build_app(config: Arc<ServerConfig>, auth_service: Arc<dyn AuthService + Send + Sync>, token_service: Arc<dyn AccessTokenService + Send + Sync>) -> Router {
    Router::new()
        .nest("/admin", Router::new()
            .route("/generate_link",
                   post(controllers::admin::generate_link),
            )
            .route("/user/:user_id",
                   get(controllers::admin::get_user)
                       .put(controllers::admin::update_user)
                       .delete(controllers::admin::delete_user),
            )
            .route("/users", get(controllers::admin::list_users))
            .route_layer(Extension(auth_service.clone()))
            .route_layer(Extension(token_service.clone())),
        )
        .nest("/oauth2", Router::new()
            .route("/authorize", get(controllers::oauth2::authorize))
            .route("/callback", get(controllers::oauth2::callback))
            .route("/token", post(controllers::oauth2::token)),
        )
        .route("/health", get(health_check))
        .route("/invite", post(send_invite))
        .route("/logout", post(logout))
        .route("/otp", post(otp))
        .route("/recover", post(recover_password))
        .route("/settings", get(get_settings))
        .route("/signup", post(controllers::signup))
        // .route("/user", get(controllers::user::get_current_user))
        .route("/verify", get(verify_signup))
        .route("/email/:locale/:template_type/:format", get(email_template_handler))
        .route("/email/:locale/:template_type", get(email_template_handler_mp))
        .layer(axum::middleware::from_fn(rustproof::middleware::extract_claims))
        .layer(Extension(config.clone()))
        .layer(Extension(auth_service))
        .layer(Extension(token_service))
}

async fn create_user() -> impl axum::response::IntoResponse {
    StatusCode::OK
}

#[derive(Serialize)]
struct HealthResponse {
    version: String,
    name: String,
    description: String,
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(crate::HealthResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
        name: env!("CARGO_PKG_NAME").to_string(),
        description: env!("CARGO_PKG_DESCRIPTION").to_string(),
    })).into_response()
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


async fn email_template_handler_mp(
    Path((locale, template_type)): Path<(String, TemplateType)>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let cache_control = headers.get("Cache-Control").map(|v| v.to_str().unwrap_or("")).unwrap_or("");

    // Example response for plain text and HTML formats
    let plain_text = format!("{:?} Template in {}", template_type, locale);
    let html_content = format!("<html><body><h1>{:?} Template in {}</h1></body></html>", template_type, locale);

    // Use MultipartBuilder to create the multipart response
    let (content_type, body) = rustproof::config::utils::MultipartBuilder::new("boundary42")
        .add_part("text/plain; charset=utf-8", &plain_text)
        .add_part("text/html; charset=utf-8", &html_content)
        .build();

    // Build the final response
    let mut response = Response::builder()
        .header(CONTENT_TYPE, content_type)
        .body(body)
        .unwrap();

    // Add Cache-Control header if present
    if !cache_control.is_empty() {
        response.headers_mut().insert("Cache-Control", cache_control.parse().unwrap());
    }

    response
}