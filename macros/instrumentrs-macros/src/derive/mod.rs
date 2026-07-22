//! Module to implement the derive macro.

use proc_macro2::TokenStream;
use syn::{Data, DeriveInput};

mod cmd;
mod enums;
mod error;
mod utils;

pub fn process(item: TokenStream) -> syn::Result<TokenStream> {
    let ast: DeriveInput = syn::parse2(item)?;

    let impl_param = match &ast.data {
        Data::Enum(data_enum) => enums::get_impl_enum(&ast, data_enum),
        _ => unimplemented!("Only implemented for enums"),
    };

    impl_param
}
