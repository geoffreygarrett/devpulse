// use std::sync::Arc;
// use axum::{Extension, extract::Query, Json};
// use axum::body::Body;
// use axum::http::HeaderMap;
// use axum::response::Response;
// use serde::Deserialize;
// use utoipa::{IntoParams, ToSchema};
// use crate::models::{
//     BadRequest, InternalServerError,
//     ResponseDetail, SummarizeCommitRequest, SummarizedCommitResponse, TooManyRequests,
//     Unauthorized,
// };
// use crate::route;
//
// #[derive(Debug, Deserialize, IntoParams, ToSchema)]
// #[into_params(style = Form, parameter_in = Query)]
// pub struct ResponseDetailQuery {
//     #[serde(rename = "Detail")]
//     detail: Option<ResponseDetail>, // Optional. The level of detail: simple, detailed. Default is simple.
// }
//
// #[allow(dead_code)]
// impl Default for ResponseDetailQuery {
//     fn default() -> Self {
//         Self {
//             detail: Some(ResponseDetail::Simple),
//         }
//     }
// }
//
// /// Commit Range
// ///
// /// Create a new analysis for a repository in the commit range, if the analysis exists,
// /// it will be returned.
// #[route(
//     get,
//     path = "/repository/commit/summarize",
//     operation_id = "/repository/summarize-commit",
// // params(
// //     ResponseDetailQuery
// // ),
//     responses(
//         (status = 200, response = SummarizedCommitResponse),
//         (status = 400, response = BadRequest),
//         (status = 401, response = Unauthorized),
//         (status = 429, response = TooManyRequests),
//         (status = 500, response = InternalServerError),
//     ),
//     request_body(
//         content = GetCommitRequest,
//         description = "The repository URL and commit range to analyze",
//         content_type = "application/json",
//     ),
//     tag = "Repository",
// )]
// pub async fn summarize_commit(
//     Extension(state): Extension<Arc<AppState>>,
//     headers: HeaderMap, params: Option<Query<ResponseDetailQuery>>,
//     Json(payload): Json<SummarizeCommitRequest>,
// ) -> Response<Body> {
//     devpulse_core::services::summarize_commit_service(headers, params, payload).await.into_response()
// }
