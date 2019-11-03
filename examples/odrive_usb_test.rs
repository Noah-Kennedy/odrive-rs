use std::io::{stdin, BufReader};
use std::env::args;
use odrive_rs::ascii::ODrive;
use serialport::SerialPortSettings;
use std::path::Path;

fn main() {
    let input_reader = BufReader::new(stdin());
    let args: Vec<String> = args().collect();
    let mut settings = SerialPortSettings::default();
    settings.baud_rate = 115_200;
    let serial = serialport::posix::TTYPort::open(Path::new(&args[1]), &settings);
    let odrive = ODrive::new(serial);
}