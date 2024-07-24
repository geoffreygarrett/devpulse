use snafu::Snafu;

use super::annotation::Annotation;

#[derive(Debug, Snafu)]
pub enum AnnotatorError {
    #[snafu(display("Failed to issue annotation: {}", source))]
    IssueAnnotation { source: Box<dyn std::error::Error> },
}

pub type Result<T, E = AnnotatorError> = std::result::Result<T, E>;

pub trait Annotator {
    fn get_annotation_string(&self, annotation: &Annotation) -> String;

    fn issue_annotation(&self, annotation: Annotation) -> Result<()> {
        let annotation_str = self.get_annotation_string(&annotation);
        println!("{}", annotation_str);
        Ok(())
    }

    fn issue_annotations(&self, annotations: Vec<Annotation>) -> Result<()> {
        for annotation in annotations {
            self.issue_annotation(annotation)?;
        }
        Ok(())
    }

    fn get_annotations_strings(&self, annotations: &[Annotation]) -> Vec<String> {
        annotations
            .iter()
            .map(|a| self.get_annotation_string(a))
            .collect()
    }
}
