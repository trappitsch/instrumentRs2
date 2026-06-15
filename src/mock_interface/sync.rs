//! Provides a blocking mock interface to test instrument drivers.

use std::{
    io::{Read, Write},
    thread,
};

use crate::mock_interface::errors::MockError;

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
    /// Index where we are in the write.
    write_idx: usize,
    /// Number of flushes we expect when writing.
    ///
    /// Everytime a full command is written, the interface must be flushed. Thus, this number is
    /// equal to the number of full write commands.
    flush_exp: usize,
    /// Number of flushes "received" / counted.
    ///
    /// We expect one flush to be called for every full package that is sent to the device.
    flush_rec: usize,
}

impl MockInterface {
    /// Creates a new Mock interface with the loaded expected_reads and exected_writes.
    ///
    /// For each complete write and read call, you must supply a Vec<u8> with the expected bytes
    /// that are sent to and read from the device. The number of complete write calls must ultimately
    /// be the number of how many times flush was called.
    pub fn new(expected_reads: Vec<Vec<u8>>, expected_writes: Vec<Vec<u8>>) -> Self {
        let expected_read = expected_reads.into_iter().flatten().collect();
        let flush_exp = expected_writes.len();
        let expected_write = expected_writes.into_iter().flatten().collect();
        Self {
            expected_read,
            read_idx: 0,
            expected_write,
            write_idx: 0,
            flush_exp,
            flush_rec: 0,
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
            let bytes = &self.expected_read[self.read_idx..];
            let rdbl = String::from_utf8_lossy(bytes);
            panic!(
                "The expected read vector was not fully depleted: Remaining data we expected to read: {:?} (string: '{}')",
                bytes, rdbl
            )
        }
        if self.write_idx != self.expected_write.len() {
            let bytes = &self.expected_write[self.write_idx..];
            let rdbl = String::from_utf8_lossy(bytes);
            panic!(
                "The expected write vector was not fully depleted: Remaining data we expected to write: {:?} (string: '{}')",
                bytes, rdbl
            )
        }
        if self.flush_exp != self.flush_rec {
            panic!(
                "We expected the interface to be flushed {} time(s) but it was only flushed {} time(s).",
                self.flush_exp, self.flush_rec
            )
        }
    }
}

impl Drop for MockInterface {
    fn drop(&mut self) {
        // if we aleady panicked, ignore the drop checks.
        if !thread::panicking() {
            self.finalize();
        }
    }
}

impl Read for MockInterface {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let end_idx = self.read_idx + buf.len();
        if end_idx > self.expected_read.len() {
            return Err(MockError::NoMoreReadData.into());
        }

        buf.copy_from_slice(&self.expected_read[self.read_idx..end_idx]);

        self.read_idx += buf.len();

        Ok(buf.len())
    }
}

impl Write for MockInterface {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let end_idx = self.write_idx + buf.len();

        if end_idx > self.expected_write.len() {
            return Err(MockError::NoMoreWriteData.into());
        }

        for b in buf {
            if &self.expected_write[self.write_idx] != b {
                return Err(MockError::UnexpectedWrite {
                    expected: self.expected_write[self.write_idx],
                    expected_char: self.expected_write[self.write_idx] as char,
                    recieved: *b,
                    received_char: *b as char,
                }
                .into());
            }

            self.write_idx += 1;
        }

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.flush_rec += 1;
        Ok(())
    }
}
