//! Error types that we use, implemented with `thiserror`.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum InstrumentRsError {
    #[error("This is a placeholder error...")]
    PlaceholderError,
}
