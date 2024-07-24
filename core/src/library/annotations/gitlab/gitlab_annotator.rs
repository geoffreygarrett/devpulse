/// GitLabAnnotator formats annotations into strings suitable for GitLab CI/CD annotations.
///
/// # Details
/// - Formats a log message for GitLab CI/CD with appropriate fields based on the annotation type.
/// - Constructs the message based on available fields in the `Annotation`.
///
/// # Example
/// For an annotation with level `Warning`, file `"example.rs"`, start line `42`, start column `5`, and message `"Example message"`, the output will be:
/// `"warning in file: example.rs on line: 42, column: 5. Example message"`
use super::super::prelude::*;

pub struct GitLabAnnotator;

impl Annotator for GitLabAnnotator {
    /// Formats an annotation into a string suitable for GitLab CI/CD annotations.
    ///
    /// # Parameters
    /// - `annotation`: The annotation to be formatted.
    ///
    /// # Returns
    /// A string formatted for GitLab CI/CD annotations.
    fn get_annotation_string(&self, annotation: &Annotation) -> String {
        // Construct line and column information based on available fields
        let line_info = match (annotation.line, annotation.col) {
            (Some(line), Some(column)) => format!("on line: {}, column: {}", line, column),
            (Some(line), None) => format!("on line: {}", line),
            (None, Some(column)) => format!("at column: {}", column),
            _ => "".to_string(), // handle unexpected combinations
        };

        format!(
            "{} in file: {} {}. {}",
            annotation.level, annotation.file, line_info, annotation.message
        )
    }
}
