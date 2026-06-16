//! All of this should be implemented with a macro to create the async instrument interface.

use instrumentrs2::{
    InstrumentError,
    futures::{AsyncReadExt, AsyncWriteExt},
    transport::TransportAsync,
};

use crate::Parameter;

pub struct DigOutBoxAsync<I>
where
    I: AsyncReadExt + AsyncWriteExt + Unpin + Send,
{
    pub(crate) interface: I,
    pub terminator: String,
    pub num_channels: usize,
}

impl<I> DigOutBoxAsync<I>
where
    I: AsyncReadExt + AsyncWriteExt + Unpin + Send,
{
    pub fn new(interface: I) -> Self {
        Self {
            interface,
            terminator: String::from("\n"),
            num_channels: 16,
        }
    }

    pub async fn get_name(&mut self) -> Result<String, InstrumentError> {
        let a = self.query("*IDN", None, None).await?;
        String::try_from_writable(a)
    }

    // And it should continue with the rest.
    //
    // The async macro, as one can see, is going to be mostly identical to the sync macro, except for:
    // - Traits that are implemented.
    // -
}
