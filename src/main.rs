use std::time::Duration;
use std::io::prelude::*;
use serial::prelude::*;
    
fn main() {
    let mut sp = serial::open("COM3").unwrap();
    println!("Opened serial port");
    let port_settings = serial::PortSettings {
        baud_rate: serial::Baud9600,
        char_size: serial::Bits8,
        flow_control: serial::FlowNone,
        parity:     serial::ParityNone,
        stop_bits: serial::Stop1
    };
    sp.configure(&port_settings).unwrap();
    sp.set_timeout(Duration::from_secs(2)).unwrap();
    loop {
        let mut buf: [u8; 128] = [0; 128];
        sp.read(&mut buf).unwrap();
        print!("{}", std::str::from_utf8(&buf).unwrap());
    }

    
}
