// // src/grpc/server.rs
//
// use tonic::{transport::Server, Request, Response, Status};
//
// use crate::grpc::server::proto::dev_pulse_service_server::{
//     DevPulseService, DevPulseServiceServer,
// };
// use crate::grpc::server::proto::{
//     CommitRangeRequest, CommitRangeResponse, DeveloperPerformance, DeveloperPerformanceRequest,
// };
//
// pub mod proto {
//     tonic::include_proto!("devpulse");
// }
//
// #[derive(Default)]
// pub struct DevPulseServiceImpl;
// // https://docs.shuttle.rs/templates/tutorials/custom-service#getting-started
// // https://docs.shuttle.rs/templates/tutorials/custom-service#getting-started
//
// #[tonic::async_trait]
// impl DevPulseService for DevPulseServiceImpl {
//     async fn analyze_commit_range(
//         &self, request: Request<CommitRangeRequest>,
//     ) -> Result<Response<CommitRangeResponse>, Status> {
//         // Implement your business logic here.
//         // For now, returning an empty response.
//         let response = CommitRangeResponse {
//             repository: request.into_inner().repository_url,
//             commit_range: None, // Add proper CommitRangeDetails
//         };
//         Ok(Response::new(response))
//     }
//
//     async fn get_developer_performance(
//         &self, request: Request<DeveloperPerformanceRequest>,
//     ) -> Result<Response<DeveloperPerformance>, Status> {
//         // Implement your business logic here.
//         // For now, returning an empty response.
//         let response = DeveloperPerformance {
//             username: request.into_inner().username,
//             total_commits: 0,
//             total_prs: 0,
//             average_time_to_merge: String::new(),
//             repositories: Vec::new(),
//         };
//         Ok(Response::new(response))
//     }
// }
//
// pub async fn start_grpc_server() -> Result<(), Box<dyn std::error::Error>> {
//     let addr = "[::1]:50051".parse().unwrap();
//     let devpulse_service = DevPulseServiceImpl::default();
//
//     Server::builder()
//         .add_service(DevPulseServiceServer::new(devpulse_service))
//         .serve(addr)
//         .await?;
//
//     Ok(())
// }
