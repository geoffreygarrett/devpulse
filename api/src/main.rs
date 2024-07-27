#![allow(warnings)]

use std::sync::Arc;
use std::time::SystemTime;

use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::trace::TraceLayer;

pub(crate) use utils::auto_route::route;

// use crate::grpc::my_service::{MyRequest, MyResponse};
// use crate::grpc::my_service_server::{MyService, MyServiceServer};
use crate::models::ServerState;

mod accept;
#[allow(unused_imports)]
mod conversion;
mod errors;
mod grpc;
mod http;
mod models;
mod utils;

// mod models {
//     // mod devpulse {
//     include!(concat!(env!("OUT_DIR"), "/devpulse.rs"));
//     // }
//
//     // use devpulse::*;
// }
// pub mod my_service {
//     tonic::include_proto!("devpulse");
// }

// pub struct MyGrpcService;
//
// #[tonic::async_trait]
// impl MyService for MyGrpcService {
//     async fn my_function(
//         &self, request: Request<MyRequest>,
//     ) -> Result<Response<MyResponse>, Status> {
//         println!("Received request: {:?}", request);
//
//         let response = MyResponse {
//             message: format!("Hello {}", request.into_inner().name),
//         };
//
//         Ok(Response::new(response))
//     }
// }
//
// async fn serve_grpc(addr: String) -> Result<(), Box<dyn std::error::Error>> {
//     let service = MyServiceServer::new(MyGrpcService);
//
//     Server::builder()
//         .add_service(service)
//         .serve(addr.parse()?)
//         .await?;
//
//     Ok(())
// }
//
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let start_time = Arc::new(RwLock::new(SystemTime::now()));
    let server_state = ServerState::new(start_time);

    let router = http::create_router().layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(AddExtensionLayer::new(server_state.clone())), // Cloning for potential gRPC use
    );

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
