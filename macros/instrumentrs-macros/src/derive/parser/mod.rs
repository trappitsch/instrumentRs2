//! Parsing the commands and validate them for correctness.
//!
//! The bracket parser also allows returning the necessary parts to then create parsers for
//! delimited command strings and character length command strings required for creating the parsers
//! that decipher instrument answers in instrumentrs.

use thiserror::Error;

mod command_parser;
mod placeholder;

/// Parsing errors.
#[derive(Debug, Error)]
pub enum CommandParserError {
    /// For a placeholder that needs a delimited parser, a delimiter must be present between placeholders.
    #[error("DelimiterNotFound")]
    DelimiterNotFound,
    /// One or more of the placeholders are invalid.
    #[error("InvalidPlaceholder")]
    InvalidPlaceholder,
    /// Mixed placeholders were found, which is invalid.
    #[error("MixedPlaceholder")]
    MixedPlaceholder,
    /// No placeholder was found.
    #[error("NoPlaceholder")]
    NoPlaceholder,
    /// Positional arguments of placeholder are not unique.
    #[error("PositionNotUnique")]
    PositionNotUnique,
}
