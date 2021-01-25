use std::{error::Error, io::{BufRead, BufReader}, time::Duration};
use nmea::ParseResult;
use serial::prelude::*;
    
const PORT_CONFIG: serial::PortSettings = serial::PortSettings {
    baud_rate:          serial::Baud9600,
    char_size:          serial::Bits8,
    flow_control:       serial::FlowNone,
    parity:             serial::ParityNone,
    stop_bits:          serial::Stop1
};

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

    sp.configure(&PORT_CONFIG)?;
    let br = BufReader::new(sp);
    for line in br.lines() {
        match line {
            Ok(line) => process_line_verbose(&line),
            Err(e) if e.kind() == std::io::ErrorKind::TimedOut => {} ,
            Err(e) => return Err(Box::new(e))
        }        
    }
    println!("end of lines");
    Ok(())
}

fn process_line_verbose(line: &str) {
    match nmea::parse(line.as_bytes()) {
        Ok(r) => match r {
            ParseResult::GGA(data) => println!("{:?}", data),
            ParseResult::GLL(data) => println!("{:?}", data),
            ParseResult::RMC(data) => println!("{:?}", data),
            ParseResult::GSV(data) => println!("{}/{}   {:?}", data.sentence_num, data.number_of_sentences, data.sats_info),
            ParseResult::GSA(data) => println!("{:?}", data),
            ParseResult::VTG(data) => println!("{:?}", data),
            ParseResult::TXT(data) => println!("{:?}", data),
            ParseResult::Unsupported(_) => println!("Unsupported sentence: {}", line),
            
        } 
        Err(_) => {}
    }
}

fn process_line_summary(line: &str) {
    match nmea::parse(line.as_bytes()) {
        Ok(r) => match r {
            ParseResult::GGA(data) => println!("{:?}", data),
            ParseResult::GLL(data) => println!("{:?}", data),
            ParseResult::RMC(data) => println!("{:?}", data),
            ParseResult::GSV(data) => println!("{}/{}   {:?}", data.sentence_num, data.number_of_sentences, data.sats_info),
            ParseResult::GSA(data) => println!("{:?}", data),
            ParseResult::VTG(data) => println!("{:?}", data),
            ParseResult::TXT(data) => println!("{:?}", data),
            ParseResult::Unsupported(_) => println!("Unsupported sentence: {}", line),
            
        } 
        Err(_) => {}
    }
}


