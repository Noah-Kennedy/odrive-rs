use super::*;

#[test]
fn test_set_pole_pairs() {
    let mut odrive = init_odrive();
    odrive.set_motor_pole_pairs(AxisID::Zero, 25).unwrap();
    assert_eq!(b"w axis0.motor.config.pole_pairs 25\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}

#[test]
fn test_set_resistance_calibration_max_voltage() {
    let mut odrive = init_odrive();
    odrive.set_motor_resistance_calib_max_voltage(AxisID::Zero, 25.0).unwrap();
    assert_eq!(b"w axis0.motor.config.resistance_calib_max_voltage 25\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}

#[test]
fn test_set_requested_current_range() {
    let mut odrive = init_odrive();
    odrive.set_motor_requested_current_range(AxisID::Zero, 25.0).unwrap();
    assert_eq!(b"w axis0.motor.config.requested_current_range 25\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}

#[test]
fn test_set_current_control_bandwidth() {
    let mut odrive = init_odrive();
    odrive.set_motor_current_control_bandwidth(AxisID::Zero, 25.0).unwrap();
    assert_eq!(b"w axis0.motor.config.current_control_bandwidth 25\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}