// use std::sync::Arc;
// use tonic::{transport::Server, Request, Response, Status};
//
// use crate::services::{AuthService, AccessTokenService};
//
// // Import the generated gRPC code
// // Assuming you have a proto file defining your services
// pub mod rustproof {
//     tonic::include_proto!("rustproof");
// }
//
// use rustproof::auth_server::{Auth, AuthServer};
// use rustproof::{SignUpRequest, SignUpResponse, LoginRequest, LoginResponse};
//
// // Implement the gRPC service
// pub struct RustProofAuthService {
//     auth_service: Arc<dyn AuthService + Send + Sync>,
//     token_service: Arc<dyn AccessTokenService + Send + Sync>,
// }
//
// #[tonic::async_trait]
// impl Auth for RustProofAuthService {
//     async fn sign_up(&self, request: Request<SignUpRequest>) -> Result<Response<SignUpResponse>, Status> {
//         let req = request.into_inner();
//         // Implement sign up logic using self.auth_service
//         // This is a placeholder implementation
//         Ok(Response::new(SignUpResponse {
//             user_id: "new_user_id".to_string(),
//             message: "User signed up successfully".to_string(),
//         }))
//     }
//
//     async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
//         let req = request.into_inner();
//         // Implement login logic using self.auth_service and self.token_service
//         // This is a placeholder implementation
//         Ok(Response::new(LoginResponse {
//             token: "access_token".to_string(),
//             user_id: "user_id".to_string(),
//         }))
//     }
//
//     // Implement other RPC methods defined in your proto file...
// }
//
// pub fn build_grpc_service(
//     auth_service: Arc<dyn AuthService + Send + Sync>,
//     token_service: Arc<dyn AccessTokenService + Send + Sync>,
// ) -> AuthServer<RustProofAuthService> {
//     let grpc_service = RustProofAuthService {
//         auth_service,
//         token_service,
//     };
//
//     AuthServer::new(grpc_service)
// }
//
// // Helper function to run the gRPC server
// pub async fn run_grpc_server(addr: std::net::SocketAddr, auth_service: AuthServer<RustProofAuthService>) {
//     println!("gRPC server listening on {}", addr);
//
//     Server::builder()
//         .add_service(auth_service)
//         .serve(addr)
//         .await
//         .unwrap();
// }