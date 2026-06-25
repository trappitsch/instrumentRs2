use std::{thread, time::Duration};

use digoutbox::{DigOut, DigOutBox, DigOutState, InstrumentRsError};

pub fn main() -> Result<(), InstrumentRsError> {
    let port = "/dev/ttyACM0";
    let baud = 9600;

    let mut interface = serialport::new(port, baud).open().unwrap();
    interface.set_timeout(Duration::from_secs(3)).unwrap();

    let mut inst = DigOutBox::new(interface);

    println!("{:?}", inst.get_name());
    inst.channel(DigOut::Out1)
        .set_channel(DigOutState::On)
        .unwrap();
    inst.channel(DigOut::Out2)
        .set_channel(DigOutState::On)
        .unwrap();
    inst.channel(DigOut::Out4)
        .set_channel(DigOutState::On)
        .unwrap();

    println!("Status ch1: {}", inst.channel(DigOut::Out1).get_channel()?);

    println!("Interlock state as bool {}", inst.get_interlock_state()?);

    println!(
        "software lockout state as usize {}",
        inst.get_software_lockout()?
    );

    thread::sleep(Duration::from_secs(1));

    let status = inst.get_all()?;
    println!("{:?}", status);

    thread::sleep(Duration::from_secs(1));
    inst.set_all_off().unwrap();

    Ok(())
}
