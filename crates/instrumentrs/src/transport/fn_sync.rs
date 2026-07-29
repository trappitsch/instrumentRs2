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

/// Function to read until a terminator is found and then returns a Vec<u8> of what was in it.
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
