//! The transport module.
//!
//! The driver author will use this module to aid in implemented how command packages must be formed
//! in order to send/receive them to/from the device.

use crate::InstrumentError;
pub use fn_sync::{read_until_terminator, write_all};
pub use writable::Writable;

pub mod fn_sync;
pub mod writable;

/// The transport trait takes a mutable reference to self in order to interact with the interface.
///
/// The two generic argumens are `C`: The type of the command that is supplied and `R`: what's
/// returned. While `C` can be a reference, `R` typically cannot as it is read from the instrument
/// and then forwarded.
pub trait Transport<W: Writable, WR: Writable> {
    type Channel;
    /// The send command that you need to implement.
    fn sendcmd(
        &mut self,
        cmd: W,
        idx: Option<Self::Channel>,
        args: Option<&[W]>,
    ) -> Result<(), InstrumentError>;

    /// The query command that you need to implement.
    fn query(
        &mut self,
        cmd: W,
        idx: Option<Self::Channel>,
        args: Option<&[W]>,
    ) -> Result<WR, InstrumentError>;
}
