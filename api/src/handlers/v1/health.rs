use std::pin::Pin;
use std::sync::Arc;

use async_stream::stream;
use futures_util::Stream;
use tokio::sync::RwLock;
use tonic::{Request, Response, Status};

pub use health_server::{Health, HealthServer};

use crate::models::ServerState;

tonic::include_proto!("api.health.v1");

// use crate::prelude::*;
// use crate::grpc::health::v1::{
//     health_server::{Health, HealthServer},
//     HealthCheckRequest, HealthCheckResponse, health_check_response,
// };

/// `HealthServiceImpl` provides the implementation for the Health gRPC service.
///
/// This service includes two methods:
/// - `check`: A unary RPC to check the health status of a service.
/// - `watch`: A server-streaming RPC to continuously check the health status of a service.
pub struct HealthServiceImpl {
    pub(crate) server_state: Arc<RwLock<ServerState>>,
}

// /// Implement the Health service.
// impl HealthServiceImpl {
//     /// Create a new instance of `HealthServiceImpl`.
//     ///
//     /// # Arguments
//     ///
//     /// * `server_state` - A shared state containing server information.
//     ///
//     /// # Returns
//     ///
//     /// * `HealthServiceImpl` - A new instance of `HealthServiceImpl`.
//     // pub fn new(server_state: Arc<RwLock<ServerState>>) -> Self {
//     //     Self { server_state }
//     // }
// }

#[tonic::async_trait]
impl Health for HealthServiceImpl {
    /// Unary RPC to check the health status of a service.
    ///
    /// # Arguments
    ///
    /// * `request` - A `Request<HealthCheckRequest>` containing the service name to check.
    ///
    /// # Returns
    ///
    /// * `Result<Response<HealthCheckResponse>, Status>` - A response containing the health status of the service.
    ///
    /// # Example
    ///
    /// ```rust
    /// let response = health_service.check(request).await?;
    /// println!("Health status: {:?}", response.into_inner().status);
    /// ```
    async fn check(
        &self, request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        let _service = request.into_inner().service;

        // Implement logic to determine the health status of the service.
        // For simplicity, let's assume all services are SERVING.
        Ok(Response::new(HealthCheckResponse {
            status: health_check_response::ServingStatus::Serving.into(),
        }))
    }

    /// Server-streaming RPC type alias for the `watch` method.
    type WatchStream =
        Pin<Box<dyn Stream<Item = Result<HealthCheckResponse, Status>> + Send + Sync + 'static>>;

    /// Server-streaming RPC to continuously check the health status of a service.
    ///
    /// This method streams the health status of a service at regular intervals.
    ///
    /// # Arguments
    ///
    /// * `request` - A `Request<HealthCheckRequest>` containing the service name to check.
    ///
    /// # Returns
    ///
    /// * `Result<Response<Self::WatchStream>, Status>` - A response containing a stream of health statuses.
    ///
    /// # Example
    ///
    /// ```rust
    /// let response = health_service.watch(request).await?;
    /// let mut stream = response.into_inner();
    /// while let Some(status) = stream.message().await? {
    ///     println!("Health status: {:?}", status.status);
    /// }
    ///
    /// ```bash
    /// grpcurl -plaintext -d '{ "service": "test_service" }' localhost:50051 api.health.v1.Health/Watch
    /// ```
    /// ```
    async fn watch(
        &self, request: Request<HealthCheckRequest>,
    ) -> Result<Response<Self::WatchStream>, Status> {
        let _service = request.into_inner().service;
        let server_state = self.server_state.clone();

        let stream = stream! {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));

            loop {
                interval.tick().await;

                // Implement logic to determine the health status of the service.
                // For simplicity, let's assume all services are SERVING.
                let response = HealthCheckResponse {
                    status: health_check_response::ServingStatus::Serving.into(),
                };

                yield Ok(response);
            }
        };

        Ok(Response::new(Box::pin(stream) as Self::WatchStream))
    }
}
