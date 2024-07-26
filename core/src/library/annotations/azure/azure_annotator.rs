use super::super::prelude::*;

/// Formats annotations for Azure DevOps logging commands.
///
/// AzureAnnotator is responsible for converting annotation data into a string format
/// recognized by Azure DevOps, enabling it to display annotations such as warnings, errors,
/// and debug messages directly in the build logs. This helps in diagnostics and understanding
/// the flow of execution or any issues that arise during the build process.
///
/// # References
/// - [Azure DevOps Logging Commands](https://learn.microsoft.com/en-us/azure/devops/pipelines/scripts/logging-commands)
pub struct AzureAnnotator;

impl AzureAnnotator {
    pub fn new() -> Self {
        AzureAnnotator {}
    }

    /// Constructs a location string based on the available fields in the annotation.
    pub(crate) fn construct_location_string(annotation: &Annotation) -> String {
        match (annotation.line, annotation.end_line, annotation.col, annotation.end_col) {
            (Some(line), None, None, None) => {
                format!("sourcepath={};linenumber={}", annotation.file, line)
            }
            (Some(line), Some(end_line), None, None) => format!(
                "sourcepath={};linenumber={};linenumberEnd={}",
                annotation.file, line, end_line
            ),
            (Some(line), None, Some(col), None) => {
                format!("sourcepath={};linenumber={};columnnumber={}", annotation.file, line, col)
            }
            (Some(line), None, None, Some(end_col)) => format!(
                "sourcepath={};linenumber={};endcolumnnumber={}",
                annotation.file, line, end_col
            ),
            (Some(line), Some(end_line), Some(col), None) => format!(
                "sourcepath={};linenumber={};linenumberEnd={};columnnumber={}",
                annotation.file, line, end_line, col
            ),
            (Some(line), Some(end_line), None, Some(end_col)) => format!(
                "sourcepath={};linenumber={};linenumberEnd={};endcolumnnumber={}",
                annotation.file, line, end_line, end_col
            ),
            (Some(line), None, Some(col), Some(end_col)) => format!(
                "sourcepath={};linenumber={};columnnumber={};endcolumnnumber={}",
                annotation.file, line, col, end_col
            ),
            (Some(line), Some(end_line), Some(col), Some(end_col)) => format!(
                "sourcepath={};linenumber={};linenumberEnd={};columnnumber={};endcolumnnumber={}",
                annotation.file, line, end_line, col, end_col
            ),
            (None, None, None, None) => String::new(),
            _ => format!("sourcepath={}", annotation.file), // Default case to handle unexpected combinations
        }
    }
}

impl Annotator for AzureAnnotator {
    /// Converts an annotation into a formatted string that conforms to Azure DevOps logging commands.
    ///
    /// Depending on the annotation level (Debug, Notice, Warning, Error), this method formats
    /// the output differently to utilize Azure DevOps's capability to highlight different types
    /// of logs.
    ///
    /// # Parameters
    /// - `annotation`: A reference to the annotation instance containing all relevant data.
    ///
    /// # Returns
    /// - A string formatted according to Azure DevOps logging command specifications.
    fn get_annotation_string(&self, annotation: &Annotation) -> String {
        match annotation.level {
            AnnotationLevel::Debug => {
                format!("##[debug]{} : debug : {}", annotation.file, annotation.message)
            }
            AnnotationLevel::Notice => {
                format!("##[command]##[notice]{} : {}", annotation.file, annotation.message)
            }
            AnnotationLevel::Warning | AnnotationLevel::Error => {
                let type_str = match annotation.level {
                    AnnotationLevel::Warning => "warning",
                    AnnotationLevel::Error => "error",
                    _ => unreachable!(),
                };

                let location_str = Self::construct_location_string(annotation);

                format!(
                    "##vso[task.logissue type={};{}]{}",
                    type_str, location_str, annotation.message
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debug_annotation() {
        let annotator = AzureAnnotator;
        let annotation = Annotation {
            file: "tests/src/main.rs".to_string(),
            line: Some(5),
            end_line: None,
            col: None,
            end_col: None,
            message: "Debug message".to_string(),
            level: AnnotationLevel::Debug,
        };

        let result = annotator.get_annotation_string(&annotation);
        assert_eq!(result, "##[debug]tests/src/main.rs : debug : Debug message");
    }

    #[test]
    fn test_notice_annotation() {
        let annotator = AzureAnnotator;
        let annotation = Annotation {
            file: "tests/src/main.rs".to_string(),
            line: Some(15),
            end_line: Some(20),
            col: None,
            end_col: None,
            message: "Consider refactoring this function".to_string(),
            level: AnnotationLevel::Notice,
        };

        let result = annotator.get_annotation_string(&annotation);
        assert_eq!(
            result,
            "##[command]##[notice]tests/src/main.rs : Consider refactoring this function"
        );
    }

    #[test]
    fn test_warning_annotation() {
        let annotator = AzureAnnotator;
        let annotation = Annotation {
            file: "tests/src/main.rs".to_string(),
            line: Some(25),
            end_line: None,
            col: Some(10),
            end_col: Some(15),
            message: "Ensure the command format is correct.".to_string(),
            level: AnnotationLevel::Warning,
        };

        let result = annotator.get_annotation_string(&annotation);
        assert_eq!(
            result,
            "##vso[task.logissue type=warning;sourcepath=tests/src/main.rs;linenumber=25;columnnumber=10;endcolumnnumber=15]Ensure the command format is correct."
        );
    }

    #[test]
    fn test_error_annotation() {
        let annotator = AzureAnnotator;
        let annotation = Annotation {
            file: "tests/src/main.rs".to_string(),
            line: Some(30),
            end_line: None,
            col: Some(20),
            end_col: None,
            message: "This function implementation is incorrect.".to_string(),
            level: AnnotationLevel::Error,
        };

        let result = annotator.get_annotation_string(&annotation);
        assert_eq!(
            result,
            "##vso[task.logissue type=error;sourcepath=tests/src/main.rs;linenumber=30;columnnumber=20]This function implementation is incorrect."
        );
    }
}
