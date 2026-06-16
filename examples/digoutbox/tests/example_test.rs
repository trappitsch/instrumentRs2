use digoutbox::{Channel, DigOutBox};

use instrumentrs2::mock_interface::MockInterface;
use instrumentrs2::{smock, u, uplain};

static TERM: &str = "\n";

#[test]
fn turn_on_channel3() {
    let expected_writes = vec!["DO3 1\n".as_bytes().to_vec()];
    let interface = MockInterface::new(vec![], expected_writes);

    let mut inst = DigOutBox::new(interface);

    inst.channel(3).unwrap().set_channel(Channel::On).unwrap();
}

#[test]
fn read_channel2_on() {
    let expected_writes = vec!["DO2?\n".as_bytes().to_vec()];
    let expected_reads = vec!["1\n".as_bytes().to_vec()];
    let interface = MockInterface::new(expected_reads, expected_writes);

    let mut inst = DigOutBox::new(interface);

    let ch2_state = inst.channel(2).unwrap().get_channel().unwrap();
    std::assert_matches!(ch2_state, Channel::On);
}

#[test]
fn read_channel2_macro() {
    let expected_writes = ["DO2?"];
    let expected_reads = ["1"];

    let mut inst = smock!(DigOutBox, expected_reads, expected_writes, TERM);

    let ch2_state = inst.channel(2).unwrap().get_channel().unwrap();
    std::assert_matches!(ch2_state, Channel::On);
}

#[test]
fn read_channel2_macro_pretty() {
    let expected_writes = ["DO2?"];
    let expected_reads = ["1"];

    let mut inst = smock!(DigOutBox, expected_reads, expected_writes, TERM);

    let ch2_state = u!(inst.channel(2).unwrap().get_channel());
    std::assert_matches!(ch2_state, Channel::On);
}

#[test]
fn test_on_channel0_with_macro() {
    let expected_writes = ["DO0 1", "DO2 0"];
    let expected_reads: [&str; 0] = [];

    // We can get the device from our sync mock interface (smock).
    let mut inst = smock![DigOutBox, expected_reads, expected_writes, TERM];

    // If an error occurs you can print it in pretty fashion if your terminal supports colors...
    u![inst.channel(0).unwrap().set_channel(Channel::On)];

    // ... or in regular, uncolored, fashion. The pretty version, if it fails for any reasons, defaults to this.
    uplain![inst.channel(2).unwrap().set_channel(Channel::Off)];
}

#[test]
fn get_name() {
    let expected_writes = vec!["*IDN?\n".as_bytes().to_vec()];
    let expected_reads = vec!["DigOutBox\n".as_bytes().to_vec()];
    let interface = MockInterface::new(expected_reads, expected_writes);

    let mut inst = DigOutBox::new(interface);

    let name = inst.get_name().unwrap();
    assert_eq!("DigOutBox", name);
}
