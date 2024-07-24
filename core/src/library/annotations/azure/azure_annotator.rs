/// AzureAnnotator provides methods to format annotations into Azure DevOps logging commands.
/// For more details on Azure DevOps logging commands, refer to:
/// https://learn.microsoft.com/en-gb/azure/devops/pipelines/scripts/logging-commands?view=azure-devops&tabs=bash#task-commands
use super::super::prelude::*;

pub struct AzureAnnotator;

impl Annotator for AzureAnnotator {
    /// Formats an annotation into a string suitable for Azure DevOps logging commands.
    ///
    /// The logging commands are based on the type of the annotation:
    /// - `Debug`: Logs a debug message using `##[debug]`.
    /// - `Notice`: Logs a notice message using `##[command]##[notice]`.
    /// - `Warning` and `Error`: Logs an issue using `##vso[task.logissue]`.
    ///
    /// # Parameters
    /// - `annotation`: The annotation to be formatted.
    ///
    /// # Returns
    /// A string formatted for Azure DevOps logging.
    fn get_annotation_string(&self, annotation: &Annotation) -> String {
        let location_str = annotation.format();
        match annotation.level {
            AnnotationLevel::Debug => format!(
                "##[debug]{} : debug : {}",
                location_str, annotation.message
            ),
            AnnotationLevel::Notice => format!(
                "##[command]##[notice]{} : {}",
                location_str, annotation.message
            ),
            _ => {
                let issue_command = match (annotation.line, annotation.end_line, annotation.col, annotation.end_col) {
                    (None, None, None, None) => format!(
                        "##vso[task.logissue type={};sourcepath={};]{}",
                        annotation.level,
                        annotation.file,
                        annotation.message
                    ),
                    (Some(line), Some(end_line), None, None) => format!(
                        "##vso[task.logissue type={};sourcepath={};linenumber={};linenumberEnd={};]{}",
                        annotation.level,
                        annotation.file,
                        line,
                        end_line,
                        annotation.message
                    ),
                    (Some(line), None, Some(col), Some(end_col)) => format!(
                        "##vso[task.logissue type={};sourcepath={};linenumber={};columnnumber={};endcolumnnumber={};]{}",
                        annotation.level,
                        annotation.file,
                        line,
                        col,
                        end_col,
                        annotation.message
                    ),
                    _ => format!(
                        "##vso[task.logissue type={};sourcepath={};linenumber={};columnnumber={};endcolumnnumber={};]{}",
                        annotation.level,
                        annotation.file,
                        annotation.line.unwrap_or_default(),
                        annotation.col.unwrap_or_default(),
                        annotation.end_col.unwrap_or_default(),
                        annotation.message
                    ),
                };

                issue_command
            },
        }
    }
}
