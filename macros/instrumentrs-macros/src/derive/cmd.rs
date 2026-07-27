//! This module takes care of formatting the command string with its arguments.
//! It also holds the functions to parse the command string.
//! TODO:

use syn::{Attribute, spanned::Spanned};

use crate::derive::utils;

/// This struct serves to store command string delimiters.
///
/// For a command: "XXX{},{}YY{}ZZZ"
/// this would result in "XXX" being stored in 'before', [",", "YY"] being stored in 'between', and
/// "ZZZ" being stored in after.
#[derive(Debug, Default, PartialEq)]
pub struct CmdDelimiters {
    pub before: String,
    pub between: Vec<String>,
    pub after: String,
}

/// This struct serves for the return when parsing the next item of the command.
#[derive(Debug)]
enum ParseReturn {
    /// So we found a placeholder...
    Placeholder(PrPlaceholder),
    /// After the last placeholder is parsed, this is returned with the rest (rightmost part).
    Last(String),
}

/// This struct holds the information that we need when `parse_next` found a placeholder.
#[derive(Debug, Default)]
struct PrPlaceholder {
    /// What came before this placeholder
    before: String,
    /// What is the order of this placeholder (if any)
    order: Option<usize>,
    /// What comes after this placeholder
    remainder: String,
}

/// The order of the placeholders, as a simple vec.
#[derive(Debug, Default)]
pub struct PlaceholderOrder {
    order: Vec<Option<usize>>,
}

impl PlaceholderOrder {
    // Check if all placeholders have order (all are Some).
    fn all_have_order(&self) -> bool {
        self.order.iter().all(|f| f.is_some())
    }

    // Check if all placeholders have no order (all are None).
    fn all_have_no_order(&self) -> bool {
        self.order.iter().all(|f| f.is_none())
    }

    // Check if the ordering is mixed (not allowed in `CommandParseFormat`)
    fn has_mixed_order(&self) -> bool {
        !self.all_have_order() && !self.all_have_no_order()
    }

    // Get a Vec<usize> that represents the sorting based on the order.
    //
    // Example:
    // self.order = Vec<None, Some(0), None, Some(1)>
    //
    // Then the order returned would be:
    //
    // Vec<2, 0, 3, 1>
    //
    // Panics:
    // - There are ordered and unordered placeholders. This should be unreachable since it is
    // checked on creation.
    fn get_sorting(&self) -> Vec<usize> {
        if self.all_have_no_order() {
            (0..self.len()).collect()
        } else if self.all_have_order() {
            self.order
                .iter()
                .map(|f| f.expect("all have order"))
                .collect()
        } else {
            unreachable!(
                "mixed order/no-order in placeholders should throw an error on creation - report as bug"
            )
        }
    }

    pub fn sort_slice<'a, T>(&self, sl: &'a [T]) -> Vec<&'a T> {
        let sorting_order = self.get_sorting();
        let mut pairs: Vec<_> = sl.iter().zip(sorting_order).collect();
        pairs.sort_by_key(|p| p.1);

        pairs.iter().map(|p| p.0).collect()
    }

    /// How many orders there are (equal to number placeholders).
    pub fn len(&self) -> usize {
        self.order.len()
    }

    // Push a new order.
    fn push(&mut self, val: Option<usize>) {
        self.order.push(val);
    }
}

/// A struct to parse a command, determine placeholder, and take over formatting.
#[derive(Debug, Default)]
pub struct CommandParseFormat {
    command: String,
    delim: CmdDelimiters,
    order: PlaceholderOrder,
}

impl TryFrom<&str> for CommandParseFormat {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let command = value.to_string();
        let mut delim = CmdDelimiters::default();
        let mut order = PlaceholderOrder::default();

        // parse first one
        let mut parsed = parse_next(value)?;
        let mut remainder = match parsed {
            ParseReturn::Placeholder(PrPlaceholder {
                before: bef,
                order: ord,
                remainder: rem,
            }) => {
                delim.before = bef;
                order.push(ord);
                rem
            }
            ParseReturn::Last(l) => return Err(format!("command {} contains no placeholder", l)),
        };

        loop {
            parsed = parse_next(&remainder)?;

            remainder = match parsed {
                ParseReturn::Placeholder(PrPlaceholder {
                    before: bef,
                    order: ord,
                    remainder: rem,
                }) => {
                    delim.between.push(bef);
                    order.push(ord);
                    rem
                }
                ParseReturn::Last(last) => {
                    delim.after = last;
                    break;
                }
            }
        }

        Ok(CommandParseFormat {
            command,
            delim,
            order,
        })
    }
}

impl CommandParseFormat {
    /// Create a new `FormatCommand` for evaluating an enum.
    ///
    /// If created in this way, formatted values made with `format_with_one` can safely be
    /// unwrapped as the same checks have been done at creation.
    ///
    /// Error:
    /// - Could not find the "cmd" attribute in the provided attrs.
    /// - An ordered placeholder was provided, which is invalid for enums.
    /// - Number of placeholders is not equal to 1.
    pub fn try_new_enum<S: Spanned>(attrs: &[Attribute], site: &S) -> syn::Result<Self> {
        let sa = utils::get_named_attribute_content_string(attrs, "cmd", site)?;

        let placeholder_error =
            syn::Error::new(sa.span, "command string must have exactly one empty {}");

        let cpf = Self::try_from(sa.value.as_str()).map_err(|_| placeholder_error.clone())?;

        if cpf.number_placeholders() != 1 || !cpf.order.all_have_no_order() {
            return Err(placeholder_error);
        }

        Ok(cpf)
    }

