//! Macros to work with the Mock interface.
//!
//! These macros allow for easy setup of mocking instruments and checking for test errors.

#[macro_export]
/// If your function errored, display the error with `eprintln!()`.
///
/// This is a helper Macro to use in your tests. You can use it instead of unwrapping what you send
/// to your instruent.
/// If result is an error, this macro will print the appropriate error and then panic. If all is
/// good, it will return the unpacked result.
///
/// Only available on feature "mock-interface".
macro_rules! uplain {
    ($chk:expr) => {{
        match $chk {
            Ok(i) => i,
            Err(msg) => {
                eprintln!("{msg}");
                panic!("An error (see message above) occured on indicated line.");
            }
        }
    }};
}

#[macro_export]
/// If your function errored, display a pretty error with `eprintln!()`.
///
/// This is a helper Macro to use in your tests. You can use it instead of unwrapping what you send
/// to your instruent. It will print the failure to your console.
/// If result is an error, this macro will print the appropriate error in a pretty way and then panic.
/// If all is good, it will return the unpacked result.
///
/// Only available on feature "mock-interface".
macro_rules! u {
    ($chk:expr) => {{
        match $chk {
            Ok(i) => i,
            Err(msg) => {
                if let Ok(pmsg) = $crate::mock_interface::errors::pretty_error(&msg) {
                    eprintln!("{pmsg}");
                } else {
                    eprintln!("{msg}");
                }
                panic!("An error (see message above) occured on indicated line.");
            }
        }
    }};
}

#[macro_export]
/// Create your Instrument loaded with expected reads and writes and an automatic terminator in a mock interface.
///
/// This macro is valuable for testing as it allows you to easily construct your instrument and load
/// it with the interface that contains already the expected read and write values for testing.
///
/// Only available on feature "mock-interface".
macro_rules! smock {
    ($instrument: ident, $expected_reads:expr, $expected_writes:expr, $terminator:expr) => {{
        let term_bts = $crate::transport::Writable::to_byte_slice(&$terminator).to_vec();

        let reads: Vec<Vec<u8>> = $expected_reads
            .iter()
            .map(|w| {
                let mut bts = $crate::transport::Writable::to_byte_slice(w).to_vec();
                bts.extend_from_slice(&term_bts);
                bts
            })
            .collect();
        let writes: Vec<Vec<u8>> = $expected_writes
            .iter()
            .map(|w| {
                let mut bts = $crate::transport::Writable::to_byte_slice(w).to_vec();
                bts.extend_from_slice(&term_bts);
                bts
            })
            .collect();
        let interface = $crate::mock_interface::MockInterface::new(reads, writes);
        $instrument::new(interface)
    }};
}

#[macro_export]
/// Create your Instrument loaded with expected reads and writes without an automatic terminator in a mock interface.
///
/// This macro is valuable for testing as it allows you to easily construct your instrument and load
/// it with the interface that contains already the expected read and write values for testing.
/// Note that no terminator is added. This is useful for instruments that send and receive
/// well-defined packages where the number of bytes are counted when reading.
///
/// Only available on feature "mock-interface".
macro_rules! smockb {
    ($instrument: ident, $expected_reads:expr, $expected_writes:expr) => {{
        let reads: Vec<Vec<u8>> = $expected_reads
            .iter()
            .map(|w| $crate::transport::Writable::to_byte_slice(w).to_vec())
            .collect();
        let writes: Vec<Vec<u8>> = $expected_writes
            .iter()
            .map(|w| $crate::transport::Writable::to_byte_slice(w).to_vec())
            .collect();
        let interface = $crate::mock_interface::MockInterface::new(reads, writes);
        $instrument::new(interface)
    }};
}
