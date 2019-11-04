use std::env::args;
use std::io::{BufRead, BufReader, stdin};
use std::io::Write;
use std::path::Path;

use serialport::SerialPortSettings;

use odrive_rs::ascii::ODrive;

fn main() {
    // Get CLI args
    let args: Vec<String> = args().collect();

    // Create serial port settings
    let mut settings = SerialPortSettings::default();

    // ODrive uses 115200 baud
    settings.baud_rate = 115_200;

    // Create serial port
    let serial = serialport::posix::TTYPort::open(Path::new(&args[1]), &settings).expect("Failed to open port");

    // Create odrive connection
    let mut odrive = ODrive::new(serial);

    // STDIN reader
    let mut input_reader = BufReader::new(stdin());

    println!("Welcome to the odrive terminal.");
    println!("Type a line and press enter to send a message.");
    println!("Type !exit to exit.");

    // Create line buffer
    let mut line = String::with_capacity(20);
    loop {
        // Read next line from stdin
        input_reader.read_line(&mut line).unwrap();
        let trimmed = line.trim();

        if trimmed != "!exit" {
            // Write response to stdout
            writeln!(odrive, "{}", trimmed).expect("Failed to send command to odrive!");
            let response = odrive.read_string().unwrap();
            if !response.is_empty() {
                println!("{}", response);
            }

            // clear line buffer
            line.clear()
        } else {
            break;
        }
    }
}