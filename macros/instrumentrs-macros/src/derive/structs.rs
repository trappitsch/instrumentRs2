//! This module handles structs.

use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{DeriveInput, Fields, Type};

use crate::derive::{cmd::CommandParseFormat, error, utils};

/// Create the implementation for a struct.
pub fn get_impl(ast: &DeriveInput, fields: &Fields) -> syn::Result<TokenStream> {
    let mut err_agg = error::ErrorAggregator::new();

    let name = &ast.ident;

    let cpf = match CommandParseFormat::try_new_struct(&ast.attrs, &ast.ident.span()) {
        Ok(res) => res,
        Err(err) => {
            err_agg.push(err);
            CommandParseFormat::try_from("{}").expect("valid command string")
        }
    };

    let fields_and_types: Vec<(&Ident, &Type)> = fields
        .iter()
        .map(|f| {
            let id = f.ident.as_ref().expect("only_named_fields");
            let ty = &f.ty;
            (id, ty)
        })
        .collect();

    let to_writable = match cpf.get_struct_to_writable(&fields_and_types, &ast.ident.span()) {
        Ok(res) => res,
        Err(err) => {
            err_agg.push(err);
            quote!()
        }
    };

    let try_from_writable =
        match cpf.get_struct_try_from_writable(&fields_and_types, &ast.ident.span()) {
            Ok(res) => res,
            Err(err) => {
                err_agg.push(err);
                quote!()
            }
        };

    let ok_value = quote! {
        impl InstrumentParameter<String> for #name {
            #to_writable
            #try_from_writable
        }
    };

    err_agg.get_result(ok_value)
}

//     fn try_from_writable(val: String) -> Result<Self, InstrumentError> {
//         let split_vals = val.trim().split(',').collect::<Vec<&str>>();
//         if split_vals.len() != 4 {
//             return Err(InstrumentError::BadInstrumentResponseString { msg: val });
//         }
//
//         let resistance = HeaterResistance::try_from_writable(split_vals[0].into())?;
//         let max_current = HeaterMaxOutputCurrent::try_from_writable(format!(
//             "{},{}",
//             split_vals[1], split_vals[2]
//         ))?;
//         let display = HeaterOutputDisplay::try_from_writable(split_vals[3].into())?;
//
//         Ok(Self {
//             resistance,
//             max_current,
//             display,
//         })
//     }
// }
