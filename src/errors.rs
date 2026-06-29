//! Possible error types when working with `InstrumentRs`-derived instruments.
//!
//! TODO: This needs to be tested with at least a "string" and a "&[u8]"/"bytes" instrument to make
//! sure we have captured all possible error types (for now).

use std::{ffi::os_str::Display, fmt::Debug, io, num, string};

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
    /// Parse error if parsing a float from a string fails.
    #[error(transparent)]
    ParseFloatError(#[from] num::ParseFloatError),
    /// Parse error if parsing an int from a string fails.
    #[error(transparent)]
    ParseIntError(#[from] num::ParseIntError),
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
    /// A unitful value is out of range.
    ///
    /// This is generally used as an error to show the user that that a unitful value is out of range.
    #[error(
        "Provided value {val} {unit} is out of range:\n \
            - Minimum value allowed: {val_min} {unit}\n \
            - Maximum value allowed: {val_max} {unit}"
    )]
    UnitfulValueOutOfRange {
        unit: String,
        val: f64,
        val_min: f64,
        val_max: f64,
    },
    /// A float value is out of range.
    #[error(
        "Provided float value {val} is out of range:\n \
            - Minimum value allowed: {val_min}\n \
            - Maximum value allowed: {val_max}"
    )]
    FloatValueOutOfRange {
        val: f64,
        val_min: f64,
        val_max: f64,
    },
    /// A signed integer value is out of range.
    #[error(
        "Provided integer value {val} is out of range:\n \
            - Minimum value allowed: {val_min}\n \
            - Maximum value allowed: {val_max}"
    )]
    IIntOutOfRange {
        val: isize,
        val_min: isize,
        val_max: isize,
    },
    /// A unsigned integer value is out of range.
    #[error(
        "Provided integer value {val} is out of range:\n \
            - Minimum value allowed: {val_min}\n \
            - Maximum value allowed: {val_max}"
    )]
    UIntOutOfRange {
        val: usize,
        val_min: usize,
        val_max: usize,
    },
}
