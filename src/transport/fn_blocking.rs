//! Functions associated with the transport module.
//! TODO:
//! - Think about async handler

use std::io::{Read, Write};

use crate::InstrumentRsError;

pub fn write_all<I: Write>(
    interface: &mut I,
    buf: &[u8],
    terminator: &[u8],
) -> Result<(), InstrumentRsError> {
    interface.write_all(buf).unwrap();
    interface.write_all(terminator).unwrap();

    interface.flush().unwrap();

    Ok(())
}

/// Function to read until a terminator is found and then returns a Vec<u8> of what was in it.
pub fn read_until_terminator<I: Read>(
    interface: &mut I,
    terminator: &[u8],
) -> Result<Vec<u8>, InstrumentRsError> {
    let mut ret = vec![];

    let mut buf = [0u8];
    loop {
        interface.read_exact(&mut buf).unwrap();
        ret.push(buf[0]);

        if ret.len() >= terminator.len()
            && ret.get(ret.len() - terminator.len()..ret.len()).unwrap() == terminator
        {
            break;
        }
    }
    Ok(ret)
}
