use std::env::args;
use std::path::Path;

use serialport::SerialPortSettings;

use odrive_rs::commands::ODrive;
use odrive_rs::enumerations::{AxisID, ControlMode, EncoderMode};
use std::thread::sleep;
use std::time::Duration;

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

    sleep(Duration::from_millis(200));
    odrive.erase_configuration().unwrap();

    odrive.set_motor_pole_pairs(AxisID::Zero, 15).unwrap();
    odrive.set_motor_pole_pairs(AxisID::One, 15).unwrap();
    odrive.set_motor_resistance_calib_max_voltage(AxisID::Zero, 4.0).unwrap();
    odrive.set_motor_resistance_calib_max_voltage(AxisID::One, 4.0).unwrap();
    odrive.set_motor_requested_current_range(AxisID::Zero, 25.0).unwrap();
    odrive.set_motor_requested_current_range(AxisID::One, 25.0).unwrap();
    odrive.set_motor_current_control_bandwidth(AxisID::Zero, 100.0).unwrap();
    odrive.set_motor_current_control_bandwidth(AxisID::One, 100.0).unwrap();

    odrive.set_encoder_mode(AxisID::Zero, EncoderMode::EncoderModeHall).unwrap();
    odrive.set_encoder_mode(AxisID::One, EncoderMode::EncoderModeHall).unwrap();
    odrive.set_encoder_cpr(AxisID::Zero, 90).unwrap();
    odrive.set_encoder_cpr(AxisID::One, 90).unwrap();
    odrive.set_encoder_bandwidth(AxisID::Zero, 100.0).unwrap();
    odrive.set_encoder_bandwidth(AxisID::One, 100.0).unwrap();

    odrive.set_position_gain(AxisID::Zero, 1.0).unwrap();
    odrive.set_position_gain(AxisID::One, 1.0).unwrap();

    odrive.set_velocity_gain(AxisID::Zero, 0.02).unwrap();
    odrive.set_velocity_gain(AxisID::One, 0.02).unwrap();

    odrive.set_velocity_integrator_gain(AxisID::Zero, 0.1).unwrap();
    odrive.set_velocity_integrator_gain(AxisID::One, 0.1).unwrap();

    odrive.set_velocity_limit(AxisID::Zero, 1000.0).unwrap();
    odrive.set_velocity_limit(AxisID::One, 1000.0).unwrap();

    odrive.set_control_mode(AxisID::Zero, ControlMode::VelocityControl).unwrap();
    odrive.set_control_mode(AxisID::One, ControlMode::VelocityControl).unwrap();

    odrive.save_configuration().unwrap();
}