//! Functions associated with the transport module.
//!
//! TODO: Async
//! - Think about async handler
//! - How can we make this rt independent?

use std::io::{Read, Write};

use crate::InstrumentError;

pub fn write_all<I: Write>(
    interface: &mut I,
    buf: &[u8],
    terminator: &[u8],
) -> Result<(), InstrumentError> {
    interface.write_all(buf)?;
    interface.write_all(terminator)?;

    interface.flush()?;

    Ok(())
}

/// Function to read a defined number of bytes and return the read as a Vec<u8>.
///
/// This function is helpful when working with instruments that, e.g., submit some header that
/// contains the number of data bytes (which might be the bytes of interest) to read.
/// Arguments:
/// - interface: a &mut of the interface from which to read.
/// - num_bytes: The number of bytes that we want to read.
pub fn read_number_of_bytes<I: Read>(
    interface: &mut I,
    num_bytes: usize,
) -> Result<Vec<u8>, InstrumentError> {
    let mut ret: Vec<u8> = Vec::with_capacity(num_bytes);

    let mut buf = [0u8];
    for _ in 0..num_bytes {
        interface.read_exact(&mut buf)?;
        ret.push(buf[0]);
    }

    Ok(ret)
}

/// Function to read until a terminator is found and then returns a Vec<u8> of what was in it.
///
/// Arguments:
/// - interface: a &mut of the interface from which to read.
/// - terminator: the terminator we are looking for.
pub fn read_until_terminator<I: Read>(
    interface: &mut I,
    terminator: &[u8],
) -> Result<Vec<u8>, InstrumentError> {
    let mut ret = vec![];

    let mut buf = [0u8];
    loop {
        interface.read_exact(&mut buf)?;
        ret.push(buf[0]);

        if let Some(end) = ret.get(ret.len() - terminator.len()..ret.len())
            && end == terminator
        {
            break;
        }
    }
    Ok(ret)
}
