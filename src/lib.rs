//! Test implementation of `InstrumentRs` for version `0.2`.
//!
//! This is my playground to test out certain things.

// Exports of internal functionality.
pub use errors::InstrumentRsError;
pub use transport::{Transport, read_until_terminator, writable::Writable};

mod device;
mod errors;
mod mock_interface;
mod transport;
