//! Implementations for the device and transport.
//!
//! This is the part that the driver-writer needs to implement for eachd river - the specifics.

// HERE BEGINS THE MANUAL IMPLEMENTATION

use std::io::{Read, Write};

use instrumentrs2::{
    InstrumentRsError,
    transport::{Transport, Writable, read_until_terminator, write_all},
};

use crate::DigOutBox;

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

        write_all(&mut self.interface, &cmd_vec, self.terminator.as_bytes())?;

        Ok(())
    }

    fn query(&mut self, cmd: &str, idx: Option<usize>) -> Result<String, InstrumentRsError> {
        let cmd_vec = self.make_pkg(cmd, idx, None);

        write_all(&mut self.interface, &cmd_vec, self.terminator.as_bytes())?;

        let res = read_until_terminator(&mut self.interface, self.terminator.to_byte_slice())?;
        Ok(String::from_utf8(res)?)
    }
}
