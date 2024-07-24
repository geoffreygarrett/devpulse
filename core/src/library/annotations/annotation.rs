use std::fmt;

#[derive(Debug, Clone)]
pub enum AnnotationType {
    Notice,
    Warning,
    Error,
    Debug,
}

impl fmt::Display for AnnotationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let annotation_type = match self {
            AnnotationType::Notice => "notice",
            AnnotationType::Warning => "warning",
            AnnotationType::Error => "error",
            AnnotationType::Debug => "debug",
        };
        write!(f, "{}", annotation_type)
    }
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
