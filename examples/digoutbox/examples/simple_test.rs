use std::{thread, time::Duration};

use digoutbox::{Channel, DigOutBox};
use instrumentrs2::InstrumentRsError;

pub fn main() -> Result<(), InstrumentRsError> {
    let port = "/dev/ttyACM0";
    let baud = 9600;

    let mut interface = serialport::new(port, baud).open().unwrap();
    interface.set_timeout(Duration::from_secs(3)).unwrap();

    let mut dev = DigOutBox::new(interface);

    println!("{:?}", dev.get_name());
    for it in 0..14 {
        dev.channel(it)?.set_channel(Channel::On).unwrap();
    }

    println!("Status ch3: {}", dev.channel(3)?.get_channel()?);

    println!("Interlock state as bool {}", dev.get_interlock_state()?);

    println!(
        "software lockout state as usize {}",
        dev.get_software_lockout()?
    );

    thread::sleep(Duration::from_secs(1));

    let status = dev.get_all()?;
    println!("{:?}", status);

    thread::sleep(Duration::from_secs(1));
    dev.set_all_off().unwrap();

    Ok(())
}
