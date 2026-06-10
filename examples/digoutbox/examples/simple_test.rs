use std::{thread, time::Duration};

use digoutbox::{Channel, DigOutBox, InstrumentRsError};

pub fn main() -> Result<(), InstrumentRsError> {
    let port = "/dev/ttyACM0";
    let baud = 9600;

    let mut interface = serialport::new(port, baud).open().unwrap();
    interface.set_timeout(Duration::from_secs(3)).unwrap();

    let mut inst = DigOutBox::new(interface);

    println!("{:?}", inst.get_name());
    for it in 0..14 {
        inst.channel(it)?.set_channel(Channel::On).unwrap();
    }

    println!("Status ch3: {}", inst.channel(3)?.get_channel()?);

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
