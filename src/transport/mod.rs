//! Module to hold the trait and default implementations via Macros for how to package and unackage
//! commands to an instrument. We call this here the transport logic.

use std::io::Read;

use crate::{InstrumentRsError, Writable};

pub mod writable;

/// The transport trait takes a mutable reference to self in order to interact with the interface.
///
/// The two generic argumens are `C`: The type of the command that is supplied and `R`: what's
/// returned. While `C` can be a reference, `R` typically cannot as it is read from the instrument
/// and then forwarded.
pub trait Transport<W: Writable, WR: Writable> {
    fn sendcmd(
        &mut self,
        cmd: W,
        idx: Option<usize>,
        args: Option<&[W]>,
    ) -> Result<(), InstrumentRsError>;
    fn query(&mut self, cmd: W, idx: Option<usize>) -> Result<WR, InstrumentRsError>;
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
