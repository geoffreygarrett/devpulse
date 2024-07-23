use std::sync::Arc;
use std::time::SystemTime;

use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::sync::RwLock;
use utoipa::{ToResponse, ToSchema};

const APPLICATION_VND_DEVPULSE_V1_JSON: &str = "application/vnd.devpulse.v1+json";
const APPLICATION_VND_DEVPULSE_V1_YAML: &str = "application/vnd.devpulse.v1+yaml";
const APPLICATION_VND_DEVPULSE_V1_TOML: &str = "application/vnd.devpulse.v1+toml";
const APPLICATION_VND_DEVPULSE_V1_XML: &str = "application/vnd.devpulse.v1+xml";

/// Represents a request to analyze a specific commit range in a repository.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct CommitRangeRequest {
    #[schema(example = json!({
        "type": "github",
        "owner": "bazelbuild",
        "name": "rules_rust"
    }))]
    pub repository: Repository,
    #[schema(example = "6c2bd67")]
    pub start_commit: String,
    #[schema(example = "6b10ce3")]
    pub end_commit: String,
}

/// Represents the response containing the results from analyzing a commit range.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct CommitRangeAnalysis {
    #[schema(example = json!({
        "type": "github",
        "owner": "bazelbuild",
        "name": "rules_rust"
    }))]
    pub repository: Repository,
    #[schema(example = json!({
        "start_commit": "6c2bd67",
        "end_commit": "6b10ce3",
        "total_commits": 6,
        "total_additions": 1163,
        "total_deletions": 59,
        "top_contributors": [
            {"username": "Daniel Wagner-Hall", "commits": 1144},
            {"username": "Milan Vukov", "commits": 60},
            {"username": "Marcel Hlopko", "commits": 18}
        ]
    }))]
    pub commit_range: CommitRangeDetails,
}

impl CommitRangeAnalysis {
    pub fn new(repository: &Repository, commit_range: CommitRangeDetails) -> Self {
        CommitRangeAnalysis {
            repository: repository.clone(),
            commit_range,
        }
    }
}

/// Details the results of a commit range analysis, including commits, additions, deletions, and contributors.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct CommitRangeDetails {
    #[schema(example = "6c2bd67")]
    pub start_commit: String,
    #[schema(example = "6b10ce3")]
    pub end_commit: String,
    #[schema(example = "6")]
    pub total_commits: i32,
    #[schema(example = "1163")]
    pub total_additions: i32,
    #[schema(example = "59")]
    pub total_deletions: i32,
    #[schema(example = json!([
        {"username": "Daniel Wagner-Hall", "commits": 1144},
        {"username": "Milan Vukov", "commits": 60},
        {"username": "Marcel Hlopko", "commits": 18}
    ]))]
    pub top_contributors: Vec<Contributor>,
}

/// Represents a contributor's information within a commit range analysis.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct Contributor {
    pub username: String,
    pub commits: i32,
}

/// Represents a developer's performance metrics.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct DeveloperPerformanceAnalysis {
    pub username: String,
    pub total_commits: i32,
    pub total_prs: i32,
    pub average_time_to_merge: String,
    pub repositories: Vec<RepositoryContribution>,
}

impl DeveloperPerformanceAnalysis {
    pub fn new(
        username: &str, total_commits: i32, total_prs: i32, average_time_to_merge: &str,
        repositories: Vec<RepositoryContribution>,
    ) -> Self {
        DeveloperPerformanceAnalysis {
            username: username.to_string(),
            total_commits,
            total_prs,
            average_time_to_merge: average_time_to_merge.to_string(),
            repositories,
        }
    }
}

