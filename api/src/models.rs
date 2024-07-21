use serde::{Serialize, Deserialize};
use utoipa::{ToSchema};

/// DTO representing a request to analyze a range of commits within a repository.
///
/// This struct is used to transfer data from the client to the server
/// when requesting an analysis of a specific commit range in a repository.
///
/// # Fields
/// - `repository_url`: The URL of the repository to analyze.
/// - `start_commit`: The hash of the starting commit for the analysis.
/// - `end_commit`: The hash of the ending commit for the analysis.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct CommitRangeRequest {
    #[schema(example = "https://github.com/bazelbuild/rules_rust")]
    pub repository_url: String,
    #[schema(example = "6c2bd67")]
    pub start_commit: String,
    #[schema(example = "6b10ce3")]
    pub end_commit: String,
}

/// DTO representing the response from analyzing a range of commits within a repository.
///
/// This struct is used to transfer data from the server to the client
/// with the results of the commit range analysis.
///
/// # Fields
/// - `repository`: The name or URL of the repository analyzed.
/// - `commit_range`: The details of the analyzed commit range.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct CommitRangeResponse {
    pub repository: String,
    pub commit_range: CommitRangeDetails,
}

/// DTO representing the detailed results of a commit range analysis.
///
/// This struct encapsulates the detailed results of analyzing a range of commits,
/// including total commits, additions, deletions, and top contributors.
///
/// # Fields
/// - `start_commit`: The hash of the starting commit for the analysis.
/// - `end_commit`: The hash of the ending commit for the analysis.
/// - `total_commits`: The total number of commits in the range.
/// - `total_additions`: The total number of lines added in the range.
/// - `total_deletions`: The total number of lines deleted in the range.
/// - `top_contributors`: A list of the top contributors in the commit range.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct CommitRangeDetails {
    pub start_commit: String,
    pub end_commit: String,
    pub total_commits: i32,
    pub total_additions: i32,
    pub total_deletions: i32,
    pub top_contributors: Vec<Contributor>,
}

/// DTO representing a contributor's information in a commit range analysis.
///
/// This struct represents the details of a contributor in the analyzed commit range,
/// including the username and the number of commits.
///
/// # Fields
/// - `username`: The username of the contributor.
/// - `commits`: The number of commits made by the contributor in the commit range.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct Contributor {
    pub username: String,
    pub commits: i32,
}

/// DTO representing a developer's performance metrics.
///
/// This struct is used to transfer data from the server to the client
/// with the performance metrics of a specific developer.
///
/// # Fields
/// - `username`: The username of the developer.
/// - `total_commits`: The total number of commits made by the developer.
/// - `total_prs`: The total number of pull requests made by the developer.
/// - `average_time_to_merge`: The average time taken to merge the developer's pull requests.
/// - `repositories`: A list of repositories the developer has contributed to.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct DeveloperPerformance {
    pub username: String,
    pub total_commits: i32,
    pub total_prs: i32,
    pub average_time_to_merge: String,
    pub repositories: Vec<RepositoryContribution>,
}

/// DTO representing a developer's contributions to a repository.
///
/// This struct represents the contributions made by a developer to a specific repository,
/// including the URL of the repository and the number of commits made.
///
/// # Fields
/// - `url`: The URL of the repository.
/// - `commits`: The number of commits made by the developer in the repository.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct RepositoryContribution {
    pub url: String,
    pub commits: i32,
}
