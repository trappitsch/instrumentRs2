//! Module to hold the trait and default implementations via Macros for how to package and unackage
//! commands to an instrument. We call this here the transport logic.

use crate::InstrumentRsError;
pub use fn_blocking::{read_until_terminator, write_all};
pub use writable::Writable;

pub mod fn_blocking;
pub mod writable;

/// The transport trait takes a mutable reference to self in order to interact with the interface.
///
/// The two generic argumens are `C`: The type of the command that is supplied and `R`: what's
/// returned. While `C` can be a reference, `R` typically cannot as it is read from the instrument
/// and then forwarded.
pub trait Transport<W: Writable, WR: Writable> {
    /// The send command that you need to implement.
    fn sendcmd(
        &mut self,
        cmd: W,
        idx: Option<usize>,
        args: Option<&[W]>,
    ) -> Result<(), InstrumentRsError>;

    /// The query command that you need to implement.
    fn query(&mut self, cmd: W, idx: Option<usize>) -> Result<WR, InstrumentRsError>;
}
