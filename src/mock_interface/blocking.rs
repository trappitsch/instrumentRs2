//! Provides a blocking mock interface to test instrument drivers.

use std::{
    io::{self, Read, Write},
    thread,
};

use thiserror::Error;

/// A mock interface for testing instrument drivers based on `InstrumentRs`.
///
/// TODO: Example
#[derive(Debug)]
pub struct MockInterface {
    /// What we expect the interface to read from the device.
    ///
    /// This is a flattened vector of all the bytes that we expect to read.
    /// While reading the vector, we will consume it.
    expected_read: Vec<u8>,
    /// Index where we are currently reading.
    ///
    /// At the end, this must be equal to the length of the vector.
    read_idx: usize,
    /// What we expect the interface to be write to the instrument.
    ///
    /// If filled, this is a flattened vector of all bytes we expect to write.
    /// We will consume this vector when writing.
    expected_write: Vec<u8>,
    /// Counter how many writes we have already done.
    ///
    /// In the end, this must be equal to the length of the expected_write.
    write_cnt: usize,
    /// Number of flushes we expect when writing.
    ///
    /// Everytime a full command is written, the interface must be flushed. Thus, this number is
    /// equal to the number of full write commands.
    flush_exp: usize,
    /// Counter of how many flushes have been called.
    ///
    /// We expect one flush to be called for every full package that is sent to the device.
    flush_cnt: usize,
}

impl MockInterface {
    /// Creates a new Mock interface with the loaded expected_reads and exected_writes.
    ///
    /// For each complete write and read call, you must supply a Vec<u8> with the expected bytes
    /// that are sent to and read from the device. The number of comlete write calls must ultimately
    /// be the number of how many times flush was called.
    pub fn new(expected_reads: Vec<Vec<u8>>, expected_writes: Vec<Vec<u8>>) -> Self {
        let expected_read = expected_reads.into_iter().flatten().collect();
        let flush_exp = expected_writes.len();
        let expected_write = expected_writes.into_iter().flatten().collect();
        Self {
            expected_read,
            read_idx: 0,
            expected_write,
            write_cnt: 0,
            flush_exp,
            flush_cnt: 0,
        }
    }

    /// Finalize the test and reset the mock interface.
    ///
    /// This will check that we read all the expected_read data, that all the expected_write data
    /// was written, and that flush was called the correct number of times. Then, the interface is
    /// reset to its original state.
    ///
    /// Finalize is automatically called when the interface is dropped. If any of the checks do not
    /// pass, it will panic with an appropriate error message of the first check that failed.
    pub fn finalize(&mut self) {
        if self.read_idx != self.expected_read.len() {
            panic!(
                "The expected read vector was not fully depleted: Remaining data we expected to read: {:?}",
                &self.expected_read[self.read_idx..]
            )
        }
        if self.write_cnt != self.expected_write.len() {
            panic!(
                "The expected write vector was not fully depleted: Remaining data we expected to write: {:?}",
                &self.expected_write[self.write_cnt..]
            )
        }
        if self.flush_exp != self.flush_cnt {
            panic!(
                "We expected the interface to be flushed {} times but it was only flushed {} times.",
                self.flush_exp, self.flush_cnt
            )
        }
    }
}

impl Drop for MockInterface {
    fn drop(&mut self) {
        // if we aleady panicked, ignore the drop checks.
        if thread::panicking() {
            return;
        }
        self.finalize();
    }
}

impl Read for MockInterface {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let end_idx = self.read_idx + buf.len();
        if end_idx > self.expected_read.len() {
            return Err(MockError::NoMoreReadData.into());
        }

        buf.copy_from_slice(&self.expected_read[self.read_idx..end_idx]);

        self.read_idx += 1;

        Ok(buf.len())
    }
}

impl Write for MockInterface {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let end_idx = self.write_cnt + buf.len();

        if end_idx > self.expected_write.len() {
            return Err(MockError::NoMoreWriteData.into());
        }

        for b in buf {
            if &self.expected_write[self.write_cnt] != b {
                return Err(MockError::UnexpectedWrite {
                    exp: self.expected_write[self.write_cnt],
                    rec: *b,
                }
                .into());
            }

            self.write_cnt += 1;
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.flush_cnt += 1;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum MockError {
    #[error("UnexpectedWrite: Write expected byte {exp:?} but received byte {rec:?}.")]
    UnexpectedWrite { exp: u8, rec: u8 },
    #[error("NoMoreReadData: The expected_read data were depleted but more data were requested.")]
    NoMoreReadData,
    #[error(
        "NoMoreWriteData: The expected_write data were depleted but more writes were performed by the instrument."
    )]
    NoMoreWriteData,
}

impl From<MockError> for io::Error {
    fn from(err: MockError) -> Self {
        let kind = match &err {
            MockError::UnexpectedWrite { exp: _, rec: _ } => io::ErrorKind::InvalidInput,
            MockError::NoMoreReadData => io::ErrorKind::InvalidData,
            MockError::NoMoreWriteData => io::ErrorKind::InvalidData,
        };
        io::Error::new(kind, err)
    }
}
