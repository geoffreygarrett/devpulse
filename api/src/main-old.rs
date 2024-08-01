#![allow(warnings)]

use std::sync::Arc;
use std::time::SystemTime;

use tokio::sync::RwLock;
use tonic::{Request, Response, Status};
use tonic::transport::Server;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::trace::TraceLayer;

pub(crate) use utils::auto_route::route;

// use crate::grpc::my_service::{MyRequest, MyResponse};
// use crate::grpc::my_service_server::{MyService, MyServiceServer};
use crate::models::ServerState;
use crate::operational_service_server::{OperationalService, OperationalServiceServer};

mod accept;
mod errors;
mod grpc;
mod http;
mod models;
mod utils;

tonic::include_proto!("operational");

struct Operations {
    server_state: Arc<RwLock<ServerState>>,
}

impl OperationalService for Operations {
    async fn get_health_check(
        &self, request: Request<()>,
    ) -> Result<Response<HealthCheck>, Status> {
        todo!()
    }

    async fn get_uptime(&self, request: Request<()>) -> Result<Response<Uptime>, Status> {
        todo!()
    }

    async fn get_metrics(&self, request: Request<()>) -> Result<Response<Metrics>, Status> {
        todo!()
    }

    async fn get_application_status(
        &self, request: Request<()>,
    ) -> Result<Response<ApplicationStatus>, Status> {
        todo!()
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let start_time = Arc::new(RwLock::new(SystemTime::now()));
    let server_state = Arc::new(RwLock::new(ServerState::new(start_time)));

    let router = http::create_router().layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(AddExtensionLayer::new(server_state.clone())), // Cloning for potential gRPC use
    );

    // let greeter = MyGreeter::default();
    let operations = Operations {
        server_state: server_state.clone(),
    };

    let addr = "[::1]:50051".parse()?;

    Server::builder()
        .add_service(OperationalServiceServer::new(operations))
        .serve(addr)
        .await?;
    //     .add_service(GreeterServer::new(greeter))
    //     .serve(addr)
    //     .await?;
    //

    // Configure HTTP server
    // let http_addr = "127.0.0.1:3000";
    // let http_server = tokio::spawn(async move {
    //     axum::Server::bind(&http_addr.parse().unwrap())
    //         .serve(router.into_make_service())
    //         .await
    //         .expect("HTTP server failed to run");
    // });

    // // Configure gRPC server
    // let grpc_addr = "127.0.0.1:50051"; // Make sure this does not conflict with HTTP server
    // let grpc_server = tokio::spawn(async move {
    //     serve_grpc(grpc_addr.to_string())
    //         .await
    //         .expect("gRPC server failed to run");
    // });

    // // Wait for both servers to complete
    // let _ = tokio::join!(http_server, grpc_server);

    Ok(router.into())
}
