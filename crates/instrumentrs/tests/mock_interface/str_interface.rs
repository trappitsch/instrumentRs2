//! Tests related to reading and writing the mock interface.

use std::io::Write;

use cool_asserts::assert_panics;
use instrumentrs::{smock, transport::Writable, u};

use crate::InstrumentStr;

static TERM: &str = "\n";

/// Check that a write only and a write and read routine passes with the mock interface.
#[test]
fn read_write_passes() {
    let exp_reads = ["answer: query"];
    let exp_writes = ["cmd: sendcmd", "cmd query"];

    let mut inst = smock!(InstrumentStr, exp_reads, exp_writes, TERM);
    // write only
    u!(inst.write_str(exp_writes[0]));

    let rec = u!(inst.query_str(exp_writes[1]));
    assert_eq!("answer: query\n", rec);
}

/// Panic if we have unused writes in the mock interface when it is dropped.
#[test]
fn panic_unused_writes() {
    let exp_reads: Vec<&str> = vec![];
    let exp_writes = ["this is an unexpected write; 42"];

    let inst = smock!(InstrumentStr, exp_reads, exp_writes, TERM);
    assert_panics!(
        drop(inst),
        includes("expected write vector"),
        includes(exp_writes[0])
    );
}

/// Panic if we have unused reads in the mock interface when it is dropped.
#[test]
fn panic_unused_reads() {
    let exp_reads = ["this is an unexpected read; 42"];
    let exp_writes: Vec<&str> = vec![];

    let inst = smock!(InstrumentStr, exp_reads, exp_writes, TERM);
    assert_panics!(
        drop(inst),
        includes("expected read vector"),
        includes(exp_reads[0])
    );
}

/// Panic if the more flushes to the interface took place than should have happened.
#[test]
fn panic_too_many_flushes() {
    let exp: Vec<&str> = vec![];

    let mut inst = smock!(InstrumentStr, exp, exp, TERM);
    inst.interface.flush().unwrap();

    assert_panics!(
        drop(inst),
        includes("flushed 0 time(s) but it was only flushed 1")
    )
}

/// Panic if the number of flushes to the interface is smaller than the number of writes.
#[test]
fn panic_too_few_flushes() {
    let exp_read: Vec<&str> = vec![];
    let exp_write = ["WRITE"];

    let mut inst = smock!(InstrumentStr, exp_read, exp_write, TERM);
    inst.interface
        .write_all(format!("{}{}", exp_write[0], TERM).to_byte_slice())
        .unwrap();

    assert_panics!(
        drop(inst),
        includes("flushed 1 time(s) but it was only flushed 0")
    )
}

/// Error when no more read data were expected but the instrument requrested more.
#[test]
fn error_no_more_reads_expected() {
    let exp_read: Vec<&str> = vec![];
    let exp_write = ["CMD"];

    let mut inst = smock!(InstrumentStr, exp_read, exp_write, TERM);

    match inst.query_str(exp_write[0]) {
        Err(e) => assert!(e.to_string().contains("NoMoreReadData")),
        Ok(_) => panic!("Should have returned a NoMoreReadData error."),
    }
}

/// Error when no more write data were expected but the instrument expected to send more.
///
/// This error is hard to actually trigger, as the `UnexpectedWrite` error takes precedence in
/// almost all cases. We remove the terminator here from being sent, otherwise the '\n' that would
/// be sent would interfere with the `NoMoreWriteData` error.
#[test]
fn error_no_more_writes_expected() {
    let exp_read: Vec<&str> = vec![];
    let exp_write = ["CM"];

    let mut inst = smock!(InstrumentStr, exp_read, exp_write, "");

    match inst.write_str("CMD") {
        Err(e) => assert!(e.to_string().contains("NoMoreWriteData")),
        Ok(_) => panic!("Should have returned a NoMoreWriteData error."),
    }

    // Write is not depleted, so this must panic when dropped.
    assert_panics!(drop(inst));
}

/// Error when an unexected byte was writen to to the device.
#[test]
fn error_unexpected_write() {
    let exp_read: Vec<&str> = vec![];
    let exp_write = ["CMD"];

    let mut inst = smock!(InstrumentStr, exp_read, exp_write, TERM);

    match inst.write_str("RMD") {
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
