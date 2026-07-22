use proc_macro::TokenStream;

mod derive;
mod parameter;

#[proc_macro]
pub fn __instrument_parameter(_: TokenStream) -> TokenStream {
    parameter::instrument_parameter_trait().into()
}

#[proc_macro_derive(Parameter, attributes(param, cmd))]
pub fn instrumentrs_parameter(item: TokenStream) -> TokenStream {
    match derive::process(item.into()) {
        Ok(res) => res.into(),
        Err(err) => err.into_compile_error().into(),
    }
}