    /// Create a new `FromatCommand` for evaluating a named struct.
    ///
    /// Error:
    /// - Could not find the "cmd" attribute in teh provided attrs.
    /// - An ordered argument could not be parsed.
    /// - A mix of ordered and unordered arguments were provided.
    pub fn try_new_struct<S: Spanned>(
        attrs: &[Attribute],
        site: &S,
        numb_fields: usize,
    ) -> syn::Result<Self> {
        let sa = utils::get_named_attribute_content_string(attrs, "cmd", site)?;

        let cpf = Self::try_from(sa.value.as_str()).map_err(|e| syn::Error::new(sa.span, e))?;

        if cpf.order.len() != numb_fields {
            return Err(syn::Error::new(
                sa.span,
                format!(
                    "struct has {} field(s) but {} placeholders were provided",
                    numb_fields,
                    cpf.order.len()
                ),
            ));
        }

        if cpf.order.has_mixed_order() {
            return Err(syn::Error::new(
                sa.span,
                "a mix of positional {0} and non positional {} placeholders is not allowed",
            ));
        }

        Ok(cpf)
    }

    /// Get the command.
    pub fn get_command(&self) -> &str {
        self.command.as_str()
    }

    /// Get the CmdDelimiters.
    pub fn get_cmd_delimiters(&self) -> &CmdDelimiters {
        &self.delim
    }

    /// Get the sorting order for the placeholders.
    pub fn get_placeholder_order(&self) -> &PlaceholderOrder {
        &self.order
    }

    /// Get the number of placeholders (equal to the number of stored orders).
    pub fn number_placeholders(&self) -> usize {
        self.order.len()
    }

    /// This simply replaces the `{}` in the command string with the given string.
    ///
    /// This method is mainly for enums, where we have a well defined placeholder, i.e., exactly one
    /// '{}'.
    ///
    /// Error:
    /// - Number of placeholders is not == 1.
    /// - Placeholder has an order.
    pub fn format_with_one(&self, s: &str) -> Result<String, String> {
        if self.number_placeholders() != 1 {
            return Err(
                "number of {} placeholders in formatting string must be equal to one".to_string(),
            );
        }

        if !self.order.all_have_no_order() {
            return Err(
                "the command string has order and thus cannot be formatted in this way."
                    .to_string(),
            );
        }

        Ok(self.command.replace("{}", s))
    }
}

/// Parse the next placeholder from a given string slice.
fn parse_next(s: &str) -> Result<ParseReturn, String> {
    let mut before = String::new();
    let mut order_str = String::new();

    let mut start_found = false;
    let mut end_index = None;

    for (it, c) in s.chars().enumerate() {
        match start_found {
            false => {
                if c == '{' {
                    start_found = true;
                } else {
                    before.push(c);
                }
            }
            true => {
                if c == '}' {
                    end_index = Some(it);
                    break;
                } else {
                    order_str.push(c);
                }
            }
        }
    }

    if let Some(end) = end_index {
        let order = parse_order_str(&order_str)?;
        let pr_placeholder = PrPlaceholder {
            before,
            order,
            remainder: s.chars().skip(end + 1).collect::<String>(),
        };
        Ok(ParseReturn::Placeholder(pr_placeholder))
    } else {
        Ok(ParseReturn::Last(s.chars().collect::<String>()))
    }
}

// Parse a string to a Some(order).
//
// - If the string is empty, the return is 'None'.
// - If the string contains a number, the return is 'Some(number)'.
//
// Error:
// - Cannot parse the string to a usize.
fn parse_order_str(s: &str) -> Result<Option<usize>, String> {
    if s.is_empty() {
        return Ok(None);
    }

    let ord: usize = s
        .parse()
        .map_err(|_| format!("cannot parse {} to usize", s))?;

    Ok(Some(ord))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cpf_empty() {
        let cpf = CommandParseFormat::try_from("");
        assert!(cpf.is_err());
    }

    #[test]
    fn cpf_one_simple_formatter() {
        let cpf = CommandParseFormat::try_from("{}").unwrap();
        assert_eq!(cpf.number_placeholders(), 1);
        assert!(cpf.order.all_have_no_order());

        let delim_expected = CmdDelimiters::default();
        assert_eq!(cpf.delim, delim_expected);
    }

    #[test]
    fn cpf_with_last_space() {
        let cpf = CommandParseFormat::try_from("{} ").unwrap();

        assert_eq!(cpf.number_placeholders(), 1);
        assert!(cpf.order.all_have_no_order());

        let delim_expected = CmdDelimiters {
            after: " ".to_string(),
            ..Default::default()
        };
        assert_eq!(cpf.delim, delim_expected);
    }

    #[test]
    fn cpf_delimiter() {
        let cpf = CommandParseFormat::try_from("St {}, {},{}   {}end").unwrap();

        assert_eq!(cpf.number_placeholders(), 4);
        assert!(cpf.order.all_have_no_order());

        let delim_expected = CmdDelimiters {
            before: "St ".to_string(),
            between: vec![", ".to_string(), ",".to_string(), "   ".to_string()],
            after: "end".to_string(),
        };
        assert_eq!(cpf.delim, delim_expected);
    }

    #[test]
    fn cpf_get_order_all_ordered() {
        let cpf = CommandParseFormat::try_from("{2} {0} {4} {3} ASDF {1}").unwrap();

        let expected_order = [2, 0, 4, 3, 1];
        assert_eq!(cpf.order.get_sorting(), expected_order);
    }

    #[test]
    fn cpf_get_order_all_unordered() {
        let cpf = CommandParseFormat::try_from("{} {} {} {}").unwrap();
        assert_eq!(cpf.order.get_sorting(), [0, 1, 2, 3]);
    }
}
