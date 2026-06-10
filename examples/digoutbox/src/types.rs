//! Holds the types that input/output with DigOutBox can take.
//!
//! TODO: We need derive macros for the Parameter impls. Ideally something that can take arguments
//! like in `thiserror``, there we impl how to display, here we should impl what the writable looks like.

use std::fmt;

use crate::Parameter;
use instrumentrs2::InstrumentRsError;

/// State of the channel, is it on or off?
#[derive(Clone, Copy, Debug)]
pub enum Channel {
    /// The channel is on.
    On,
    /// The channel is off.
    Off,
}

impl Parameter<String> for Channel {
    fn to_writable(&self) -> String {
        match self {
            Channel::On => "1".to_string(),
            Channel::Off => "0".to_string(),
        }
    }

    fn try_from_writable(val: String) -> Result<Self, InstrumentRsError> {
        match val.trim() {
            "0" => Ok(Channel::Off),
            "1" => Ok(Channel::On),
            _ => Err(InstrumentRsError::BadInstrumentResponseString {
                msg: val.trim().to_string(),
            }),
        }
    }
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Channel::Off => write!(f, "Off"),
            Channel::On => write!(f, "On"),
        }
    }
}

/// State of all channels
#[derive(Clone, Debug)]
pub struct Channels {
    /// Status of channel 0.
    pub ch0: Channel,
    /// Status of channel 1.
    pub ch1: Channel,
    /// Status of channel 2.
    pub ch2: Channel,
    /// Status of channel 3.
    pub ch3: Channel,
    /// Status of channel 4.
    pub ch4: Channel,
    /// Status of channel 5.
    pub ch5: Channel,
    /// Status of channel 6.
    pub ch6: Channel,
    /// Status of channel 7.
    pub ch7: Channel,
    /// Status of channel 8.
    pub ch8: Channel,
    /// Status of channel 9.
    pub ch9: Channel,
    /// Status of channel 10.
    pub ch10: Channel,
    /// Status of channel 11.
    pub ch11: Channel,
    /// Status of channel 12.
    pub ch12: Channel,
    /// Status of channel 13.
    pub ch13: Channel,
    /// Status of channel 14.
    pub ch14: Channel,
    /// Status of channel 15.
    pub ch15: Channel,
}

impl Parameter<String> for Channels {
    fn to_writable(&self) -> String {
        unreachable!("This function is unreachable.")
    }

    fn try_from_writable(val: String) -> Result<Self, InstrumentRsError> {
        let splt = val.trim().split(',');
        let mut vals = vec![];
        for s in splt {
            match s {
                "0" => vals.push(Channel::Off),
                "1" => vals.push(Channel::On),
                _ => {
                    return Err(InstrumentRsError::BadInstrumentResponseString {
                        msg: val.trim().to_string(),
                    });
                }
            }
        }

        if vals.len() != 16 {
            return Err(InstrumentRsError::BadInstrumentResponseString {
                msg: val.trim().to_string(),
            });
        }

        Ok(Self {
            ch0: vals[0],
            ch1: vals[1],
            ch2: vals[2],
            ch3: vals[3],
            ch4: vals[4],
            ch5: vals[5],
            ch6: vals[6],
            ch7: vals[7],
            ch8: vals[8],
            ch9: vals[9],
            ch10: vals[10],
            ch11: vals[11],
            ch12: vals[12],
            ch13: vals[13],
            ch14: vals[14],
            ch15: vals[15],
        })
    }
}

impl Parameter<String> for bool {
    fn to_writable(&self) -> String {
        if *self {
            "1".to_string()
        } else {
            "0".to_string()
        }
    }

    fn try_from_writable(val: String) -> Result<Self, InstrumentRsError> {
        match val.trim() {
            "0" => Ok(false),
            "1" => Ok(true),
            _ => Err(InstrumentRsError::BadInstrumentResponseString {
                msg: val.trim().to_string(),
            }),
        }
    }
}

impl Parameter<String> for usize {
    fn to_writable(&self) -> String {
        todo!()
    }

    fn try_from_writable(val: String) -> Result<Self, InstrumentRsError> {
        match val.trim() {
            "1" => Ok(1),
            "0" => Ok(0),
            _ => Err(InstrumentRsError::BadInstrumentResponseString {
                msg: val.trim().to_string(),
            }),
        }
    }
}

// If we want to strip a string after it is returned, we need to impl this too.
//
// This is also necessary to be as general as possible!
impl Parameter<String> for String {
    fn to_writable(&self) -> String {
        String::from(self)
    }

    fn try_from_writable(val: String) -> Result<String, InstrumentRsError> {
        Ok(String::from(val.trim()))
    }
}
