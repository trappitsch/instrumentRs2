//! Test implementation for the digoutbox.
//!
//! TODO: Fix all unwraps and return vectors properly.
//!
//! TODO: Design choices:
//! - Well, setting the device up with `Writable` types is probably bad... In a Macro, how would I
//!   pick what is a writable and what is not? Can't run `format!()` on byte slices...
//!   Might be better to set it all up as Vec<u8> commands...
//! - Do we need a check that the number of channels are in range?

use std::{io::Read, io::Write};

use instrumentrs2::{InstrumentRsError, Transport, Writable, read_until_terminator};

use crate::dev::Parameter;
pub use crate::{
    dev::DigOutBox,
    types::{Channel, Channels},
};

mod dev;
mod types;

// HERE BEGINS THE MANUAL IMPLEMENTATION

impl<I: Read + Write> DigOutBox<I> {
    fn make_pkg(&self, cmd: &str, idx: Option<usize>, args: Option<&[&str]>) -> Vec<u8> {
        // Turn command into an array of vector bytes.
        let mut cmd = Vec::from(cmd);

        // add channel if it exists
        if let Some(i) = idx {
            format!("{i}").as_bytes().iter().for_each(|b| cmd.push(*b));
        }

        // add arguments, all separated by a space as per driver description
        if let Some(inner) = args {
            for arg in inner {
                arg.as_bytes().iter().for_each(|b| {
                    cmd.push(0x20); // space
                    cmd.push(*b);
                });
            }
        } else {
            cmd.push(0x3F); // ?
        }

        // add terminator
        self.terminator.as_bytes().iter().for_each(|b| cmd.push(*b));
        cmd
    }
}

impl<I: Read + Write> Transport<&str, String> for DigOutBox<I> {
    fn sendcmd(
        &mut self,
        cmd: &str,
        idx: Option<usize>,
        args: Option<&[&str]>,
    ) -> Result<(), InstrumentRsError> {
        let cmd_vec = self.make_pkg(cmd, idx, args);
        self.interface.write_all(&cmd_vec).unwrap();
        self.interface.flush().unwrap();

        Ok(())
    }

    fn query(&mut self, cmd: &str, idx: Option<usize>) -> Result<String, InstrumentRsError> {
        let cmd_vec = self.make_pkg(cmd, idx, None);

        self.interface.write_all(&cmd_vec).unwrap();
        self.interface.flush().unwrap();

        let res =
            read_until_terminator(&mut self.interface, self.terminator.to_byte_slice()).unwrap();
        Ok(String::from_utf8(res).unwrap())
    }
}
