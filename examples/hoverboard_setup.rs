use std::env::args;
use std::io::{BufRead, BufReader, stdin};
use std::io::Write;
use std::path::Path;

use serialport::SerialPortSettings;

use odrive_rs::commands::ODrive;
use odrive_rs::enumerations::{Axis, EncoderMode, ControlMode};

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

    odrive.set_motor_pole_pairs(Axis::Zero, 15).unwrap();
    odrive.set_motor_pole_pairs(Axis::One, 15).unwrap();
    odrive.set_motor_resistance_calib_max_voltage(Axis::Zero, 4.0).unwrap();
    odrive.set_motor_resistance_calib_max_voltage(Axis::One, 4.0).unwrap();
    odrive.set_motor_requested_current_range(Axis::Zero, 25.0).unwrap();
    odrive.set_motor_requested_current_range(Axis::One, 25.0).unwrap();
    odrive.set_motor_current_control_bandwidth(Axis::Zero, 100.0).unwrap();
    odrive.set_motor_current_control_bandwidth(Axis::One, 100.0).unwrap();

    odrive.set_encoder_mode(Axis::Zero, EncoderMode::EncoderModeHall).unwrap();
    odrive.set_encoder_mode(Axis::One, EncoderMode::EncoderModeHall).unwrap();
    odrive.set_encoder_cpr(Axis::Zero, 90).unwrap();
    odrive.set_encoder_cpr(Axis::One, 90).unwrap();
    odrive.set_encoder_bandwidth(Axis::Zero, 100.0).unwrap();
    odrive.set_encoder_bandwidth(Axis::One, 100.0).unwrap();

    odrive.set_position_gain(Axis::Zero, 1.0).unwrap();
    odrive.set_position_gain(Axis::One, 1.0).unwrap();

    odrive.set_velocity_gain(Axis::Zero, 0.02).unwrap();
    odrive.set_velocity_gain(Axis::One, 0.02).unwrap();

    odrive.set_velocity_integrator_gain(Axis::Zero, 0.1).unwrap();
    odrive.set_velocity_integrator_gain(Axis::One, 0.1).unwrap();

    odrive.set_velocity_limit(Axis::Zero, 1000.0).unwrap();
    odrive.set_velocity_limit(Axis::One, 1000.0).unwrap();

    odrive.set_control_mode(Axis::Zero, ControlMode::VelocityControl).unwrap();
    odrive.set_control_mode(Axis::One, ControlMode::VelocityControl).unwrap();

    odrive.save_configuration().unwrap();
}