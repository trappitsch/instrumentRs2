//! Test implementation of `InstrumentRs` for version `0.2`.
//!
//! This is my playground to test out certain things.

// Modules
pub mod instrument;
pub mod transport;

// Features
#[cfg(feature = "mock-interface")]
pub mod mock_interface;

// Reexport from the core crate
pub use instrumentrs_core as __core;
pub use instrumentrs_core::errors::InstrumentError;

// Reexport macros
pub use instrumentrs_macros::Parameter;
