//! Module to hold a test interface that we can use for testing.

pub use errors::MockError;
pub use sync::MockInterface;

pub mod errors;
mod macros;
mod sync;
