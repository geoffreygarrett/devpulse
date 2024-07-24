use super::super::prelude::*;

pub struct GitLabAnnotator;

impl Annotator for GitLabAnnotator {
    fn get_annotation_string(&self, annotation: &Annotation) -> String {
        format!(
            "{} in file: {} on line: {}, column: {}. {}",
            annotation.annotation_type,
            annotation.file,
            annotation.line,
            annotation.start_column,
            annotation.message
        )
    }
}
