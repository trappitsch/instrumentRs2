//! This module takes care of formatting the command string with its arguments.
//! It also holds the functions to parse the command string.

use syn::{Attribute, spanned::Spanned};

use crate::derive::utils;

#[derive(Debug, Default)]
pub struct CommandString {
    cmd: String,
}

impl CommandString {
    /// Create a new `FormatCommand` for evaluating an enum.
    ///
    /// Error:
    /// - 'cmd' attribute not found.
    /// - Command does not contain exactly one {}.
    pub fn try_new_enum<S: Spanned>(attrs: &[Attribute], site: &S) -> syn::Result<Self> {
        let sa = utils::get_named_attribute_content_string(attrs, "cmd", site)?;

        // we expect exactly one empty '{}'
        if sa.value.matches("{}").count() != 1 {
            return Err(syn::Error::new(
                sa.span,
                "command string must have exactly one empty {}",
            ));
        }

        Ok(Self { cmd: sa.value })
    }
    pub fn format_with(&self, value: &str) -> String {
        self.cmd.replace("{}", value).to_string()
    }
}
