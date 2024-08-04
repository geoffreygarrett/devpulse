use std::backtrace::Backtrace;

use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum ProtobufError {
    #[snafu(display("Missing field: {field}"))]
    MissingField { field: String, backtrace: Backtrace },
    #[snafu(display("Conversion error: {reason}"))]
    ConversionError {
        reason: String,
        backtrace: Backtrace,
    },
    #[snafu(display("Unknown error"))]
    Unknown { backtrace: Backtrace },
}

#[derive(Debug)]
pub enum ParseError {
    Pest(pest::error::Error<crate::models::acl::parser::Rule>),
    MissingPermission,
    MissingUser,
    MissingField(&'static str),
}
