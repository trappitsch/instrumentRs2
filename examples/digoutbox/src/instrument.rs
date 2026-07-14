//! All of these things should be implemented with a macro!

use std::io::{Read, Write};

/// The parameter trait implements the methods to transform any of your enums or structs into / from
/// a `Writable` parameter.
use crate::InstrumentError;
use instrumentrs2::transport::{Transport, Writable};

use crate::{DigOut, DigOutState, DigOutStates};

pub trait Parameter<W: Writable>: Sized {
    fn to_writable(&self) -> W;
    fn try_from_writable(val: W) -> Result<Self, InstrumentError>;
}

pub struct DigOutBox<I: Read + Write> {
    pub(crate) interface: I,
    pub terminator: String,
}

impl<I: Read + Write> DigOutBox<I> {
    pub fn new(interface: I) -> Self {
        Self {
            interface,
            terminator: String::from("\n"),
        }
    }

    pub fn channel(&mut self, idx: DigOut) -> DigOutBoxChannel<'_, I> {
        DigOutBoxChannel::new(idx, self)
    }

    pub fn set_all_off(&mut self) -> Result<(), InstrumentError> {
        self.sendcmd("ALLOFF", None, Some(&[]))
    }

    /// Get all channels in its own structure.
    pub fn get_all(&mut self) -> Result<DigOutStates, InstrumentError> {
        let a = self.query("ALLDO", None, None)?;
        DigOutStates::try_from_writable(a)
    }

    /// Bool demo - intentionally not newtyped.
    pub fn get_interlock_state(&mut self) -> Result<bool, InstrumentError> {
        let a = self.query("INTERLOCKS", None, None)?;
        bool::try_from_writable(a)
    }

    /// Integer demo - software lockout state.
    pub fn get_software_lockout(&mut self) -> Result<usize, InstrumentError> {
        let a = self.query("SWL", None, None)?;
        usize::try_from_writable(a)
    }

    pub fn get_name(&mut self) -> Result<String, InstrumentError> {
        let a = self.query("*IDN", None, None)?;
        String::try_from_writable(a)
    }

    pub fn get_terminator(&self) -> &str {
        &self.terminator
    }
}

pub struct DigOutBoxChannel<'a, I: Read + Write> {
    device: &'a mut DigOutBox<I>,
    idx: DigOut,
}

impl<'d, I: Read + Write> DigOutBoxChannel<'d, I> {
    /// Returns a new channel. This can only be done through the device.
    fn new(idx: DigOut, device: &'d mut DigOutBox<I>) -> Self {
        Self { device, idx }
    }

    /// Newtype demo for one channel.
    pub fn get_channel(&mut self) -> Result<DigOutState, InstrumentError> {
        let a = self.device.query("DO", Some(self.idx), None)?;
        DigOutState::try_from_writable(a)
    }

    pub fn set_channel(&mut self, val: DigOutState) -> Result<(), InstrumentError> {
        self.device
            .sendcmd("DO", Some(self.idx), Some(&[&val.to_writable()]))
    }
}
