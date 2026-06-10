//! Test implementation of `InstrumentRs` for version `0.2`.
//!
//! This is my playground to test out certain things.

// Exports of internal functionality.
pub use errors::InstrumentRsError;

mod errors;
mod instrument;
pub mod mock_interface;
pub mod transport;
