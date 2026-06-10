use digoutbox::{Channel, DigOutBox};

use instrumentrs2::mock_interface::MockInterface;

#[test]
fn turn_on_channel0() {
    let expected_writes = vec!["DO0 1\n".as_bytes().to_vec()];
    let interface = MockInterface::new(vec![], expected_writes);

    let mut inst = DigOutBox::new(interface);

    inst.channel(0).unwrap().set_channel(Channel::On).unwrap();
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
