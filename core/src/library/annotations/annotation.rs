use std::fmt;

use derive_builder::Builder;

/// AnnotationLevel represents different levels of annotations.
#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationLevel {
    Notice,
    Warning,
    Error,
    Debug,
}

impl fmt::Display for AnnotationLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let annotation_level = match self {
            AnnotationLevel::Notice => "notice",
            AnnotationLevel::Warning => "warning",
            AnnotationLevel::Error => "error",
            AnnotationLevel::Debug => "debug",
        };
        write!(f, "{}", annotation_level)
    }
}

/// Annotation represents an annotation with a file, target range, message, and level.
#[derive(Debug, Clone, PartialEq, Builder)]
#[builder(setter(into))]
pub struct Annotation {
    pub file: String,
    pub line: Option<usize>,
    pub end_line: Option<usize>,
    pub col: Option<usize>,
    pub end_col: Option<usize>,
    pub message: String,
    pub level: AnnotationLevel,
}

impl Annotation {
    /// Create a new Annotation using the builder pattern.
    pub fn builder() -> AnnotationBuilder {
        AnnotationBuilder::default()
    }

    /// Create a new annotation for the whole file.
    pub fn new_whole_file(file: String, message: String, level: AnnotationLevel) -> Self {
        Annotation {
            file,
            line: None,
            end_line: None,
            col: None,
            end_col: None,
            message,
            level,
        }
    }

    /// Create a new annotation for a line range.
    pub fn new_line_range(
        file: String, line: usize, end_line: usize, message: String, level: AnnotationLevel,
    ) -> Self {
        Annotation {
            file,
            line: Some(line),
            end_line: Some(end_line),
            col: None,
            end_col: None,
            message,
            level,
        }
    }

    /// Create a new annotation for a column range.
    pub fn new_column_range(
        file: String, line: usize, col: usize, end_col: usize, message: String,
        level: AnnotationLevel,
    ) -> Self {
        Annotation {
            file,
            line: Some(line),
            end_line: None,
            col: Some(col),
            end_col: Some(end_col),
            message,
            level,
        }
    }

    /// Formats the annotation for output.
    pub fn format(&self) -> String {
        let range = match (self.line, self.end_line, self.col, self.end_col) {
            (None, None, None, None) => "".to_string(),
            (Some(line), Some(end_line), None, None) => {
                format!("({},{})", line, end_line)
            }
            (Some(line), None, Some(col), Some(end_col)) => {
                format!("({},{}-{})", line, col, end_col)
            }
            (Some(line), None, None, None) => {
                format!("({})", line)
            }
            _ => "".to_string(), // handle unexpected combinations
        };
        format!("{}{} : {} : {}", self.file, range, self.level, self.message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_whole_file() {
        let annotation = Annotation::builder()
            .file("main.rs".to_string())
            .line(None)
            .end_line(None)
            .col(None)
            .end_col(None)
            .message("Whole file annotation".to_string())
            .level(AnnotationLevel::Notice)
            .build()
            .unwrap();
        assert_eq!(annotation.file, "main.rs");
        assert_eq!(annotation.line, None);
        assert_eq!(annotation.end_line, None);
        assert_eq!(annotation.col, None);
        assert_eq!(annotation.end_col, None);
        assert_eq!(annotation.message, "Whole file annotation");
        assert_eq!(annotation.level, AnnotationLevel::Notice);
    }

    #[test]
    fn test_builder_line_range() {
        let annotation = Annotation::builder()
            .file("main.rs".to_string())
            .line(Some(10))
            .end_line(Some(20))
            .col(None)
            .end_col(None)
            .message("Line range annotation".to_string())
            .level(AnnotationLevel::Warning)
            .build()
            .unwrap();
        assert_eq!(annotation.file, "main.rs");
        assert_eq!(annotation.line, Some(10));
        assert_eq!(annotation.end_line, Some(20));
        assert_eq!(annotation.col, None);
        assert_eq!(annotation.end_col, None);
        assert_eq!(annotation.message, "Line range annotation");
        assert_eq!(annotation.level, AnnotationLevel::Warning);
    }

    #[test]
    fn test_builder_column_range() {
        let annotation = Annotation::builder()
            .file("main.rs".to_string())
            .line(Some(15))
            .end_line(None)
            .col(Some(5))
            .end_col(Some(10))
            .message("Column range annotation".to_string())
            .level(AnnotationLevel::Error)
            .build()
            .unwrap();
        assert_eq!(annotation.file, "main.rs");
        assert_eq!(annotation.line, Some(15));
        assert_eq!(annotation.end_line, None);
        assert_eq!(annotation.col, Some(5));
        assert_eq!(annotation.end_col, Some(10));
        assert_eq!(annotation.message, "Column range annotation");
        assert_eq!(annotation.level, AnnotationLevel::Error);
    }

    #[test]
    fn test_annotation_level_display() {
        assert_eq!(format!("{}", AnnotationLevel::Notice), "notice");
        assert_eq!(format!("{}", AnnotationLevel::Warning), "warning");
        assert_eq!(format!("{}", AnnotationLevel::Error), "error");
        assert_eq!(format!("{}", AnnotationLevel::Debug), "debug");
    }

    #[test]
    fn test_format_whole_file() {
        let annotation = Annotation::new_whole_file(
            "main.rs".to_string(),
            "Whole file annotation".to_string(),
            AnnotationLevel::Notice,
        );
        assert_eq!(annotation.format(), "main.rs : notice : Whole file annotation");
    }

    #[test]
    fn test_format_line_range() {
        let annotation = Annotation::new_line_range(
            "main.rs".to_string(),
            10,
            20,
            "Line range annotation".to_string(),
            AnnotationLevel::Warning,
        );
        assert_eq!(annotation.format(), "main.rs(10,20) : warning : Line range annotation");
    }

    #[test]
    fn test_format_column_range() {
        let annotation = Annotation::new_column_range(
            "main.rs".to_string(),
            15,
            5,
            10,
            "Column range annotation".to_string(),
            AnnotationLevel::Error,
        );
        assert_eq!(annotation.format(), "main.rs(15,5-10) : error : Column range annotation");
    }
}
