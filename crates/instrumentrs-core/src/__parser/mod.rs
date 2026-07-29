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
//! - `{:B}`: A placeholder that defines that this placeholder always contains `B` characters.
//!   Here `B` must be parsable as a number.
//! - `{P:B}` A positional, character length defining placeholder.
//!
//! Note: A command string MUST only use one type of placeholder; mix and match is not allowed.

pub mod char_length_parser;
pub mod delimited_parser;
