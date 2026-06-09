//! Test implementation for the digoutbox.
//!
//! TODO: Fix all unwraps and return vectors properly.
//!
//! TODO: Design choices:
//! - Well, setting the device up with `Writable` types is probably bad... In a Macro, how would I
//!   pick what is a writable and what is not? Can't run `format!()` on byte slices...
//!   Might be better to set it all up as Vec<u8> commands...
//! - Do we need a check that the number of channels are in range?

pub use crate::{
    dev::{DigOutBox, Parameter},
    types::{Channel, Channels},
};

pub use instrumentrs2::InstrumentRsError;

mod dev;
mod impl_dev;
mod types;
