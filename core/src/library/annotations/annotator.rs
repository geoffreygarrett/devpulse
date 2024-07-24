use anyhow::Result;

use crate::annotations::annotation::Annotation;

pub trait Annotator {
    fn get_annotation_string(&self, annotation: &Annotation) -> String;

    fn issue_annotation(&self, annotation: Annotation) -> Result<()> {
        println!("{}", self.get_annotation_string(&annotation));
        Ok(())
    }

    fn issue_annotations(&self, annotations: Vec<Annotation>) -> Result<()> {
        for annotation in annotations {
            self.issue_annotation(annotation)?;
        }
        Ok(())
    }

    fn get_annotations_strings(&self, annotations: &Vec<Annotation>) -> Vec<String> {
        annotations
            .iter()
            .map(|a| self.get_annotation_string(a))
            .collect()
    }
}
