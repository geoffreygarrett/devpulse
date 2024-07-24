use super::super::prelude::*;

pub struct AzureAnnotator;

impl Annotator for AzureAnnotator {
    fn get_annotation_string(&self, annotation: &Annotation) -> String {
        format!(
            "##vso[task.logissue type={};sourcepath={};linenumber={};columnnumber={};endcolumnnumber={};]{}",
            annotation.annotation_type,
            annotation.file,
            annotation.line,
            annotation.start_column,
            annotation.end_column,
            annotation.message
        )
    }
}
