use std::{error::Error, time::Duration};
use std::io::prelude::*;
use serial::prelude::*;
    
fn main() {
    match run() {
        Err(e) => {
            println!("A fatal error occured:");
            println!("{}", e.to_string());
        }
        Ok(_) => {}
    }    
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut sp = serial::open("COM3")?;
    println!("Opened serial port");
    let port_settings = serial::PortSettings {
        baud_rate: serial::Baud9600,
        char_size: serial::Bits8,
        flow_control: serial::FlowNone,
        parity:     serial::ParityNone,
        stop_bits: serial::Stop1
    };
    sp.configure(&port_settings)?;
    sp.set_timeout(Duration::from_secs(2))?;
    loop {
        let mut buf: [u8; 128] = [0; 128];
        sp.read(&mut buf)?;
        print!("{}", std::str::from_utf8(&buf)?);
    }
}