#[derive(ToResponse)]
pub(crate) enum DeveloperPerformanceAnalysisResponse {
    Json(#[content("application/vnd.devpulse.v1+json")] DeveloperPerformanceAnalysis),
    Yaml(#[content("application/vnd.devpulse.v1+yaml")] DeveloperPerformanceAnalysis),
    Toml(#[content("application/vnd.devpulse.v1+toml")] DeveloperPerformanceAnalysis),
    Xml(#[content("application/vnd.devpulse.v1+xml")] DeveloperPerformanceAnalysis),
}

/// Represents a developer's contributions to a repository.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct RepositoryContribution {
    pub url: String,
    pub commits: i32,
}

/// Enum for specifying the detail level of a response.
#[derive(Debug, Deserialize, ToSchema)]
pub(crate) enum ResponseDetail {
    Simple,
    Detailed,
}

/// Enum for specifying the response format.
#[derive(Debug, Deserialize, ToSchema)]
pub(crate) enum ResponseFormat {
    Json,
    Xml,
    Yaml,
}
// headers:
// X-RateLimit-Limit:
// schema:
// type: integer
// description: The number of allowed requests in the current period
// X-RateLimit-Remaining:
// schema:
// type: integer
// description: The number of remaining requests in the current period
// X-RateLimit-Reset:
// schema:
// type: integer
// description: The number of seconds left in the current period

macro_rules! impl_into_response {
    ($type:ty) => {
        impl IntoResponse for $type {
            fn into_response(self) -> Response {
                let json = serde_json::to_string(&self).unwrap();
                (*Self::CODE, json).into_response()
                // Response::builder()
                //     .status(*Self::CODE)
                //     .header("content-type", APPLICATION_VND_DEVPULSE_V1_JSON)
                //     .body(serde_json::to_string(&self).unwrap())
                //     .unwrap()
            }
        }
    };
}

/// Represents a rate limit error response with retry information.
#[derive(ToResponse, ToSchema, Serialize)]
#[response(
    description = "Too Many Requests",
    content_type = APPLICATION_VND_DEVPULSE_V1_JSON,
    headers(
    // https://github.com/OAI/OpenAPI-Specification/issues/1586
    ("X-RateLimit-Limit", description = "The number of allowed requests in the current period"),
    ("X-RateLimit-Remaining", description = "The number of remaining requests in the current period"),
    ("X-RateLimit-Reset", description = "The number of seconds left in the current period")
    )
)]
pub struct TooManyRequests {
    pub message: String,
    pub retry_after: Option<i32>, // Retry time in seconds
}

impl TooManyRequests {
    const CODE: &'static StatusCode = &StatusCode::TOO_MANY_REQUESTS;
    pub fn new(retry_after_seconds: Option<i32>) -> Self {
        TooManyRequests {
            message: "You have made too many requests. Please try again later.".to_string(),
            retry_after: retry_after_seconds,
        }
    }
}

// impl_into_response!(TooManyRequests);
impl IntoResponse for TooManyRequests {
    fn into_response(self) -> Response {
        let json = serde_json::to_string(&self).unwrap();
        Response::builder()
            .status(*Self::CODE)
            .header("content-type", APPLICATION_VND_DEVPULSE_V1_JSON)
            .header("X-RateLimit-Limit", "100")
            .header("X-RateLimit-Remaining", "0")
            .body::<axum::body::Body>(axum::body::Body::from(json))
            .unwrap()
    }
}

// impl IntoResponse for TooManyRequests {
//     fn into_response(self) -> Response {
//         let mut response = (StatusCode::TOO_MANY_REQUESTS, Json(&self)).into_response();
//         if let Some(retry_after) = self.retry_after {
//             response
//                 .headers_mut()
//                 .insert("X-RateLimit-Reset", format!("{}", retry_after).parse().unwrap());
//         }
//         response
//     }
// }

/// Bad request error response.
#[derive(ToResponse, ToSchema, Serialize)]
#[response(description = "Bad Request", content_type = APPLICATION_VND_DEVPULSE_V1_JSON)]
pub struct BadRequest {
    pub message: String,
}

impl BadRequest {
    const CODE: &'static StatusCode = &StatusCode::BAD_REQUEST;
    pub fn new(message: &str) -> Self {
        BadRequest {
            message: message.to_string(),
        }
    }
}

impl_into_response!(BadRequest);

/// Unauthorized error response.
#[derive(ToResponse, ToSchema, Serialize)]
#[response(description = "Unauthorized", content_type = APPLICATION_VND_DEVPULSE_V1_JSON)]
pub struct Unauthorized {
    pub message: String,
}

impl Unauthorized {
    const CODE: &'static StatusCode = &StatusCode::UNAUTHORIZED;
    pub fn new(message: &str) -> Self {
        #[allow(unused)]
        Unauthorized {
            message: message.to_string(),
        }
    }
}

impl_into_response!(Unauthorized);

#[derive(ToResponse, ToSchema, Serialize)]
#[response(
    description = "Internal Server Error",
    content_type = APPLICATION_VND_DEVPULSE_V1_JSON
)]
pub struct InternalServerError {
    pub message: String,
}

