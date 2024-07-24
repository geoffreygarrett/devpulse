use std::env;

use snafu::{ResultExt, Snafu};

use super::annotation::Annotation;
use super::annotator::{Annotator, AnnotatorError};
use super::azure::azure_annotator::AzureAnnotator;
use super::bitbucket::bitbucket_annotator::BitbucketAnnotator;
use super::github::github_annotator::GitHubAnnotator;
use super::gitlab::gitlab_annotator::GitLabAnnotator;

#[derive(Debug, Snafu)]
pub enum AnnotationServiceError {
    #[snafu(display("Unknown platform"))]
    UnknownPlatform,

    #[snafu(display("Failed to issue annotation: {}", source))]
    IssueAnnotation { source: AnnotatorError },
}

pub type Result<T, E = AnnotationServiceError> = std::result::Result<T, E>;

#[derive(PartialEq, Debug)]
pub enum Platform {
    GitHub,
    Azure,
    GitLab,
    Bitbucket,
}

impl Platform {
    pub fn from_env() -> Self {
        match env::var("CI_PLATFORM")
            .unwrap_or_else(|_| "github".to_string())
            .to_lowercase()
            .as_str()
        {
            "github" => Platform::GitHub,
            "azure" => Platform::Azure,
            "gitlab" => Platform::GitLab,
            "bitbucket" => Platform::Bitbucket,
            _ => Platform::GitHub,
        }
    }
}

pub struct AnnotationService {
    annotator: Box<dyn Annotator>,
}

impl AnnotationService {
    pub fn new() -> Result<Self> {
        let platform = Platform::from_env();
        let annotator: Box<dyn Annotator> = Self::create_annotator(platform);

        Ok(Self { annotator })
    }

    fn create_annotator(platform: Platform) -> Box<dyn Annotator> {
        match platform {
            Platform::GitHub => Box::new(GitHubAnnotator),
            Platform::Azure => Box::new(AzureAnnotator),
            Platform::GitLab => Box::new(GitLabAnnotator),
            Platform::Bitbucket => Box::new(BitbucketAnnotator),
        }
    }

    pub fn new_with_platform(platform: Platform) -> Result<Self> {
        let annotator: Box<dyn Annotator> =
            Self::create_annotator(platform);

        Ok(Self { annotator })
    }

    pub fn issue_annotations(&self, annotations: Vec<Annotation>) -> Result<()> {
        self.annotator
            .issue_annotations(annotations)
            .context(IssueAnnotationSnafu)
    }

    pub fn issue_annotation(&self, annotation: Annotation) -> Result<()> {
        self.annotator
            .issue_annotation(annotation)
            .context(IssueAnnotationSnafu)
    }

    pub fn get_annotation_string(&self, annotation: &Annotation) -> String {
        self.annotator.get_annotation_string(annotation)
    }

    pub fn get_annotations_strings(&self, annotations: &[Annotation]) -> Vec<String> {
        self.annotator.get_annotations_strings(annotations)
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::Once;

    use crate::annotations::annotation::{Annotation, AnnotationType};

    use super::*;

    // Ensure env variables are set only once for all tests
    static INIT: Once = Once::new();

    fn setup() {
        INIT.call_once(|| {
            env::set_var("CI_PLATFORM", "github");
        });
    }

    #[test]
    fn test_annotation_service_new_github() {
        let service = AnnotationService::new_with_platform(Platform::GitHub).unwrap();
        assert_eq!(
            service.get_annotation_string(&dummy_annotation(AnnotationType::Notice)),
            "::notice file=dummy.rs,line=1,endLine=1,col=1,endColumn=1::This is a dummy annotation"
        );
    }

    #[test]
    fn test_annotation_service_new_azure() {
        let service = AnnotationService::new_with_platform(Platform::Azure).unwrap();
        assert_eq!(
            service.get_annotation_string(&dummy_annotation(AnnotationType::Warning)),
            "##vso[task.logissue type=warning;sourcepath=dummy.rs;linenumber=1;columnnumber=1;endcolumnnumber=1;]This is a dummy annotation"
        );
    }

    #[test]
    fn test_annotation_service_new_gitlab() {
        let service = AnnotationService::new_with_platform(Platform::GitLab).unwrap();
        assert_eq!(
            service.get_annotation_string(&dummy_annotation(AnnotationType::Error)),
            "error in file: dummy.rs on line: 1, column: 1. This is a dummy annotation"
        );
    }

    #[test]
    fn test_annotation_service_new_bitbucket() {
        let service = AnnotationService::new_with_platform(Platform::Bitbucket).unwrap();
        assert_eq!(
            service.get_annotation_string(&dummy_annotation(AnnotationType::Error)),
            "Annotation: error: This is a dummy annotation in file: dummy.rs on line: 1"
        );
    }

    #[test]
    fn test_issue_annotations() {
        let service = AnnotationService::new_with_platform(Platform::GitHub).unwrap();
        let annotations = vec![
            dummy_annotation(AnnotationType::Notice),
            dummy_annotation(AnnotationType::Notice),
        ];
        assert!(service.issue_annotations(annotations).is_ok());
    }

    #[test]
    fn test_issue_annotation() {
        let service = AnnotationService::new_with_platform(Platform::GitHub).unwrap();
        let annotation = dummy_annotation(AnnotationType::Notice);
        assert!(service.issue_annotation(annotation).is_ok());
    }

    #[test]
    fn test_get_annotation_string() {
        let service = AnnotationService::new_with_platform(Platform::GitHub).unwrap();
        let annotation = dummy_annotation(AnnotationType::Notice);
        assert_eq!(
            service.get_annotation_string(&annotation),
            "::notice file=dummy.rs,line=1,endLine=1,col=1,endColumn=1::This is a dummy annotation"
        );
    }

    #[test]
    fn test_get_annotations_strings() {
        let service = AnnotationService::new_with_platform(Platform::GitHub).unwrap();
        let annotations = vec![
            dummy_annotation(AnnotationType::Notice),
            dummy_annotation(AnnotationType::Notice),
        ];
        let annotation_strings = service.get_annotations_strings(&annotations);
        assert_eq!(annotation_strings.len(), 2);
        assert_eq!(
            annotation_strings[0],
            "::notice file=dummy.rs,line=1,endLine=1,col=1,endColumn=1::This is a dummy annotation"
        );
        assert_eq!(
            annotation_strings[1],
            "::notice file=dummy.rs,line=1,endLine=1,col=1,endColumn=1::This is a dummy annotation"
        );
    }

    fn dummy_annotation(annotation_type: AnnotationType) -> Annotation {
        Annotation {
            file: "dummy.rs".to_string(),
            line: 1,
            end_line: 1,
            start_column: 1,
            end_column: 1,
            message: "This is a dummy annotation".to_string(),
            annotation_type,
        }
    }
}
