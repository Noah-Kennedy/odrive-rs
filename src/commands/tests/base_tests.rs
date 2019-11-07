use super::*;

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