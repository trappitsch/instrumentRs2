//! This module parses command strings.
//!
//! It is public as the functions are needed for the derive macro. However, it is excluded from the
//! documentation and the module and function names begin with `__`. We thus consider it NOT part of
//! the public API.
//!
//! An instrument returns for example the following:
//! - "CHANNEL A1"
//!
//! If the command formatting string is "CHANNEL {}", then "A1" would be the part we want to part out.
//!
//! There are 4 types of placeholders that we will allow in command:
//! - `{}`: Empty placeholders, parameters after parsing should be returned in order.
//! - `{P}`: A positional placeholder: parameters after parsing should be returned in the order,
//!   here `P` must be parsable as a `usize`.
//! - `{X..=Y}`: A placeholder that the range of charactes to read. Can be exclusive upper bound
//!   (X..Y) or inclusive (X..=Y). X and Y must be parsable as numbers.
//! - `{X..=Y:P}` Placeholder with range of characters to read and a positional argument P.
//!
//! Note: A command string MUST only use one type of placeholder; mix and match is not allowed.

pub mod char_length_parser;
pub mod string_delimited_parser;
