//! Contains a very simple mock instrument that can be used for testing the MockInterface.

use std::io::{Read, Write};

use instrumentrs::{
    InstrumentError,
    transport::{Transport, Writable, read_until_terminator, write_all},
};

/// A mock instrument using string operations.
///
/// The default transport we implement uses &str and String for the writable and writable return
/// types. However, subfunctions in the structure itself can be used to tests sending/receiving of
/// vectors of bytes.
pub struct InstrumentStr<I: Read + Write> {
    pub interface: I,
    pub terminator: String,
}

impl<I: std::io::Read + Write> InstrumentStr<I> {
    pub fn new(interface: I) -> Self {
        Self {
            interface,
            terminator: String::from("\n"),
        }
    }

    pub fn write_str(&mut self, cmd: &str) -> Result<(), InstrumentError> {
        self.sendcmd(cmd, None, None)
    }

    pub fn query_str(&mut self, cmd: &str) -> Result<String, InstrumentError> {
        self.query(cmd, None, None)
    }
}

impl<I: Read + Write> Transport<&str, String> for InstrumentStr<I> {
    type Channel = usize;
    fn sendcmd(
        &mut self,
        cmd: &str,
        _idx: Option<usize>,
        _args: Option<&[&str]>,
    ) -> Result<(), instrumentrs::InstrumentError> {
        let buf = cmd.to_byte_slice();
        write_all(&mut self.interface, buf, self.terminator.to_byte_slice())?;
        Ok(())
    }

    fn query(
        &mut self,
        cmd: &str,
        _idx: Option<usize>,
        _args: Option<&[&str]>,
    ) -> Result<String, InstrumentError> {
        let buf = cmd.to_byte_slice();
        write_all(&mut self.interface, buf, self.terminator.to_byte_slice())?;
        let res = read_until_terminator(&mut self.interface, self.terminator.to_byte_slice())?;
        Ok(String::from_utf8(res)?)
    }
}

/// A mock instrument using byte packing.
///
/// The default transport we implement uses &str and String for the writable and writable return
/// types. However, subfunctions in the structure itself can be used to tests sending/receiving of
/// vectors of bytes.
pub struct InstrumentU8<I: Read + Write> {
    pub interface: I,
}

impl<I: std::io::Read + Write> InstrumentU8<I> {
    pub fn new(interface: I) -> Self {
        Self { interface }
    }

    pub fn write_u8(&mut self, cmd: &[u8]) -> Result<(), InstrumentError> {
        self.sendcmd(cmd, None, None)
    }

    pub fn query_u8(&mut self, cmd: &[u8]) -> Result<Vec<u8>, InstrumentError> {
        self.query(cmd, None, None)
    }
}

impl<I: Read + Write> Transport<&[u8], Vec<u8>> for InstrumentU8<I> {
    type Channel = usize;
    fn sendcmd(
        &mut self,
        cmd: &[u8],
        _idx: Option<usize>,
        _args: Option<&[&[u8]]>,
    ) -> Result<(), InstrumentError> {
        self.interface.write_all(cmd)?;
        self.interface.flush()?;
        Ok(())
    }

    /// Read always exactly 5 bytes back.
    fn query(
        &mut self,
        cmd: &[u8],
        _idx: Option<usize>,
        _args: Option<&[&[u8]]>,
    ) -> Result<Vec<u8>, InstrumentError> {
        self.sendcmd(cmd, _idx, None)?;
        let mut buf = [0u8; 5];
        self.interface.read_exact(&mut buf)?;
        Ok(buf.to_vec())
    }
}
