use std::env::args;
use std::path::Path;

use serialport::SerialPortSettings;

use odrive_rs::commands::ODrive;
use odrive_rs::enumerations::{Axis, AxisState, ControlMode, EncoderMode};

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

    odrive.run_state(Axis::Zero, AxisState::ClosedLoopControl, true).unwrap();
    odrive.run_state(Axis::One, AxisState::ClosedLoopControl, true).unwrap();

    odrive.set_velocity(Axis::Zero, 120.0, None).unwrap();
    odrive.set_velocity(Axis::One, 120.0, None).unwrap();
}