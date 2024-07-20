use axum::{Json, Router, routing::get};
use axum::extract::Path;
use axum::http::StatusCode;

use devpulse_core::git::{CodeChurn, process_repository_v1};

async fn hello_world() -> &'static str {
    "Hello, world!"
}
async fn process_repository(
    Path((old_commit, new_commit)): Path<(String, String)>,
) -> (StatusCode, Json<Vec<CodeChurn>>) {
    let url = "https://github.com/tudat-team/tudatpy";
    match process_repository_v1(&url, &old_commit, &new_commit) {
        Ok(churns) => (StatusCode::OK, Json(churns)),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])),
    }
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/process-repository/v1/:old_commit/:new_commit", get(process_repository));
    Ok(router.into())
}
