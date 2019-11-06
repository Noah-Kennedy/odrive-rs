use std::env::args;
use std::io::{BufRead, BufReader, stdin};
use std::io::Write;
use std::path::Path;

use serialport::SerialPortSettings;

use odrive_rs::commands::ODrive;
use odrive_rs::enumerations::{Axis, EncoderMode, ControlMode, AxisState};

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

    odrive.run_state(Axis::Zero, AxisState::MotorCalibration, true).unwrap();
    odrive.run_state(Axis::One, AxisState::MotorCalibration, true).unwrap();

    // set motor pre calibrated
    writeln!(odrive, "w axis0.motor.config.pre_calibrated 1").unwrap();
    writeln!(odrive, "w axis1.motor.config.pre_calibrated 1").unwrap();

    odrive.run_state(Axis::Zero, AxisState::EncoderOffsetCalibration, true).unwrap();
    odrive.run_state(Axis::One, AxisState::EncoderOffsetCalibration, true).unwrap();

    odrive.set_encoder_pre_calibrated(Axis::Zero, true).unwrap();
    odrive.set_encoder_pre_calibrated(Axis::One, true).unwrap();

    odrive.save_configuration();
}