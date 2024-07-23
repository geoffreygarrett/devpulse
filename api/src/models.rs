use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use utoipa::{ToResponse, ToSchema};

const APPLICATION_VND_DEVPULSE_V1_JSON: &str = "application/vnd.devpulse.v1+json";
const APPLICATION_VND_DEVPULSE_V1_YAML: &str = "application/vnd.devpulse.v1+yaml";
const APPLICATION_VND_DEVPULSE_V1_TOML: &str = "application/vnd.devpulse.v1+toml";
const APPLICATION_VND_DEVPULSE_V1_XML: &str = "application/vnd.devpulse.v1+xml";

/// Represents a request to analyze a specific commit range in a repository.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct CommitRangeRequest {
    #[schema(example = "https://github.com/bazelbuild/rules_rust")]
    pub repository_url: String,
    #[schema(example = "6c2bd67")]
    pub start_commit: String,
    #[schema(example = "6b10ce3")]
    pub end_commit: String,
}

/// Represents the response containing the results from analyzing a commit range.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct CommitRangeAnalysis {
    pub repository: String,
    pub commit_range: CommitRangeDetails,
}

impl CommitRangeAnalysis {
    pub fn new(repository: &str, commit_range: CommitRangeDetails) -> Self {
        CommitRangeAnalysis {
            repository: repository.to_string(),
            commit_range,
        }
    }
}

// #[derive(utoipa::ToResponse)]
// enum Person {
//     #[response(examples(
//         ("Person1" = (value = json!({"name": "name1"}))),
//         ("Person2" = (value = json!({"name": "name2"})))
//     ))]
//     Admin(#[content("application/vnd-custom-v1+json")] Admin),
//
//     #[response(example = json!({"name": "name3", "id": 1}))]
//     Admin2(
//         #[content("application/vnd-custom-v2+json")]
//         #[to_schema]
//         Admin2,
//     ),
// }

/// Details the results of a commit range analysis, including commits, additions, deletions, and contributors.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct CommitRangeDetails {
    pub start_commit: String,
    pub end_commit: String,
    pub total_commits: i32,
    pub total_additions: i32,
    pub total_deletions: i32,
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
