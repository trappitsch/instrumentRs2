//! Test implementation for the digoutbox.

pub use crate::{
    instrument::{DigOutBox, Parameter},
    types::{Channel, Channels},
};

pub use instrumentrs2::InstrumentRsError;

mod impl_instrument;
mod instrument;
mod types;

#[cfg(feature = "async")]
pub use crate::instrument_async::DigOutBoxAsync;

#[cfg(feature = "async")]
mod impl_instrument_async;
#[cfg(feature = "async")]
mod instrument_async;
