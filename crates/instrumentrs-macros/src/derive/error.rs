//! Provides an error aggregator and associated methods.
//!
//! This can be used to combine syn errors and return them as a combined error.

use proc_macro2::TokenStream;

#[derive(Debug, Default)]
pub struct ErrorAggregator {
    errors: Vec<syn::Error>,
}

impl ErrorAggregator {
    /// Get an empty error aggregator.
    pub fn new() -> Self {
        Default::default()
    }

    /// Push a new value to the error aggregator.
    pub fn push(&mut self, err: syn::Error) {
        self.errors.push(err);
    }

    /// Get the result of the aggregated errors.
    ///
    /// Takes the "good" value that is returned if the error aggregator is empty.
    /// Otherwise, it returns an aggregated error.
    /// Consumes the error aggregator.
    pub fn get_result(self, ok: TokenStream) -> syn::Result<TokenStream> {
        if self.errors.is_empty() {
            Ok(ok)
        } else {
            let mut ea_iter = self.errors.into_iter();
            let mut aggregated = ea_iter
                .next()
                .expect("already checked that it is not empty");

            ea_iter.for_each(|err| aggregated.combine(err));

            Err(aggregated)
        }
    }
}
