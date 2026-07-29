//! This module handles enums.

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DataEnum, DeriveInput};

use crate::derive::{cmd::CommandParseFormat, error, utils};

/// Create the implementation for an enum.
pub fn get_impl(ast: &DeriveInput, data: &DataEnum) -> syn::Result<TokenStream> {
    let mut err_agg = error::ErrorAggregator::new();

    let name = &ast.ident;

    let cpf = match CommandParseFormat::try_new_enum(&ast.attrs, &ast.ident.span()) {
        Ok(res) => res,
        Err(err) => {
            err_agg.push(err);
            CommandParseFormat::try_from("{}").expect("valid command string")
        }
    };

    let mut string_attributes = Vec::new(); // needed duplicate checking

    let fields: Vec<(&Ident, String)> = data
        .variants
        .iter()
        .map(|v| {
            let formatter = match utils::get_named_attribute_content_string(&v.attrs, "param", &v) {
                Ok(res) => {
                    let ret_val = cpf
                        .format_with_one(&res.value)
                        .expect("checked when command parse formatter for enum was created");
                    string_attributes.push(res);
                    ret_val
                }
                Err(err) => {
                    err_agg.push(err);
                    String::new()
                }
            };
            (&v.ident, formatter)
        })
        .collect();

    // check for duplicate values
    utils::check_duplicates(string_attributes, &mut err_agg);

    let to_writable_fields = get_fields_to_writable(&fields);
    let try_from_writable_fields = get_fields_try_from_writable(&fields);

    let ok_value = quote! {
        impl InstrumentParameter<String> for #name {
            fn to_writable(&self) -> String {
                match self {
                    #(#to_writable_fields,)*
                }
            }

            fn try_from_writable(val: String) -> Result<Self, ::instrumentrs::InstrumentError> {
                match val.trim() {
                    #(#try_from_writable_fields,)*
                    _ => Err(::instrumentrs::InstrumentError::BadInstrumentResponseString {
                        msg: val.trim().to_string(),
                    })
                }
            }
        }
    };

    err_agg.get_result(ok_value)
}

/// Creates a `Vec<TokenStream>` with the fields for the `to_writable` function.
fn get_fields_to_writable(fields: &[(&Ident, String)]) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|(id, s)| quote!(Self::#id => String::from(#s)))
        .collect()
}

/// Creates a `Vec<TokenStream>` with the fields for the `try_from_writable` function.
fn get_fields_try_from_writable(fields: &[(&Ident, String)]) -> Vec<TokenStream> {
    fields
        .iter()
        .map(|(id, s)| quote!(#s => Ok(Self::#id)))
        .collect()
}
