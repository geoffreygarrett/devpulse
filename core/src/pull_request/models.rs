/// Represents the severity level of an annotation.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

/// Represents a range within a file, including optional column start and end for precise positioning.
#[derive(Debug, Clone)]
pub struct CodeRange {
    file_path: String,
    start_line: usize,
    start_column: Option<usize>,
    end_line: usize,
    end_column: Option<usize>,
}

/// Represents an annotated message linked to a specific range in code.
#[derive(Debug, Clone)]
pub struct Annotation {
    range: CodeRange,
    severity: Severity,
    message: String,
}
