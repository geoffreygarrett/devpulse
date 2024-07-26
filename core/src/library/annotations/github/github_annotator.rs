use super::super::prelude::*;

/// GitHubAnnotator provides methods to format annotations into GitHub Actions logging commands.
pub struct GitHubAnnotator;

impl GitHubAnnotator {
    pub fn new() -> Self {
        GitHubAnnotator {}
    }

    /// Constructs a location string based on the available fields in the annotation.
    pub(crate) fn construct_location_string(annotation: &Annotation) -> String {
        match (annotation.line, annotation.end_line, annotation.col, annotation.end_col) {
            (Some(start_line), Some(end_line), Some(start_col), Some(end_col)) => format!(
                "file={},line={},endLine={},col={},endColumn={}",
                annotation.file, start_line, end_line, start_col, end_col
            ),
            (Some(start_line), Some(end_line), None, None) => {
                format!("file={},line={},endLine={}", annotation.file, start_line, end_line)
            }
            (Some(line), None, Some(start_col), Some(end_col)) => {
                format!(
                    "file={},line={},col={},endColumn={}",
                    annotation.file, line, start_col, end_col
                )
            }
            (Some(line), None, None, None) => format!("file={},line={}", annotation.file, line),
            _ => "".to_string(),
        }
    }
}

impl Annotator for GitHubAnnotator {
    /// Formats an annotation into a string suitable for GitHub Actions annotations.
    ///
    /// # Parameters
    /// - `annotation`: The annotation to be formatted.
    ///
    /// # Returns
    /// A string formatted for GitHub Actions annotations.
    fn get_annotation_string(&self, annotation: &Annotation) -> String {
        let location_str = GitHubAnnotator::construct_location_string(annotation);
        format!("::{} {}::{}", annotation.level.to_string(), location_str, annotation.message)
    }
}
