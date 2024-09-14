use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::sync::Arc;

use axum::{
    extract::Extension,
    http::StatusCode
    ,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use http::Method;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
// use tower_http::body::Full;
// use tower_http::trace::TraceLayer;

mod grpc;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub api_host: Option<String>,
    #[serde(default = "default_http_port")]
    pub http_port: u16,
    #[serde(default = "default_grpc_port")]
    pub grpc_port: u16,
    #[serde(default)]
    pub api_endpoint: Option<String>,
    #[serde(default)]
    pub request_id_header: Option<String>,
    #[serde(default = "default_api_external_url")]
    pub api_external_url: Option<String>,
    #[serde(default = "default_enable_http")]
    pub enable_http: bool,
    #[serde(default = "default_enable_grpc")]
    pub enable_grpc: bool,
}
impl ServerConfig {
    // ... [other methods remain the same]

    fn get_socket_addr(&self, port: u16) -> SocketAddr {
        let ip = self.api_host
            .as_ref()
            .and_then(|host| IpAddr::from_str(host).ok())
            .unwrap_or(IpAddr::V4(std::net::Ipv4Addr::LOCALHOST));
        SocketAddr::new(ip, port)
    }
}
fn default_http_port() -> u16 { 8081 }
fn default_grpc_port() -> u16 { 50051 }
fn default_api_external_url() -> Option<String> { None }
fn default_enable_http() -> bool { true }
fn default_enable_grpc() -> bool { true }

pub async fn run_server(
    config: Arc<ServerConfig>,
    auth_service: Arc<dyn rustproof::services::AuthService + Send + Sync>,
    token_service: Arc<dyn rustproof::services::AccessTokenService + Send + Sync>,
) -> Result<(), Box<dyn std::error::Error>> {
    let http_app = build_http_app(config.clone(), auth_service.clone(), token_service.clone());
    // let grpc_service = grpc::build_grpc_service(auth_service, token_service);

    let http_addr = config.get_socket_addr(config.http_port);
    // let grpc_addr = config.get_socket_addr(config.grpc_port);

    if config.enable_http {
        tokio::spawn(run_http_server(http_app, http_addr));
    }

    if config.enable_grpc {
        // tokio::spawn(run_grpc_server(grpc_service, grpc_addr));
    }

    tokio::signal::ctrl_c().await?;
    println!("Shutting down servers");

    Ok(())
}

async fn run_http_server(app: Router, addr: SocketAddr) {
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("HTTP server running on http://{}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

// Update the build_grpc_service function in your grpc module:
pub mod grpcs {

    // ... [other code in the grpc module]

    // pub fn build_grpc_service(
    //     auth_service: Arc<dyn rustproof::services::AuthService + Send + Sync>,
    //     token_service: Arc<dyn rustproof::services::AccessTokenService + Send + Sync>,
    // ) -> impl tower::Service<Request<Body>, Response = Response<Body>, Error = Infallible>
    // + Clone
    // + Send
    // + 'static {
    //     let grpc_service = RustProofAuthService {
    //         auth_service,
    //         token_service,
    //     };
    //
    //     ServiceBuilder::new()
    //         .service(AuthServer::new(grpc_service))
    // }
}

use hyper::body::Body;
use rustproof::helper::OptionsBuilder;
// async fn run_grpc_server<S, RB>(service: S, addr: SocketAddr)
// where
//     S: tonic::server::NamedService,
//     S: tower::Service<RB, Response = hyper::body::Incoming, Error = Infallible>
//     + Clone
//     + Send
//     + 'static,
// {
//     println!("gRPC server running on http://{}", addr);
//     TonicServer::builder()
//         .add_service(service)
//         .serve(addr)
//         .await
//         .unwrap();
// }


fn build_http_app(
    config: Arc<ServerConfig>,
    auth_service: Arc<dyn rustproof::services::AuthService + Send + Sync>,
    token_service: Arc<dyn rustproof::services::AccessTokenService + Send + Sync>,
) -> Router {
    Router::new()
        .nest("/admin", Router::new()
            // .route("/generate_link",
            //        OptionsBuilder::new()
            //            .allow(Method::POST)
            //            .build()
            //            .post(rustproof::controllers::admin::generate_link),
            // )
            .route("/user/:user_id",
                   OptionsBuilder::new()
                       .allow(Method::GET)
                       .allow(Method::PUT)
                       .allow(Method::DELETE)
                       .build()
                       .get(rustproof::controllers::admin::get_user)
                       .put(rustproof::controllers::admin::update_user)
                       .delete(rustproof::controllers::admin::delete_user),
            )
            .route("/users",
                   OptionsBuilder::new()
                       .allow(Method::GET)
                       .allow(Method::POST)
                       .build()
                       .get(rustproof::controllers::admin::list_users),
            )
            .layer(Extension(auth_service.clone()))
            .layer(Extension(token_service.clone()))
            // .layer(axum::middleware::from_fn(move |req, next| {
              //     let admin_group_name = config.token.admin_group_name.clone();
              //     async move {
              //         rustproof::middleware::require_role(req, next, admin_group_name).await
              //     }
              // })),
        )
        .nest("/oauth2", Router::new()
            .route("/authorize",
                   OptionsBuilder::new()
                       .allow(Method::GET)
                       .build()
                       .get(rustproof::controllers::oauth2::authorize),
            )
            .route("/callback",
                   OptionsBuilder::new()
                       .allow(Method::GET)
                       .build()
                       .get(rustproof::controllers::oauth2::callback),
            )
            .route("/token",
                   OptionsBuilder::new()
                       .allow(Method::POST)
                       .build()
                       .post(rustproof::controllers::oauth2::token),
            ),
        )
        .route("/health", get(rustproof::controllers::health::health_check))
        // .route("/invite",
        //        OptionsBuilder::new()
        //            .allow(Method::POST)
        //            .build()
        //            .post(rustproof::controllers::invite::send_invite))
        // .route("/logout",
        //        OptionsBuilder::new()
        //            .allow(Method::POST)
        //            .build()
        //            .post(rustproof::controllers::auth::logout))
        // .route("/otp",
        //        OptionsBuilder::new()
        //            .allow(Method::POST)
        //            .build()
        //            .post(rustproof::controllers::auth::otp))
        // .route("/recover",
        //        OptionsBuilder::new()
        //            .allow(Method::POST)
        //            .build()
        //            .post(rustproof::controllers::auth::recover_password))
        // .route("/settings",
        //        OptionsBuilder::new()
        //            .allow(Method::GET)
        //            .build()
        //            .get(rustproof::controllers::settings::get_settings),
        // )
        .route("/signup",
               OptionsBuilder::new()
                   .allow(Method::POST)
                   .build()
                   .post(rustproof::controllers::signup))
        // .route("/user",
        //        OptionsBuilder::new()
        //            .allow(Method::GET)
        //            .allow(Method::PUT)
        //            .build()
        //            .get(rustproof::controllers::user::get_current_user)
        //            .put(rustproof::controllers::user::update_current_user),
        // )
        // .route("/verify",
        //        OptionsBuilder::new()
        //            .allow(Method::GET)
        //            .build()
        //            .get(rustproof::controllers::auth::verify_signup))
        .route("/email/:locale/:template_type", get(rustproof::controllers::email::email_template_handler))
        .layer(
            ServiceBuilder::new()
                .layer(axum::middleware::from_fn(rustproof::middleware::extract_claims))
                .layer(Extension(config))
                .layer(Extension(auth_service))
                .layer(Extension(token_service))
        )
}
