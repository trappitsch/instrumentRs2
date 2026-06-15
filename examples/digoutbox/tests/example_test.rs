use digoutbox::{Channel, DigOutBox};

use instrumentrs2::mock_interface::MockInterface;
use instrumentrs2::{smock, u, uplain};

#[test]
fn turn_on_channel0() {
    let expected_writes = vec!["DO0 1\n".as_bytes().to_vec()];
    let interface = MockInterface::new(vec![], expected_writes);

    let mut inst = DigOutBox::new(interface);

    inst.channel(0).unwrap().set_channel(Channel::On).unwrap();
}

#[test]
fn test_on_channel0_with_macro() {
    let expected_writes = ["DO0 1", "DO2 0"];
    let expected_reads: [&str; 0] = [];
    let terminator = "\n";

    // We can get the device from our sync mock interface (smock).
    let mut inst = smock![DigOutBox, expected_reads, expected_writes, terminator];

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
