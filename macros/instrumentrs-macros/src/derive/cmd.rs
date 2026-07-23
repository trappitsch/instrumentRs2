//! This module takes care of formatting the command string with its arguments.
//! It also holds the functions to parse the command string.
//! TODO:
//! - Refractor all to to_writable, try_from_writables out of the command processor!
//! - Refractor so we don't have any TokenStream generation here! That should happen in structs and
//! enums.
//! - Error handling for struct -> try_from_writable

use std::collections::VecDeque;

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Attribute, Type, spanned::Spanned};

use crate::derive::utils;

#[derive(Debug)]
struct Placeholder {
    before: String,
    order: Option<usize>,
}

impl Placeholder {
    /// Check if a a placeholder has a defined order.
    fn has_order(&self) -> bool {
        self.order.is_some()
    }
}

/// A struct to parse a command, determine placeholder, and take over formatting.
#[derive(Debug, Default)]
pub struct CommandParseFormat {
    command: String,
    placeholders: Vec<Placeholder>,
    end_of_command: String,
}

impl TryFrom<&str> for CommandParseFormat {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut ph = CommandParseFormat {
            command: value.to_string(),
            placeholders: vec![],
            end_of_command: String::new(),
        };

        let mut sp = ParseReturn::Before(ph.command.clone());

        while !sp.is_last() {
            sp = ph.parse_next(sp.inner())?;
        }

        if let ParseReturn::Last(end) = sp {
            ph.end_of_command = end;
        }

        Ok(ph)
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

        let cpf = Self::try_from(sa.value.as_str()).map_err(|_| {
            syn::Error::new(
                site.span(),
                "ordered placeholders are not allowed for enums",
            )
        })?;

        if cpf.number_placeholders() != 1 || !cpf.all_have_no_order() {
            return Err(syn::Error::new(
                sa.span,
                "command string must have exactly one empty {}",
            ));
        }

        Ok(cpf)
    }

    /// Create a new `FromatCommand` for evaluating a named struct.
    ///
    /// Error:
    /// - Could not find the "cmd" attribute in teh provided attrs.
    /// - An ordered argument could not be parsed.
    /// - A mix of ordered and unordered arguments were provided.
    pub fn try_new_struct<S: Spanned>(attrs: &[Attribute], site: &S) -> syn::Result<Self> {
        let sa = utils::get_named_attribute_content_string(attrs, "cmd", site)?;

        let cpf = Self::try_from(sa.value.as_str()).map_err(|e| syn::Error::new(site.span(), e))?;

        if cpf.have_mixed_order() {
            return Err(syn::Error::new(
                site.span(),
                "a mix of positional, e.g., {0}, and non positional {} placeholder arguments is not allowed",
            ));
        }

        Ok(cpf)
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

        if !self.all_have_no_order() {
            return Err(
                "the command string has order and thus cannot be formatted in this way."
                    .to_string(),
            );
        }

        Ok(self.command.replace("{}", s))
    }

    /// Get `to_writable` TokenStream for the impl.
    ///
    /// Error:
    /// - Number of idents provided do not match number of placeholders.
    pub fn get_struct_to_writable<S: Spanned>(
        &self,
        fields: &[(&Ident, &Type)],
        site: &S,
    ) -> syn::Result<TokenStream> {
        if fields.len() != self.number_placeholders() {
            return Err(syn::Error::new(
                site.span(),
                "number of fields does not match number of placeholders",
            ));
        }

        let fields: Vec<&Ident> = fields.iter().map(|(id, _)| *id).collect();

        let cmd = &self.command;

        Ok(quote! {
            fn to_writable(&self) -> String {
                ::std::format!(#cmd #(,self.#fields.to_writable())*)
            }
        })
    }

    /// Get `try_from_writable` TokenStream for the impl.
    ///
    /// Error:
    /// - Number of idents provided do not match number of placeholders.
    pub fn get_struct_try_from_writable<S: Spanned>(
        &self,
        fields_and_types: &[(&Ident, &Type)],
        site: &S,
    ) -> syn::Result<TokenStream> {
        // sort the fields with the ordering vector
        let order = self.get_order();
        let mut pairs: Vec<_> = fields_and_types.iter().zip(order.iter()).collect();
        pairs.sort_by_key(|p| p.1);

        let fields_and_types: Vec<_> = pairs.iter().map(|p| *p.0).collect();

        let idents: Vec<_> = fields_and_types.iter().map(|f| f.0).collect();
        let tys: Vec<_> = fields_and_types.iter().map(|f| f.1).collect();
        let params_index: Vec<usize> = (0..idents.len()).collect();

        let in_betweens = self.get_in_betweens();

        Ok(quote! {
            fn try_from_writable(val: String) -> Result<Self, ::instrumentrs::InstrumentError> {
                let in_betweens = [#(#in_betweens,)*];
                let mut value_to_parse = val.as_str();

                value_to_parse = value_to_parse.split_once(in_betweens[0]).unwrap().1;

                let params: Vec<&str> = in_betweens
                    .iter()
                    .skip(1)
                    .map(|s| {
                        if s.is_empty() {
                            value_to_parse
                        } else {
                            let (beg, end) = value_to_parse.split_once(s).unwrap();
                            value_to_parse = end;
                            beg
                        }
                    })
                    .collect();

                #(let #idents = #tys::try_from_writable(params[#params_index].to_string())?;)*

                Ok(Self {
                    #(#idents,)*
                })
            }
        })
    }

    /// Do all the placeholders have an order?
    fn all_have_order(&self) -> bool {
        self.placeholders.iter().all(|f| f.has_order())
    }

    /// Check that all placeholders have no order.
    fn all_have_no_order(&self) -> bool {
        self.placeholders.iter().all(|f| !f.has_order())
    }

    /// Get the in between spacers.
    ///
    /// This includes the front and rear spacer.
    fn get_in_betweens(&self) -> Vec<&str> {
        let mut acc: Vec<&str> = self
            .placeholders
            .iter()
            .map(|f| f.before.as_str())
            .collect();
        acc.push(self.end_of_command.as_str());
        acc
    }

    /// Get order.
    fn get_order(&self) -> Vec<usize> {
        let mut new_order: Vec<usize> = (0..self.placeholders.len()).collect();

        if self.all_have_no_order() {
            return new_order;
        }

        let mut orders_in_placeholder = Vec::new();
        let mut tmp: VecDeque<usize> = VecDeque::new();

        // sort in the given order values and store the ones taken out.
        new_order
            .iter_mut()
            .zip(&self.placeholders)
            .for_each(|(current_ord, p)| {
                if let Some(ph_order) = &p.order {
                    // order given: store current value and put order in place
                    orders_in_placeholder.push(*ph_order);
                    if ph_order != current_ord {
                        tmp.push_back(*current_ord);
                    }
                    *current_ord = *ph_order;
                }
            });

        // all were replaced, we are good.
        if new_order.len() == tmp.len() {
            return new_order;
        }
        dbg!(&new_order);

        // loop through this zip again, now fix the non-ordered values
        new_order
            .iter_mut()
            .zip(&self.placeholders)
            .for_each(|(current_ord, p)| {
                if let None = &p.order
                    && let Some(first) = tmp.pop_front()
                {
                    if *current_ord >= first || orders_in_placeholder.contains(current_ord) {
                        *current_ord = first
                    } else {
                        tmp.push_front(first);
                    }
                }
            });

        new_order
    }

    /// Have mixed order?
    fn have_mixed_order(&self) -> bool {
        !self.all_have_order() && !self.all_have_no_order()
    }

    /// Get the number of placeholders.
    fn number_placeholders(&self) -> usize {
        self.placeholders.len()
    }

    fn parse_next(&mut self, s: &str) -> Result<ParseReturn, String> {
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
            self.placeholders.push(Placeholder { before, order });
            Ok(ParseReturn::Before(
                s.chars().skip(end + 1).collect::<String>(),
            ))
        } else {
            Ok(ParseReturn::Last(s.chars().collect::<String>()))
        }
    }
}

