#![allow(warnings)]

use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::Path;
use std::pin::Pin;
use std::sync::Arc;
use std::time::SystemTime;

use async_stream::{stream, try_stream};
use axum_server::Server as AxumServer;
use axum_server::tls_rustls::bind_rustls;
use axum_server::tls_rustls::RustlsConfig;
use bytes::Bytes;
use futures_util::{AsyncReadExt, Stream};
use futures_util::StreamExt;
use reqwest::tls;
use tokio::sync::RwLock;
use tokio_stream::wrappers::IntervalStream;
use tonic::{Request, Response, Status};
use tonic::transport::{Identity, Server as TonicServer, ServerTlsConfig};
use tonic_reflection::server::Builder as ReflectionBuilder;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::body::Full;
use tower_http::LatencyUnit;
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

pub(crate) use utils::auto_route::route;

use crate::handlers::v1::health::{HealthServer, HealthServiceImpl};
// use crate::grpc::my_service::{MyRequest, MyResponse};
// use crate::grpc::my_service_server::{MyService, MyServiceServer};
use crate::http::routes;
use crate::models::ServerState;
use crate::operational_service_server::{OperationalService, OperationalServiceServer};

mod accept;
mod errors;
mod grpc;
mod handlers;
mod http;
mod models;
mod utils;

tonic::include_proto!("operational");

struct Operations {
    server_state: Arc<RwLock<ServerState>>,
}

async fn handle(request: Request<Full>) -> Result<Response<Full>, Infallible> {
    Ok(Response::new(Full::default()))
}

#[tonic::async_trait]
impl OperationalService for Operations {
    async fn get_uptime(&self, _request: Request<()>) -> Result<Response<Uptime>, Status> {
        let state = self.server_state.read().await;
        Ok(Response::new(Uptime {
            uptime: state.get_uptime_as_secs().await,
        }))
    }

    async fn get_metrics(&self, _request: Request<()>) -> Result<Response<Metrics>, Status> {
        let state = self.server_state.read().await;
        Ok(Response::new(Metrics {
            cpu_load: 0.0,
            memory_usage: 0,
            uptime: 0,
        }))
    }

    async fn get_application_status(
        &self, _request: Request<()>,
    ) -> Result<Response<ApplicationStatus>, Status> {
        let state = self.server_state.read().await;
        Ok(Response::new(ApplicationStatus {
            status: format!("Application Status: {:?}", state),
            message: "".to_string(),
            timestamp: 0,
        }))
    }
}

pub mod proto {
    pub(crate) const FILE_DESCRIPTOR_SET_V1: &[u8] = tonic::include_file_descriptor_set!("grpc.v1");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_filename(".env.local").ok();

    // Get the certificate paths
    let cert_paths = utils::CertificatePaths::new();

    // Use the paths as needed
    info!("Cert path: {}", cert_paths.cert_path);
    info!("Key path: {}", cert_paths.key_path);
    info!("CA Cert path: {}", cert_paths.ca_cert_path);

    // Set default values
    let default_rest_address = "0.0.0.0";
    let default_rest_port = 3000;
    let default_grpc_address = "0.0.0.0";
    let default_grpc_port = 50051;

    // Helper function to parse address and port
    fn parse_socket_addr(
        address_env: &str, port_env: &str, default_address: &str, default_port: u16,
    ) -> SocketAddr {
        let address = std::env::var(address_env).unwrap_or_else(|_| default_address.to_string());
        let port: u16 = std::env::var(port_env)
            .unwrap_or_else(|_| default_port.to_string())
            .parse()
            .expect("Invalid port");
        SocketAddr::new(address.parse().expect("Invalid address"), port)
    }

    // Read and parse environment variables or use defaults
    let rest_socket_addr =
        parse_socket_addr("REST_ADDRESS", "REST_PORT", default_rest_address, default_rest_port);
    let grpc_socket_addr =
        parse_socket_addr("GRPC_ADDRESS", "GRPC_PORT", default_grpc_address, default_grpc_port);

    // Initialize tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let start_time = Arc::new(RwLock::new(SystemTime::now()));
    let server_state = Arc::new(RwLock::new(ServerState::new(start_time.clone())));

    // REST router setup
    let rest_router = routes::create_router().layer(
        ServiceBuilder::new()
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(
                        DefaultOnResponse::new()
                            .level(Level::INFO)
                            .latency_unit(LatencyUnit::Micros),
                    ),
            )
            .layer(AddExtensionLayer::new(server_state.clone())),
    );

    // REST server setup with optional TLS
    let rest_server = if Path::new(&cert_paths.cert_path).exists() {
        let config = RustlsConfig::from_pem_file(&cert_paths.cert_path, &cert_paths.key_path)
            .await
            .unwrap();

        tokio::spawn(async move {
            info!("Starting REST server with TLS at https://127.0.0.1:3000");
            axum_server::bind_rustls(rest_socket_addr, config)
                .serve(rest_router.into_make_service())
                .await
                .expect("Failed to start REST server");
        })
    } else {
        tokio::spawn(async move {
            info!("Starting REST server without TLS at http://127.0.0.1:3000");
            axum_server::bind(rest_socket_addr)
                .serve(rest_router.into_make_service())
                .await
                .expect("Failed to start REST server");
        })
    };

    // gRPC server setup
    let operations = Operations {
        server_state: server_state.clone(),
    };

    let health_service = HealthServiceImpl {
        server_state: server_state.clone(),
    };

    let grpc_server = tokio::spawn(async move {
        let grpc_builder = TonicServer::builder();

        let grpc_server = if Path::new(&cert_paths.cert_path).exists() {
            let identity = Identity::from_pem(
                tokio::fs::read(&cert_paths.cert_path).await.unwrap(),
                tokio::fs::read(&cert_paths.key_path).await.unwrap(),
            );

            grpc_builder.tls_config(ServerTlsConfig::new().identity(identity))
        } else {
            Ok(grpc_builder)
        };

        grpc_server
            .expect("REASON")
            .add_service(
                tonic_reflection::server::Builder::configure()
                    .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET_V1)
                    .build()
                    .unwrap(),
            )
            .add_service(HealthServer::new(health_service))
            .add_service(OperationalServiceServer::new(operations))
            // .serve_with_incoming_shutdown(grpc_socket_addr, async {
            //     let mut interval = IntervalStream::new(tokio::time::interval(std::time::Duration::from_secs(1)));
            //     while interval.next().await.is_some() {
            //         info!("gRPC server is running");
            //     }
            // })
            .serve(grpc_socket_addr)
            .await
            .expect("Failed to start gRPC server");
    });

    // Wait for both servers to complete
    let _ = tokio::join!(rest_server, grpc_server);

    Ok(())
}
