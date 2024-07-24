/// GitHubAnnotator formats annotations into strings suitable for GitHub Actions annotations.
///
/// # Details
/// - Formats a log message for GitHub Actions with appropriate fields based on the type of annotation.
/// - Only includes the fields that are relevant for the annotation type.
///
/// # Example
/// For an annotation with level `Warning`, file `"example.rs"`, start line `42`, end line `50`, start column `5`, end column `10`, and message `"Example message"`, the output will be:
/// `"::warning file=example.rs,line=42,endLine=50,col=5,endColumn=10::Example message"`
use super::super::prelude::*;

pub struct GitHubAnnotator;

impl Annotator for GitHubAnnotator {
    /// Formats an annotation into a string suitable for GitHub Actions annotations.
    ///
    /// # Parameters
    /// - `annotation`: The annotation to be formatted.
    ///
    /// # Returns
    /// A string formatted for GitHub Actions annotations.
    fn get_annotation_string(&self, annotation: &Annotation) -> String {
        // Construct location string based on available fields
        let location_str = match (
            annotation.line,
            annotation.end_line,
            annotation.col,
            annotation.end_col,
        ) {
            (None, None, None, None) => "".to_string(),
            (Some(start_line), Some(end_line), None, None) => {
                format!("file={},line={},endLine={}", annotation.file, start_line, end_line)
            }
            (Some(line), None, Some(start_column), Some(end_column)) => {
                format!(
                    "file={},line={},col={},endColumn={}",
                    annotation.file, line, start_column, end_column
                )
            }
            (Some(line), None, None, None) => {
                format!("file={},line={}", annotation.file, line)
            }
            _ => "".to_string(), // handle unexpected combinations
        };

        format!("::{} {}::{}", annotation.level, location_str, annotation.message)
    }
}
