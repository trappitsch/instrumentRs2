//! All of these things should be implemented with a macro!

use std::io::{Read, Write};

/// The parameter trait implements the methods to transform any of your enums or structs into / from
/// a `Writable` parameter.
use instrumentrs2::{InstrumentRsError, Transport, Writable};

use crate::{Channel, Channels};

pub trait Parameter<W: Writable>: Sized {
    fn to_writable(&self) -> W;
    fn try_from_writable(val: W) -> Result<Self, InstrumentRsError>;
}

pub struct DigOutBox<I: Read + Write> {
    pub(crate) interface: I,
    pub terminator: String,
    pub num_channels: usize,
}

impl<I: Read + Write> DigOutBox<I> {
    pub fn new(interface: I) -> Self {
        Self {
            interface,
            terminator: String::from("\n"),
            num_channels: 16,
        }
    }

    pub fn channel(&mut self, idx: usize) -> Result<DigOutBoxChannel<'_, I>, InstrumentRsError> {
        if idx >= self.num_channels {
            Err(InstrumentRsError::PlaceholderError)
        } else {
            Ok(DigOutBoxChannel::new(idx, self))
        }
    }

    pub fn set_all_off(&mut self) -> Result<(), InstrumentRsError> {
        self.sendcmd("ALLOFF", None, Some(&[]))
    }

    /// Get all channels in its own structure.
    pub fn get_all(&mut self) -> Result<Channels, InstrumentRsError> {
        let a = self.query("ALLDO", None)?;
        Channels::try_from_writable(a)
    }

    /// Bool demo - intentionally not newtyped.
    pub fn get_interlock_state(&mut self) -> Result<bool, InstrumentRsError> {
        let a = self.query("INTERLOCKS", None)?;
        bool::try_from_writable(a)
    }

    /// Integer demo - software lockout state.
    pub fn get_software_lockout(&mut self) -> Result<usize, InstrumentRsError> {
        let a = self.query("SWL", None)?;
        usize::try_from_writable(a)
    }

    pub fn get_name(&mut self) -> Result<String, InstrumentRsError> {
        self.query("*IDN", None)
    }

    pub fn get_terminator(&self) -> &str {
        &self.terminator
    }
}

pub struct DigOutBoxChannel<'a, I: Read + Write> {
    device: &'a mut DigOutBox<I>,
    idx: usize,
}

impl<'d, I: Read + Write> DigOutBoxChannel<'d, I> {
    /// Returns a new channel. This can only be done through the device.
    fn new(idx: usize, device: &'d mut DigOutBox<I>) -> Self {
        Self { device, idx }
    }

    /// Newtype demo for one channel.
    pub fn get_channel(&mut self) -> Result<Channel, InstrumentRsError> {
        let a = self.device.query("DO", Some(self.idx))?;
        Channel::try_from_writable(a)
    }

    pub fn set_channel(&mut self, val: Channel) -> Result<(), InstrumentRsError> {
        self.device
            .sendcmd("DO", Some(self.idx), Some(&[&val.to_writable()]))
    }
}
