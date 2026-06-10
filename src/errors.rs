//! Possible error types when working with `InstrumentRs`-derived instruments.
//!
//! TODO: This needs to be tested with at least a "string" and a "&[u8]"/"bytes" instrument to make
//! sure we have captured all possible error types (for now).

use std::{fmt::Debug, io, string};

use thiserror::Error;

/// Errors that are available in InstrumentRs.
///
/// This list gathers all errors that users might encounter when writing drivers with
/// `instrumentRs`.
/// The list is marked `[#non_exhaustive]` such that any missing errors can be added later without
/// requiring a breaking change.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum InstrumentRsError {
    /// The requested channel is outside the allowed range.
    #[error("Requested channel {req} is out of range 0..{max}.")]
    ChannelOutOfRange { req: usize, max: usize },
    /// Could not convert a UTF-8 to a String.
    #[error(transparent)]
    FromUtf8(#[from] string::FromUtf8Error),
    /// An IO error occured when communicating with the device.
    #[error(transparent)]
    Io(#[from] io::Error),
    /// Could not convert the string message returned by the instrument the into specifided type.
    /// This is specified for instruments that handle conversions with `String` or `&str`.
    #[error(
        "Could not convert the message received from the instrument {msg} into the specified type."
    )]
    BadInstrumentResponseString { msg: String },
    /// Could not convert the `Vec<u8>` message returned by the instrument the into specifided type.
    /// This is specified for instruments that handle conversions with `Vec<u8>`.
    #[error(
        "Could not convert the message received from the instrument {msg:?} into the specified type."
    )]
    BadInstrumentResponseVecU8 { msg: Vec<u8> },
}
