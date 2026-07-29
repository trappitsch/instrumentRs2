//! Holds the types that input/output with DigOutBox can take.

use std::fmt;

use crate::{InstrumentError, InstrumentParameter};
use instrumentrs::Parameter;

/// State of the channel, is it on or off?
#[derive(Clone, Copy, Debug, Parameter)]
#[cmd("{}")]
pub enum DigOutState {
    /// The channel is on.
    #[param("1")]
    On,
    /// The channel is off.
    #[param("0")]
    Off,
}

impl fmt::Display for DigOutState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DigOutState::Off => write!(f, "Off"),
            DigOutState::On => write!(f, "On"),
        }
    }
}

/// State of all channels
#[derive(Clone, Debug, Parameter)]
#[cmd("{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}")]
pub struct DigOutStates {
    /// Status of channel 0.
    pub ch0: DigOutState,
    /// Status of channel 1.
    pub ch1: DigOutState,
    /// Status of channel 2.
    pub ch2: DigOutState,
    /// Status of channel 3.
    pub ch3: DigOutState,
    /// Status of channel 4.
    pub ch4: DigOutState,
    /// Status of channel 5.
    pub ch5: DigOutState,
    /// Status of channel 6.
    pub ch6: DigOutState,
    /// Status of channel 7.
    pub ch7: DigOutState,
    /// Status of channel 8.
    pub ch8: DigOutState,
    /// Status of channel 9.
    pub ch9: DigOutState,
    /// Status of channel 10.
    pub ch10: DigOutState,
    /// Status of channel 11.
    pub ch11: DigOutState,
    /// Status of channel 12.
    pub ch12: DigOutState,
    /// Status of channel 13.
    pub ch13: DigOutState,
    /// Status of channel 14.
    pub ch14: DigOutState,
    /// Status of channel 15.
    pub ch15: DigOutState,
}

impl InstrumentParameter<String> for bool {
    fn to_writable(&self) -> String {
        if *self {
            "1".to_string()
        } else {
            "0".to_string()
        }
    }

    fn try_from_writable(val: String) -> Result<Self, InstrumentError> {
        match val.trim() {
            "0" => Ok(false),
            "1" => Ok(true),
            _ => Err(InstrumentError::BadInstrumentResponseString {
                msg: val.trim().to_string(),
            }),
        }
    }
}

impl InstrumentParameter<String> for usize {
    fn to_writable(&self) -> String {
        todo!()
    }

    fn try_from_writable(val: String) -> Result<Self, InstrumentError> {
        match val.trim() {
            "1" => Ok(1),
            "0" => Ok(0),
            _ => Err(InstrumentError::BadInstrumentResponseString {
                msg: val.trim().to_string(),
            }),
        }
    }
}

// If we want to strip a string after it is returned, we need to impl this too.
//
// This is also necessary to be as general as possible!
impl InstrumentParameter<String> for String {
    fn to_writable(&self) -> String {
        String::from(self)
    }

    fn try_from_writable(val: String) -> Result<String, InstrumentError> {
        Ok(String::from(val.trim()))
    }
}
