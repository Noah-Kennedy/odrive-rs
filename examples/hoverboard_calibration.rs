use std::env::args;
use std::io::Write;
use std::path::Path;

use serialport::SerialPortSettings;

use odrive_rs::commands::ODrive;
use odrive_rs::enumerations::{AxisID, AxisState};

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

    odrive.run_state(AxisID::Zero, AxisState::MotorCalibration, true).unwrap();
    odrive.run_state(AxisID::One, AxisState::MotorCalibration, true).unwrap();

    // set motor pre calibrated
    odrive.set_motor_pre_calibrated(AxisID::Zero, true).unwrap();
    odrive.set_motor_pre_calibrated(AxisID::One, true).unwrap();

    odrive.run_state(AxisID::Zero, AxisState::EncoderOffsetCalibration, true).unwrap();
    odrive.run_state(AxisID::One, AxisState::EncoderOffsetCalibration, true).unwrap();

    odrive.set_encoder_pre_calibrated(AxisID::Zero, true).unwrap();
    odrive.set_encoder_pre_calibrated(AxisID::One, true).unwrap();

    odrive.save_configuration().unwrap();
}