#[derive(Debug)]
enum ParseReturn {
    Before(String),
    Last(String),
}

impl ParseReturn {
    fn is_last(&self) -> bool {
        matches!(self, ParseReturn::Last(_))
    }

    fn inner(&self) -> &str {
        match self {
            Self::Before(i) => i.as_str(),
            Self::Last(i) => i.as_str(),
        }
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
        let cpf = CommandParseFormat::try_from("").unwrap();
        assert_eq!(cpf.number_placeholders(), 0);
        assert!(cpf.end_of_command.is_empty());
    }

    #[test]
    fn cpf_one_simple_formatter() {
        let cpf = CommandParseFormat::try_from("{}").unwrap();
        assert_eq!(cpf.number_placeholders(), 1);
        assert!(cpf.all_have_no_order());
        assert!(cpf.end_of_command.is_empty());
    }

    #[test]
    fn cpf_with_last_space() {
        let cpf = CommandParseFormat::try_from("{} ").unwrap();

        assert_eq!(cpf.number_placeholders(), 1);
        assert!(cpf.all_have_no_order());
        assert_eq!(cpf.end_of_command, " ");
    }

    #[test]
    fn cpf_get_whats_in_between() {
        let cpf = CommandParseFormat::try_from("St {}, {},{}   {}end").unwrap();

        assert_eq!(cpf.number_placeholders(), 4);
        assert!(cpf.all_have_no_order());
        assert_eq!(cpf.end_of_command, "end");

        let expected_in_betweens = ["St ", ", ", ",", "   ", "end"];
        let received_in_betweens = cpf.get_in_betweens();
        assert_eq!(received_in_betweens, expected_in_betweens);
    }

    #[test]
    fn cpf_get_order_all_ordered() {
        let cpf = CommandParseFormat::try_from("{2} {0} {4} {3} ASDF {1}").unwrap();

        let expected_order = [2, 0, 4, 3, 1];
        assert_eq!(cpf.get_order(), expected_order);
    }

    #[test]
    fn cpf_get_order_all_unordered() {
        let cpf = CommandParseFormat::try_from("{} {} {} {}").unwrap();
        assert_eq!(cpf.get_order(), [0, 1, 2, 3]);
    }

    #[test]
    fn cpf_get_order_mixed() {
        let cpf = CommandParseFormat::try_from("{} {3} {} {0}").unwrap();
        let expected_order = [1, 3, 2, 0];
        assert_eq!(cpf.get_order(), expected_order);

        let cpf = CommandParseFormat::try_from("{} {3} {1} {2}").unwrap();
        let expected_order = [0, 3, 1, 2];
        assert_eq!(cpf.get_order(), expected_order);

        let cpf = CommandParseFormat::try_from("{} {1} {2} {}").unwrap();
        let expected_order = [0, 1, 2, 3];
        assert_eq!(cpf.get_order(), expected_order);
    }
}
