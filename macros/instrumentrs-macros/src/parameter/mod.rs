//! A very simple macro to just implement the Parameter trait.

use proc_macro2::TokenStream;
use quote::quote;

pub fn instrument_parameter_trait() -> TokenStream {
    quote! {
        pub trait InstrumentParameter<W: ::instrumentrs::transport::Writable>: Sized {
            fn to_writable(&self) -> W;
            fn try_from_writable(val: W) -> Result<Self, ::instrumentrs::InstrumentError>;
        }
    }
}
