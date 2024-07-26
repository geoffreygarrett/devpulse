mod annotation;
mod annotator;
mod azure;
mod bitbucket;
mod github;
mod gitlab;
mod service;

pub mod prelude {
    pub use crate::library::annotations::annotation::{Annotation, AnnotationLevel};
    pub use crate::library::annotations::annotator::Annotator;
    pub use crate::library::annotations::azure::azure_annotator::AzureAnnotator;
    pub use crate::library::annotations::bitbucket::bitbucket_annotator::BitbucketAnnotator;
    pub use crate::library::annotations::github::github_annotator::GitHubAnnotator;
    pub use crate::library::annotations::gitlab::gitlab_annotator::GitLabAnnotator;
    pub use crate::library::annotations::service::{AnnotationService, Platform};
}
