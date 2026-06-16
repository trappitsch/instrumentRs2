//! Implementation of the asynchronous instrument.

use instrumentrs2::{
    InstrumentRsError,
    futures::{AsyncReadExt, AsyncWriteExt},
    transport::{TransportAsync, read_until_terminator_async, write_all_async},
};

use crate::DigOutBoxAsync;

impl<I> DigOutBoxAsync<I>
where
    I: AsyncReadExt + AsyncWriteExt + Unpin + Send,
{
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

impl<I> TransportAsync<&str, String> for DigOutBoxAsync<I>
where
    I: AsyncReadExt + AsyncWriteExt + Unpin + Send,
{
    async fn sendcmd(
        &mut self,
        cmd: &str,
        idx: Option<usize>,
        args: Option<&[&str]>,
    ) -> Result<(), InstrumentRsError> {
        let cmd_vec = self.make_pkg(cmd, idx, args);
        write_all_async(&mut self.interface, &cmd_vec, self.terminator.as_bytes()).await?;
        Ok(())
    }

    async fn query(&mut self, cmd: &str, idx: Option<usize>) -> Result<String, InstrumentRsError> {
        self.sendcmd(cmd, idx, None).await?;

        let res =
            read_until_terminator_async(&mut self.interface, self.terminator.as_bytes()).await?;
        Ok(String::from_utf8(res)?)
    }
}
