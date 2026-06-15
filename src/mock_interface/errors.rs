//! Holds mock interface specific errors.

use std::{
    fmt::Display,
    io::{self, Write},
};
use termcolor::{Buffer, Color, ColorSpec, WriteColor};
use thiserror::Error;

/// Errors that can be returned when reading from/writing to the Mock interface.
///
/// All of these errors are ultimately turned into `io::Errors`. If a test panics, you will not see
/// the full output of the error message. Please use the `e` and `ep` macros provided instead of
/// unwrapping your commands in tests.
#[derive(Debug, Error)]
pub enum MockError {
    /// Writing to the command encountered an unexpected byte.
    #[error(
        "UnexpectedWrite: The write command received an unexpected byte:\n\
        - Expected byte '{expected:?}' (character: '{expected_char}')\n\
        + Received byte '{recieved:?}' (character: '{received_char}')."
    )]
    UnexpectedWrite {
        expected: u8,
        expected_char: char,
        recieved: u8,
        received_char: char,
    },
    /// More data was expected but the `expected_reads` ran out of bytes.
    #[error("NoMoreReadData: The `expected_read` data were depleted but more data were requested.")]
    NoMoreReadData,
    /// More data was expected to be written to the instrument, but the `expected_writes` ran out of
    /// bytes.
    #[error(
        "NoMoreWriteData: The `expected_write` data were depleted but more writes were performed by the instrument."
    )]
    NoMoreWriteData,
    /// Pretty printing as the no error message was resent. If you see this error, please report it
    /// as a bug.
    #[error("Coloring the error message failed")]
    PrettyPrintFailed,
}

impl From<MockError> for io::Error {
    fn from(err: MockError) -> Self {
        let kind = match &err {
            MockError::UnexpectedWrite {
                expected: _,
                expected_char: _,
                recieved: _,
                received_char: _,
            } => io::ErrorKind::InvalidInput,
            MockError::NoMoreReadData => io::ErrorKind::InvalidData,
            MockError::NoMoreWriteData => io::ErrorKind::InvalidData,
            MockError::PrettyPrintFailed => io::ErrorKind::Other,
        };
        io::Error::new(kind, err)
    }
}

/// This formatter tries to format the MockErrors with colors.
///
/// This formatter looks for an error message that is styled in the form of `MockErrors`. It
/// formates the first part before the ':' in the first line red. If the error message has three
/// lines (second line exected, third line received value), it formats the second line in yellow and
/// the third line in blue.
///
/// Note: If none of these conditions are met, a `String` without any color containing the original
/// error message is returned.
///
/// If this function fails for `termcolor` reasons or if no error message was supplied, it will
/// reutrn an `io::Error`.
pub fn pretty_error<D: Display>(emsg: D) -> Result<String, io::Error> {
    let emsg = emsg.to_string();
    let lines: Vec<&str> = emsg.split("\n").collect();

    let mut buf = Buffer::ansi();

    // write first word into buffer in bold, red.
    if let Some(l1) = lines.first()
        && let Some(idx) = l1.find(':')
    {
        buf.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;

        write!(&mut buf, "{}", &l1[..idx])?;
        buf.reset().unwrap();
        writeln!(&mut buf, "{}", &l1[idx..])?;
    } else {
        return Err(MockError::PrettyPrintFailed.into());
    }

    // second line to red, third line to green if we have three line error message.
    if lines.len() == 3 {
        buf.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        writeln!(&mut buf, "{}", lines[1])?;
        buf.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
        writeln!(&mut buf, "{}", lines[2])?;
        buf.reset()?;
    }

    Ok(String::from_utf8_lossy(buf.as_slice()).into_owned())
}
