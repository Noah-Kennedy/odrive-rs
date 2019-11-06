use super::*;
use crate::test_stream::MockStream;

fn init_odrive() -> ODrive<MockStream> {
    let stream = MockStream::default();
    ODrive::new(stream)
}

#[test]
fn test_set_current() {
    let mut odrive = init_odrive();
    odrive.set_current(Axis::Zero, 24.0).unwrap();
    assert_eq!(b"c 0 24\n".to_vec(), odrive.io_stream.buffer);
    assert!(odrive.io_stream.flushed)
}

#[test]
fn test_set_trajectory() {
    let mut odrive = init_odrive();
    odrive.set_trajectory(Axis::Zero, 24.0).unwrap();
    assert_eq!(b"t 0 24\n".to_vec(), odrive.io_stream.buffer);
    assert!(odrive.io_stream.flushed)
}

#[test]
fn test_startup_calibration_setter() {
    let mut odrive = init_odrive();
    odrive.set_startup_motor_calibration(Axis::Zero, true).unwrap();
    assert_eq!(b"w axis0.config.startup_motor_calibration 1\n".to_vec(), odrive.io_stream.buffer);
    assert!(odrive.io_stream.flushed)
}

#[test]
fn test_startup_encoder_index_search_setter() {
    let mut odrive = init_odrive();
    odrive.set_startup_encoder_index_search(Axis::Zero, true).unwrap();
    assert_eq!(b"w axis0.config.startup_encoder_index_search 1\n".to_vec(), odrive.io_stream.buffer);
    assert!(odrive.io_stream.flushed)
}

#[test]
fn test_startup_encoder_offset_calibration_setter() {
    let mut odrive = init_odrive();
    odrive.set_startup_encoder_offset_calibration(Axis::Zero, true).unwrap();
    assert_eq!(b"w axis0.config.startup_encoder_offset_calibration 1\n".to_vec(), odrive.io_stream.buffer);
    assert!(odrive.io_stream.flushed)
}

#[test]
fn test_startup_closed_loop_control_setter() {
    let mut odrive = init_odrive();
    odrive.set_startup_closed_loop_control(Axis::Zero, true).unwrap();
    assert_eq!(b"w axis0.config.startup_closed_loop_control 1\n".to_vec(), odrive.io_stream.buffer);
    assert!(odrive.io_stream.flushed)
}

#[test]
fn test_startup_sensorless_control_setter() {
    let mut odrive = init_odrive();
    odrive.set_startup_sensorless_control(Axis::Zero, true).unwrap();
    assert_eq!(b"w axis0.config.startup_sensorless_control 1\n".to_vec(), odrive.io_stream.buffer);
    assert!(odrive.io_stream.flushed)
}

#[test]
fn test_save_configuration() {
    let mut odrive = init_odrive();
    odrive.save_configuration().unwrap();
    assert_eq!(b"ss\n".to_vec(), odrive.io_stream.buffer);
    assert!(odrive.io_stream.flushed)
}

#[test]
fn test_erase_configuration() {
    let mut odrive = init_odrive();
    odrive.erase_configuration().unwrap();
    assert_eq!(b"se\n".to_vec(), odrive.io_stream.buffer);
    assert!(odrive.io_stream.flushed)
}

#[test]
fn test_set_velocity_default() {
    let mut odrive = init_odrive();
    odrive.set_velocity(Axis::Zero, 24.0, None).unwrap();
    assert_eq!(b"v 0 24 0\n".to_vec(), odrive.io_stream.buffer);
    assert!(odrive.io_stream.flushed)
}

#[test]
fn test_set_velocity_feed_forward() {
    let mut odrive = init_odrive();
    odrive.set_velocity(Axis::Zero, 24.0, Some(12.0)).unwrap();
    assert_eq!(b"v 0 24 12\n".to_vec(), odrive.io_stream.buffer);
    assert!(odrive.io_stream.flushed)
}