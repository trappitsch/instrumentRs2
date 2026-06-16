//! Functions associated with the transport module.

use futures::{AsyncReadExt, AsyncWriteExt};

use crate::InstrumentRsError;

pub async fn write_all_async<I: AsyncWriteExt + Unpin>(
    interface: &mut I,
    buf: &[u8],
    terminator: &[u8],
) -> Result<(), InstrumentRsError> {
    interface.write_all(buf).await?;
    interface.write_all(terminator).await?;

    interface.flush().await?;

    Ok(())
}

/// Function to read until a terminator is found and then returns a Vec<u8> of what was in it.
pub async fn read_until_terminator_async<I: AsyncReadExt + Unpin>(
    interface: &mut I,
    terminator: &[u8],
) -> Result<Vec<u8>, InstrumentRsError> {
    let mut ret = vec![];

    let mut buf = [0u8];
    loop {
        interface.read_exact(&mut buf).await?;
        ret.push(buf[0]);

        if let Some(end) = ret.get(ret.len() - terminator.len()..ret.len())
            && end == terminator
        {
            break;
        }
    }
    Ok(ret)
}
