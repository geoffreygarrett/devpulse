// use db_auth_v1::{RefreshToken, RefreshTokenCreate, RevokeRefreshTokenRequest};
// use db_auth_v1::refresh_token_service_server::{RefreshTokenService, RefreshTokenServiceServer};
// use tonic::{Request, Response, Status};
//
// #[cfg(feature = "server")]
// pub struct RefreshTokenServiceImpl;
//
// #[cfg(feature = "server")]
// #[tonic::async_trait]
// impl RefreshTokenService for RefreshTokenServiceImpl {
//     async fn create_refresh_token(
//         &self, request: Request<RefreshTokenCreate>,
//     ) -> Result<Response<RefreshToken>, Status> {
//         // Add logic to create a refresh token in the database
//         Ok(Response::new(RefreshToken {
//             id: 1,
//             account_id: request.into_inner().account_id,
//             issued_at: "2022-01-01T00:00:00Z".into(),
//             expires: "2022-01-02T00:00:00Z".into(),
//             revoked: false,
//             revocation_time: "".into(),
//             token: request.into_inner().token,
//         }))
//     }
//
//     async fn get_refresh_token(
//         &self, request: Request<RefreshToken>,
//     ) -> Result<Response<RefreshToken>, Status> {
//         // Add logic to get a refresh token from the database
//         Ok(Response::new(request.into_inner()))
//     }
//
//     async fn revoke_refresh_token(
//         &self, request: Request<RevokeRefreshTokenRequest>,
//     ) -> Result<Response<RefreshToken>, Status> {
//         // Add logic to revoke a refresh token in the database
//         let req = request.into_inner();
//         // Assuming some logic here to revoke the token in your DB
//         Ok(Response::new(RefreshToken {
//             id: 1, // This should be fetched from the database
//             account_id: req.account_id,
//             issued_at: "2022-01-01T00:00:00Z".into(),
//             expires: "2022-01-02T00:00:00Z".into(),
//             revoked: true,
//             revocation_time: "2022-01-01T12:00:00Z".into(),
//             token: req.token,
//         }))
//     }
// }
