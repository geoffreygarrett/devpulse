use color_eyre::Help;
use tracing::{error, Level, span};
use tracing_error::SpanTrace;

use models::*;

pub mod models;
pub mod okta_fga;

pub fn deserialize_yaml(
    yaml: &str,
) -> std::result::Result<Configuration, color_eyre::Report> {
    let span = span!(Level::INFO, "deserialize_yaml");
    let _enter = span.enter();

    serde_yaml::from_str(yaml).map_err(|e| {
        let span_trace = SpanTrace::capture();

        let error_line = e.location().map(|l| l.line()).unwrap_or(0);
        let error_column = e.location().map(|l| l.column()).unwrap_or(0);

        let error_message = format!(
            "Error: {}\nAt line: {}, column: {}\n{}",
            e,
            error_line,
            error_column,
            display_error_lines(yaml, error_line, error_column)
        );

        error!(message = %error_message, "Deserialization error");
        color_eyre::eyre::eyre!(error_message)
            .with_section(move || span_trace)
            .suggestion("Please check the YAML syntax and tags.")
    })
}

fn display_error_lines(yaml: &str, error_line: usize, error_column: usize) -> String {
    let lines: Vec<&str> = yaml.lines().collect();
    let start_line = if error_line > 3 { error_line - 3 } else { 0 };
    let end_line = (error_line + 3).min(lines.len());

    let mut output = String::new();

    for (i, line) in lines[start_line..end_line].iter().enumerate() {
        let line_number = start_line + i + 1;
        if line_number == error_line {
            output += &format!(
                "{:>4} | {}\n      {}\n",
                line_number,
                line,
                " ".repeat(error_column) + "^"
            );
        } else {
            output += &format!("{:>4} | {}\n", line_number, line);
        }
    }

    output
}
