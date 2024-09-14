use std::fmt::{self, Display};

use color_eyre::owo_colors::OwoColorize;
use pest::Span;
use serde::{de, ser};

use crate::parser::fga::Rule;

/// Alias for a `Result` with error type `Error`
pub type Result<T> = std::result::Result<T, Error>;

/// One-based line and column at which the error was detected.
#[derive(Clone, Debug, PartialEq)]
pub struct Location {
    /// The one-based line number of the error.
    pub line: usize,
    /// The one-based column number of the error.
    pub column: usize,
}

impl From<&Span<'_>> for Location {
    fn from(s: &Span<'_>) -> Self {
        let (line, column) = s.start_pos().line_col();
        Self { line, column }
    }
}

/// A custom error type for parsing.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    /// Error message with optional location.
    Message {
        /// The error message.
        msg: String,
        /// The location of the error, if applicable.
        location: Option<Location>,
    },
}

impl From<pest::error::Error<Rule>> for Error {
    fn from(err: pest::error::Error<Rule>) -> Self {
        let (line, column) = match err.line_col {
            pest::error::LineColLocation::Pos((l, c)) => (l, c),
            pest::error::LineColLocation::Span((l, c), (_, _)) => (l, c),
        };
        Error::Message {
            msg: err.to_string(),
            location: Some(Location { line, column }),
        }
    }
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message {
            msg: msg.to_string(),
            location: None,
        }
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message {
            msg: msg.to_string(),
            location: None,
        }
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Message { ref msg, ref location } => {
                match location {
                    Some(loc) => write!(formatter, "{} at line {}, column {}", msg.red(), loc.line, loc.column),
                    None => write!(formatter, "{}", msg.red()),
                }
            }
        }
    }
}

impl std::error::Error for Error {}

/// Adds location information from `span`, if `res` is an error.
pub fn set_location<T>(res: &mut Result<T>, span: &Span<'_>) {
    if let Err(ref mut e) = res {
        let Error::Message { location, .. } = e;
        if location.is_none() {
            let (line, column) = span.start_pos().line_col();
            *location = Some(Location { line, column });
        }
    }
}

/// Helper function to display error context lines.
pub fn display_error_lines(content: &str, error_line: usize, error_column: usize) -> String {
    let lines: Vec<&str> = content.lines().collect();
    let start_line = if error_line > 3 { error_line - 3 } else { 0 };
    let end_line = (error_line + 3).min(lines.len());

    let mut output = String::new();

    for (i, line) in lines[start_line..end_line].iter().enumerate() {
        let line_number = start_line + i + 1;
        if line_number == error_line {
            output += &format!(
                "{} | {}\n      {}\n",
                format!("{:>4}", line_number).red(),
                line,
                format!("{}^", " ".repeat(error_column)).red()
            );
        } else {
            output += &format!("{:>4} | {}\n", line_number, line);
        }
    }

    output
}

