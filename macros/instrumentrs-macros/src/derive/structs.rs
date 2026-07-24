//! This module handles structs.

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DeriveInput, Fields, Type};

use crate::derive::{
    cmd::{CmdDelimiters, CommandParseFormat, PlaceholderOrder},
    error,
};

/// Create the implementation for a struct.
pub fn get_impl(ast: &DeriveInput, fields: &Fields) -> syn::Result<TokenStream> {
    let mut err_agg = error::ErrorAggregator::new();

    let name = &ast.ident;

    let fields_and_types: Vec<(&Ident, &Type)> = fields
        .iter()
        .map(|f| {
            let id = f.ident.as_ref().expect("only_named_fields");
            let ty = &f.ty;
            (id, ty)
        })
        .collect();

    // This checks that the number of fields agree with the number of placeholders!
    let cpf = match CommandParseFormat::try_new_struct(
        &ast.attrs,
        &ast.ident.span(),
        fields_and_types.len(),
    ) {
        Ok(res) => res,
        Err(err) => {
            err_agg.push(err);
            CommandParseFormat::try_from("{}").expect("valid command string")
        }
    };

    // This check is already done when creating `cpf`, thus no error aggregation necessary.
    let (to_writable, try_from_writable) = if fields_and_types.len() == cpf.number_placeholders() {
        (
            get_to_writable(cpf.get_command(), &fields_and_types),
            get_struct_try_from_writable(
                cpf.get_placeholder_order(),
                cpf.get_cmd_delimiters(),
                &fields_and_types,
            ),
        )
    } else {
        (quote!(), quote!())
    };

    let ok_value = quote! {
        impl InstrumentParameter<String> for #name {
            #to_writable
            #try_from_writable
        }
    };

    err_agg.get_result(ok_value)
}

/// Get the to_writable impl of the `InstrumentParameter` trait.
///
/// Note: Before running this, make sure the number of fields and the number of placeholders agree!
fn get_to_writable(cmd: &str, fields_and_types: &[(&Ident, &Type)]) -> TokenStream {
    let fields: Vec<_> = fields_and_types.iter().map(|(id, _)| *id).collect();

    quote! {
        fn to_writable(&self) -> String {
            ::std::format!(#cmd #(,self.#fields.to_writable())*)
        }
    }
}

/// Get the try_from_writable impl of the `InstrumentParameter` trait.
///
/// Note: Before running this, make sure the number of fields and the number of placeholders agree!
fn get_struct_try_from_writable(
    order: &PlaceholderOrder,
    delims: &CmdDelimiters,
    fields_and_types: &[(&Ident, &Type)],
) -> TokenStream {
    let fields_and_types_sorted = order.sort_slice(fields_and_types);

    let CmdDelimiters {
        before,
        between,
        after,
    } = delims;

    let mut between = between.clone();
    between.push(after.clone());

    let id: Vec<&Ident> = fields_and_types_sorted.iter().map(|(i, _)| *i).collect();
    let ty: Vec<&Type> = fields_and_types_sorted.iter().map(|(_, t)| *t).collect();

    let params_index: Vec<_> = (0..order.len()).collect();

    quote! {
        fn try_from_writable(val: String) -> Result<Self, ::instrumentrs::InstrumentError> {
            let between_delims = [#(#between,)*];
            let mut value_to_parse = val.as_str();

            value_to_parse.split_once(#before).unwrap().1;

            let mut params: Vec<&str> = between_delims.iter().map(|s| {
                if s.is_empty() {
                    value_to_parse
                } else {
                    let (beg, end) = value_to_parse.split_once(s).unwrap();
                    value_to_parse = end;
                    beg
                }
            }).collect();


            #(let #id = #ty::try_from_writable(params[#params_index].to_string())?;)*

            Ok(Self {
                #(#id,)*
            })
        }
    }
}
