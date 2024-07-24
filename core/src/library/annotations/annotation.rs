#[derive(Debug, Clone)]
pub enum AnnotationType {
    Notice,
    Warning,
    Error,
    Debug,
}

#[derive(Debug, Clone)]
pub struct Annotation {
    pub file: String,
    pub line: usize,
    pub end_line: usize,
    pub start_column: usize,
    pub end_column: usize,
    pub message: String,
    pub annotation_type: AnnotationType,
}
