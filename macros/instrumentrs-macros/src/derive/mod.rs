//! Module to implement the derive macro.

use proc_macro2::{Span, TokenStream};
use syn::{Data, DataStruct, DeriveInput};

mod cmd;
mod enums;
mod error;
mod structs;
mod utils;

pub fn process(item: TokenStream) -> syn::Result<TokenStream> {
    let ast: DeriveInput = syn::parse2(item)?;

    match &ast.data {
        Data::Enum(data) => enums::get_impl(&ast, data),
        Data::Struct(DataStruct { fields: named, .. }) => structs::get_impl(&ast, named),
        _ => Err(syn::Error::new(
            Span::call_site(),
            "only implemented for enums and named structs.",
        )),
    }
}
