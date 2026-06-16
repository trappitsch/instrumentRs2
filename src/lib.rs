//! Test implementation of `InstrumentRs` for version `0.2`.
//!
//! This is my playground to test out certain things.

// Exports of internal functionality.
pub use errors::InstrumentError;

mod errors;
mod instrument;
pub mod transport;

#[cfg(feature = "mock-interface")]
pub mod mock_interface;

#[cfg(feature = "async")]
pub use futures;
