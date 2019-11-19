use super::*;

#[test]
fn test_startup_calibration_setter() {
    let mut odrive = init_odrive();
    odrive.set_startup_motor_calibration(AxisID::Zero, true).unwrap();
    assert_eq!(b"w axis0.config.startup_motor_calibration 1\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}

#[test]
fn test_startup_encoder_index_search_setter() {
    let mut odrive = init_odrive();
    odrive.set_startup_encoder_index_search(AxisID::Zero, true).unwrap();
    assert_eq!(b"w axis0.config.startup_encoder_index_search 1\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}

#[test]
fn test_startup_encoder_offset_calibration_setter() {
    let mut odrive = init_odrive();
    odrive.set_startup_encoder_offset_calibration(AxisID::Zero, true).unwrap();
    assert_eq!(b"w axis0.config.startup_encoder_offset_calibration 1\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}

#[test]
fn test_startup_closed_loop_control_setter() {
    let mut odrive = init_odrive();
    odrive.set_startup_closed_loop_control(AxisID::Zero, true).unwrap();
    assert_eq!(b"w axis0.config.startup_closed_loop_control 1\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}

#[test]
fn test_startup_sensorless_control_setter() {
    let mut odrive = init_odrive();
    odrive.set_startup_sensorless_control(AxisID::Zero, true).unwrap();
    assert_eq!(b"w axis0.config.startup_sensorless_control 1\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}
