//! Utility functions that are used across several modules.

use std::{collections::HashSet, hash::Hash};

use proc_macro2::Span;
use quote::ToTokens;
use syn::{Attribute, LitStr, spanned::Spanned};

use crate::derive::error::ErrorAggregator;

/// A trait to indicate that a span is available for this object.
pub trait HasSpan {
    /// Get the span.
    fn get_span(&self) -> Span;
}

/// A String attribute and its location.
///
/// Manual implementations of `Eq`, `PartialEq`, and `Hash` only act on the value contained in this
/// struct and not on the span. This is useful for duplicate checking, as we want to check for
/// duplicates if the value is equal but the span is different.
#[derive(Debug, Clone)]
pub struct StringAttribute {
    pub value: String,
    pub span: Span,
}

impl TryFrom<&Attribute> for StringAttribute {
    type Error = syn::Error;

    fn try_from(attr: &Attribute) -> Result<Self, Self::Error> {
        let value = attr.parse_args::<LitStr>()?.value();
        Ok(Self {
            value,
            span: attr.meta.to_token_stream().into_iter().last().span(),
        })
    }
}

impl PartialEq for StringAttribute {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for StringAttribute {}

impl Hash for StringAttribute {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl HasSpan for StringAttribute {
    fn get_span(&self) -> Span {
        self.span
    }
}

/// Get the string content of a named attribute.
///
/// This function takes a slice of `Attribute` and finds the first occurrence of the attribute
/// with a given name. It then tries to parse this attribute as a `LitStr` and return a result of
/// this parsing.
/// For the site, provide the signature of the struct or enum.
///
/// If errors occur, they are aggregated and returned in a combined way at the end.
///
/// Error:
/// - Could not parse the argument of the attribute to a `LitStr`.
/// - The attribute could not be found.
pub fn get_named_attribute_content_string<S: Spanned>(
    attrs: &[Attribute],
    name: &str,
    site: &S,
) -> syn::Result<StringAttribute> {
    match attrs.iter().find(|&a| a.path().is_ident(name)) {
        Some(attr) => attr.try_into(),
        None => Err(syn::Error::new(
            site.span(),
            format!(
                "missing attribute for derive InstrumentParam: #[{}(...)]",
                name
            ),
        )),
    }
}

/// Check the `Vec<T>` for duplicates.
///
/// Here, T must implement `Eq`, `PartialEq`, `Hash` and `Span`. If duplicates are found, a respective
/// `syn::Error` is added to the error aggregator.
pub fn check_duplicates<T>(vals: Vec<T>, err_agg: &mut ErrorAggregator)
where
    T: PartialEq + Eq + Hash + HasSpan,
{
    let mut hs = HashSet::new();

    vals.iter().for_each(|val| {
        if !hs.insert(val) {
            err_agg.push(syn::Error::new(
                val.get_span(),
                "duplicate value found: must be unique",
            ))
        }
    });
}
