use std::env::args;
use std::io::Write;
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

use serialport::SerialPortSettings;

use odrive_rs::commands::ODrive;
use odrive_rs::enumerations::{AxisID, AxisState, ControlMode};

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

    odrive.run_state(AxisID::Zero, AxisState::ClosedLoopControl, false).unwrap();
    odrive.run_state(AxisID::One, AxisState::ClosedLoopControl, false).unwrap();

    odrive.set_control_mode(AxisID::Zero, ControlMode::VelocityControl).unwrap();
    odrive.set_control_mode(AxisID::One, ControlMode::VelocityControl).unwrap();

    loop {
        println!("Forwards");
        odrive.set_velocity(AxisID::Zero, 630.0, None).unwrap();
        odrive.set_velocity(AxisID::One, 630.0, None).unwrap();
        sleep(Duration::from_millis(5_000));

        println!("Backwards");
        odrive.set_velocity(AxisID::Zero, -630.0, None).unwrap();
        odrive.set_velocity(AxisID::One, -630.0, None).unwrap();
        sleep(Duration::from_millis(5_000));

        println!("Stop");
        odrive.set_velocity(AxisID::Zero, 0.0, None).unwrap();
        odrive.set_velocity(AxisID::One, 0.0, None).unwrap();
        sleep(Duration::from_millis(5_000));
    }
}