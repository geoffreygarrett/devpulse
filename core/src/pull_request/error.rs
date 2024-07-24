use snafu::Snafu;

#[derive(Debug, Snafu)]
#[allow(dead_code)]
pub enum PullRequestError {
    #[snafu(display("Failed to fetch PR data for ID {}: {}", pr_id, source))]
    FetchError { pr_id: u64, source: std::io::Error },
    #[snafu(display("Failed to parse PR data: {}", source))]
    ParseError { source: std::io::Error },
    #[snafu(display("Failed to segment PR data: {}", message))]
    SegmentationError { message: String },
    #[snafu(display("Error analyzing PR data: {}", message))]
    AnalysisError { message: String },
}
