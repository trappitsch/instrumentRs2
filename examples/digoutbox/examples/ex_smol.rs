//! Async example smol
#![cfg(feature = "async")]

use smol_macros::main;

use digoutbox::DigOutBoxAsync;

main! {
        async fn main(){
        let port_name = "/dev/ttyACM0";
        let baud_rate = 9600;

        let port = serialport::new(port_name, baud_rate)
            .open_async()
            .expect("Failed to open port");

        let mut inst = DigOutBoxAsync::new(port);

        println!("{}", inst.get_name().await.unwrap());
    }
}
