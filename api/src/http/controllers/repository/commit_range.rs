use axum::{extract::Query, Json, response::IntoResponse};
use axum::body::Body;
use axum::http::HeaderMap;
use axum::response::Response;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use devpulse_core::services::analyze_commit_range_service;

use crate::accept::serialize_response;
use crate::models::{
    BadRequest, CommitRangeAnalysisResponse, CommitRangeRequest, InternalServerError,
    ResponseDetail, TooManyRequests, Unauthorized,
};
use crate::route;

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[into_params(style = Form, parameter_in = Query)]
pub struct ResponseDetailQuery {
    #[serde(rename = "Detail")]
    detail: Option<ResponseDetail>, // Optional. The level of detail: simple, detailed. Default is simple.
}

#[allow(dead_code)]
impl Default for ResponseDetailQuery {
    fn default() -> Self {
        Self {
            detail: Some(ResponseDetail::Simple),
        }
    }
}

/// Commit Range
///
/// Create a new analysis for a repository in the commit range, if the analysis exists,
/// it will be returned.
#[route(
    put,
    path = "/repository/commit-range",
    operation_id = "/repository/create-commit-range-analysis",
    // params(
    //     ResponseDetailQuery
    // ),
    responses(
        (status = 200, response = CommitRangeAnalysisResponse),
        (status = 400, response = BadRequest),
        (status = 401, response = Unauthorized),
        (status = 429, response = TooManyRequests),
        (status = 500, response = InternalServerError),
    ),
    request_body(
        content = CommitRangeRequest,
        description = "The repository URL and commit range to analyze",
        content_type = "application/json",
    ),
    tag = "Repository",
)]
pub async fn create_commit_range_analysis(
    headers: HeaderMap, params: Option<Query<ResponseDetailQuery>>,
    Json(payload): Json<CommitRangeRequest>,
) -> Response<Body> {
    let _params = params.unwrap_or_default();
    match analyze_commit_range_service(
        &payload.repository.into(),
        &payload.start_commit,
        &payload.end_commit,
    )
    .await
    {
        Ok(result) => serialize_response(&result, &headers),
        Err(err) => {
            let error_message = format!("Analysis error: {}", err);
            InternalServerError::new(&error_message).into_response()
        }
    }
}
