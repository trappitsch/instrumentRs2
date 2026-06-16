//! Implementation of the asynchronous instrument.

use instrumentrs2::{
    InstrumentError,
    futures::{AsyncReadExt, AsyncWriteExt},
    transport::{TransportAsync, read_until_terminator_async, write_all_async},
};

use crate::{DigOut, DigOutBoxAsync, impl_instrument::make_pkg};

impl<I> TransportAsync<&str, String> for DigOutBoxAsync<I>
where
    I: AsyncReadExt + AsyncWriteExt + Unpin + Send,
{
    type Channel = DigOut;

    async fn sendcmd(
        &mut self,
        cmd: &str,
        idx: Option<DigOut>,
        args: Option<&[&str]>,
    ) -> Result<(), InstrumentError> {
        let cmd_vec = make_pkg(cmd, idx, args);
        write_all_async(&mut self.interface, &cmd_vec, self.terminator.as_bytes()).await?;
        Ok(())
    }

    async fn query(
        &mut self,
        cmd: &str,
        idx: Option<DigOut>,
        _args: Option<&[&str]>,
    ) -> Result<String, InstrumentError> {
        self.sendcmd(cmd, idx, None).await?;

        let res =
            read_until_terminator_async(&mut self.interface, self.terminator.as_bytes()).await?;
        Ok(String::from_utf8(res)?)
    }
}
