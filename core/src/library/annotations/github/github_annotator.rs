use super::super::prelude::*;

pub struct GitHubAnnotator;

impl Annotator for GitHubAnnotator {
    fn get_annotation_string(&self, annotation: &Annotation) -> String {
        format!(
            "::{} file={},line={},endLine={},col={},endColumn={}::{}",
            annotation.annotation_type,
            annotation.file,
            annotation.line,
            annotation.end_line,
            annotation.start_column,
            annotation.end_column,
            annotation.message
        )
    }
}
