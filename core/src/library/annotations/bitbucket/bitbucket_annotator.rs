/// BitbucketAnnotator formats annotations into strings suitable for Bitbucket logging commands.
///
/// # Details
/// - Formats a simple log message for Bitbucket, including annotation level, message, file, and line number.
///
/// # Example
/// For an annotation with level `Notice`, message `"Example message"`, file `"example.rs"`, and line `42`, the output will be:
/// `"Annotation: notice: Example message in file: example.rs on line: 42"`
use super::super::prelude::*;

pub struct BitbucketAnnotator;

impl Annotator for BitbucketAnnotator {
    /// Formats an annotation into a string suitable for Bitbucket logging commands.
    ///
    /// # Parameters
    /// - `annotation`: The annotation to be formatted.
    ///
    /// # Returns
    /// A string formatted for Bitbucket logging.
    fn get_annotation_string(&self, annotation: &Annotation) -> String {
        // Construct line information based on available fields
        let line_info = match annotation.line {
            Some(line) => format!("on line: {}", line),
            None => "".to_string(),
        };

        format!(
            "Annotation: {}: {} in file: {} {}",
            annotation.level, annotation.message, annotation.file, line_info
        )
    }
}
