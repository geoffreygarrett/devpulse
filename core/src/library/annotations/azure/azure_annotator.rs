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
        match annotation.annotation_type {
            AnnotationType::Debug => format!(
                "##[debug]{}{}({},{})",
                annotation.message, annotation.file, annotation.line, annotation.start_column
            ),
            AnnotationType::Notice => format!(
                "##[command]##[notice]{}{}({},{})",
                annotation.message, annotation.file, annotation.line, annotation.start_column
            ),
            _ => format!(
                "##vso[task.logissue type={};sourcepath={};linenumber={};columnnumber={};endcolumnnumber={};]{}",
                annotation.annotation_type,
                annotation.file,
                annotation.line,
                annotation.start_column,
                annotation.end_column,
                annotation.message
            ),
        }
    }
}
