//! Test implementation for the digoutbox.

pub use crate::{
    channel::DigOut,
    instrument::{DigOutBox, Parameter},
    types::{DigOutState, DigOutStates},
};

pub use instrumentrs2::InstrumentError;

mod channel;
mod impl_instrument;
mod instrument;
mod types;

#[cfg(feature = "async")]
pub use crate::instrument_async::DigOutBoxAsync;

#[cfg(feature = "async")]
mod impl_instrument_async;
#[cfg(feature = "async")]
mod instrument_async;