impl InternalServerError {
    const CODE: &'static StatusCode = &StatusCode::INTERNAL_SERVER_ERROR;
    #[allow(unused)]
    pub fn new(message: &str) -> Self {
        InternalServerError {
            message: message.to_string(),
        }
    }
}

impl_into_response!(InternalServerError);

#[derive(ToResponse, ToSchema, Serialize)]
#[response(
    description = "Not Implemented",
    content_type = APPLICATION_VND_DEVPULSE_V1_JSON
)]
pub struct NotImplemented {
    #[schema(example = "This feature is not implemented yet.")]
    pub message: String,
}

impl NotImplemented {
    const CODE: &'static StatusCode = &StatusCode::NOT_IMPLEMENTED;
    pub fn new(message: &str) -> Self {
        NotImplemented {
            message: message.to_string(),
        }
    }
}

impl_into_response!(NotImplemented);

#[derive(Clone)]
pub struct ServerState {
    start_time: Arc<RwLock<SystemTime>>,
}

impl ServerState {
    pub fn new(start_time: Arc<RwLock<SystemTime>>) -> Self {
        ServerState { start_time }
    }

    pub async fn get_uptime_as_secs(&self) -> u64 {
        self.start_time
            .read()
            .await
            .elapsed()
            .unwrap_or_default()
            .as_secs()
    }
}

#[derive(Serialize, ToSchema, ToResponse)]
#[response(
    description = "Successful health check",
    content_type = "application/vnd.devpulse.v1+json",
    example = json!({
        "status": "ok",
        "uptime": 123456
    })
)]
pub struct HealthCheckResponse {
    status: String,
    uptime: u64,
}

impl HealthCheckResponse {
    pub fn new(status: &str, uptime: u64) -> Self {
        HealthCheckResponse {
            status: status.to_string(),
            uptime,
        }
    }
}

impl IntoResponse for HealthCheckResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Serialize, ToSchema, ToResponse)]
#[response(
    description = "Source Version",
    content_type = "application/vnd.devpulse.v1+json",
    example = json!({
        "version": "0.1.0"
    })
)]
pub(crate) struct SourceVersionResponse {
    version: String,
}

impl SourceVersionResponse {
    pub fn new(version: &str) -> Self {
        SourceVersionResponse {
            version: version.to_string(),
        }
    }
}

impl IntoResponse for SourceVersionResponse {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type")]
pub enum Repository {
    #[serde(rename = "github")]
    #[schema(
        title = "GitHubRepository",
        example = json!({
            "type": "github",
            "owner": "bazelbuild",
            "name": "rules_rust"
        })
    )]
    GitHub { owner: String, name: String },
    #[serde(rename = "gitlab")]
    #[schema(
        title = "GitLabRepository",
        example = json!({
            "type": "gitlab",
            "owner": "gitlab-org",
            "name": "gitlab"
        })
    )]
    GitLab { owner: String, name: String },
    #[serde(rename = "bitbucket")]
    #[schema(
        title = "BitbucketRepository",
        example = json!({
            "type": "bitbucket",
            "owner": "atlassian",
            "name": "pyramid"
        })
    )]
    Bitbucket { owner: String, name: String },
    #[serde(rename = "azure_repos")]
    #[schema(
        title = "AzureRepositories",
        example = json!({
            "type": "azure_repos",
            "organization": "Microsoft",
            "project": "vscode",
            "repository": "vscode"
        })
    )]
    AzureRepos {
        organization: String,
        project: String,
        repository: String,
    },
    #[serde(rename = "custom")]
    #[schema(
        title = "CustomRepository",
        example = json!({
            "type": "custom",
            "url": "https://custom.repo/url"
        })
    )]
    Custom { url: String },
}
