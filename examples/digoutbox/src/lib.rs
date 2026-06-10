//! Test implementation for the digoutbox.

pub use crate::{
    instrument::{DigOutBox, Parameter},
    types::{Channel, Channels},
};

pub use instrumentrs2::InstrumentRsError;

mod impl_instrument;
mod instrument;
mod types;
