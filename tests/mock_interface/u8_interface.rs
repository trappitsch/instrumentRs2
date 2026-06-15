//! Tests related to reading and writing the mock interface in bytes.

use std::io::Write;

use cool_asserts::assert_panics;
use instrumentrs2::{smockb, u};

use crate::InstrumentU8;

/// Check that a write only and a write and read routine passes with the mock interface.
#[test]
fn read_write_passes() {
    let exp_reads = ["Hello".as_bytes()];
    let exp_writes = ["cmd: sendcmd".as_bytes(), "cmd query".as_bytes()];

    let mut inst = smockb!(InstrumentU8, exp_reads, exp_writes);

    // write only
    u!(inst.write_u8(exp_writes[0]));

    let rec = u!(inst.query_u8(exp_writes[1]));
    assert_eq!("Hello".as_bytes(), rec);
}

/// Panic if we have unused writes in the mock interface when it is dropped.
#[test]
fn panic_unused_writes() {
    let wrt_str = "this is an unexpected write; 42";
    let exp_reads: Vec<&[u8]> = vec![];
    let exp_writes = [wrt_str.as_bytes()];

    let inst = smockb!(InstrumentU8, exp_reads, exp_writes);
    assert_panics!(
        drop(inst),
        includes("expected write vector"),
        includes(wrt_str)
    );
}

/// Panic if we have unused reads in the mock interface when it is dropped.
#[test]
fn panic_unused_reads() {
    let rd_str = "this is an unexpected read; 42";
    let exp_reads = [rd_str.as_bytes()];
    let exp_writes: Vec<&[u8]> = vec![];

    let inst = smockb!(InstrumentU8, exp_reads, exp_writes);
    assert_panics!(
        drop(inst),
        includes("expected read vector"),
        includes(rd_str)
    );
}

/// Panic if the more flushes to the interface took place than should have happened.
#[test]
fn panic_too_many_flushes() {
    let exp: Vec<&[u8]> = vec![];

    let mut inst = smockb!(InstrumentU8, exp, exp);
    inst.interface.flush().unwrap();

    assert_panics!(
        drop(inst),
        includes("flushed 0 time(s) but it was only flushed 1")
    )
}

/// Panic if the number of flushes to the interface is smaller than the number of writes.
#[test]
fn panic_too_few_flushes() {
    let exp_read: Vec<&[u8]> = vec![];
    let exp_write = ["WRITE".as_bytes()];

    let mut inst = smockb!(InstrumentU8, exp_read, exp_write);
    inst.interface.write_all(exp_write[0]).unwrap();

    assert_panics!(
        drop(inst),
        includes("flushed 1 time(s) but it was only flushed 0")
    )
}

/// Error when no more read data were expected but the instrument requrested more.
#[test]
fn error_no_more_reads_expected() {
    let exp_read: Vec<&[u8]> = vec![];
    let exp_write = ["CMD".as_bytes()];

    let mut inst = smockb!(InstrumentU8, exp_read, exp_write);

    match inst.query_u8(exp_write[0]) {
        Err(e) => assert!(e.to_string().contains("NoMoreReadData")),
        Ok(_) => panic!("Should have returned a NoMoreReadData error."),
    }
}

/// Error when no more write data were expected but the instrument expected to send more.
#[test]
fn error_no_more_writes_expected() {
    let exp_read: Vec<&[u8]> = vec![];
    let exp_write = ["CM".as_bytes()];

    let mut inst = smockb!(InstrumentU8, exp_read, exp_write);

    match inst.write_u8("CMD".as_bytes()) {
        Err(e) => assert!(e.to_string().contains("NoMoreWriteData")),
        Ok(_) => panic!("Should have returned a NoMoreWriteData error."),
    }

    // Write is not depleted, so this must panic when dropped.
    assert_panics!(drop(inst));
}

/// Error when an unexected byte was writen to to the device.
#[test]
fn error_unexpected_write() {
    let exp_read: Vec<&[u8]> = vec![];
    let exp_write = ["CMD".as_bytes()];

    let mut inst = smockb!(InstrumentU8, exp_read, exp_write);

    match inst.write_u8("RMD".as_bytes()) {
        Err(e) => {
            let e = e.to_string();
            assert!(e.contains("UnexpectedWrite"));
            assert!(e.contains("Expected byte"));
            assert!(e.contains("Received byte"));
            assert!(e.contains("'C'"));
            assert!(e.contains("'R'"));
        }
        Ok(_) => panic!("Should have returned an UnexpectedWrite error."),
    }

    // Write is not depleted, so this must panic when dropped.
    assert_panics!(drop(inst));
}
