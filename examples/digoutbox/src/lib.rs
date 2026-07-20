//! Test implementation for the digoutbox.

pub use crate::{
    channel::DigOut,
    instrument::{DigOutBox, Parameter},
    types::{DigOutState, DigOutStates},
};

pub use instrumentrs::InstrumentError;

mod channel;
mod impl_instrument;
mod instrument;
mod types;
