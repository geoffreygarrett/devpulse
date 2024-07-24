use std::env;

use anyhow::Result;

use crate::annotation::Annotation;
use crate::annotator::Annotator;
use crate::azure::azure_annotator::AzureAnnotator;
use crate::bitbucket::bitbucket_annotator::BitbucketAnnotator;
use crate::github::github_annotator::GitHubAnnotator;
use crate::gitlab::gitlab_annotator::GitLabAnnotator;

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
        let annotator: Box<dyn Annotator> = Self::create_annotator(platform)?;

        Ok(Self { annotator })
    }

    fn create_annotator(platform: Platform) -> Result<Box<dyn Annotator>> {
        let annotator: Box<dyn Annotator> = match platform {
            Platform::GitHub => Box::new(GitHubAnnotator),
            Platform::Azure => Box::new(AzureAnnotator),
            Platform::GitLab => Box::new(GitLabAnnotator),
            Platform::Bitbucket => Box::new(BitbucketAnnotator),
        };

        Ok(annotator)
    }

    pub fn issue_annotations(&self, annotations: Vec<Annotation>) -> Result<()> {
        self.annotator.issue_annotations(annotations)
    }

    pub fn issue_annotation(&self, annotation: Annotation) -> Result<()> {
        self.annotator.issue_annotation(annotation)
    }

    pub fn get_annotation_string(&self, annotation: &Annotation) -> String {
        self.annotator.get_annotation_string(annotation)
    }

    pub fn get_annotations_strings(&self, annotations: &Vec<Annotation>) -> Vec<String> {
        self.annotator.get_annotations_strings(annotations)
    }
}
