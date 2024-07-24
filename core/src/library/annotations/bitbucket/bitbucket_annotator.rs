use super::super::prelude::*;


pub struct BitbucketAnnotator;

impl Annotator for BitbucketAnnotator {
    fn get_annotation_string(&self, annotation: &Annotation) -> String {
        format!(
            "Annotation: {}: {} in file: {} on line: {}",
            annotation.annotation_type, annotation.message, annotation.file, annotation.line,
        )
    }
}
