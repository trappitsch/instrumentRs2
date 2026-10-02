//! Parsing the commands and validate them for correctness.
//!
//! The bracket parser also allows returning the necessary parts to then create parsers for
//! delimited command strings and character length command strings required for creating the parsers
//! that decipher instrument answers in instrumentrs.

use thiserror::Error;

pub mod placeholder;
pub mod string_command_parser;

/// Parsing errors.
#[derive(Debug, Error)]
pub enum CommandParserError {
    /// For a placeholder that needs a delimited parser, a delimiter must be present between placeholders.
    #[error("not all placeholders are separated by delimiters")]
    DelimiterNotFound,
    /// One or more of the placeholders are invalid.
    #[error("one or more invalid placeholders were found")]
    InvalidPlaceholder,
    /// Mixed placeholders were found, which is invalid.
    #[error("a mix of positional and non positional placeholders is not allowed")]
    MixedPlaceholder,
    /// No placeholder was found.
    #[error("command contains no placeholder")]
    NoPlaceholder,
    /// Positional arguments of placeholder are not unique.
    #[error("positional arguments are not unique")]
    PositionNotUnique,
}

/// How do we separate between commands? This is generic between u8 and String parsers.
#[derive(Debug, Clone, PartialEq)]
pub enum CommandSeparation {
    /// A delimiter is used to separate between different types.
    ///
    /// Requires a DelimiterParser to parse the commands from the instrument.
    Delimiter,
    /// Character length is used to separate between different types.
    ///
    /// Requires a CharLengthParser to parse the commands from the instrument.
    CharLength,
}
