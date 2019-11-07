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

#[test]
fn test_read_string() {
    let mut odrive = init_odrive();
    odrive.io_stream.buffer.append(&mut b"hello\n".to_vec());
    odrive.io_stream.buffer.reverse();
    let result = odrive.read_string().unwrap().unwrap();
    assert_eq!("hello", result);
}

#[test]
fn test_read_int() {
    let mut odrive = init_odrive();
    odrive.io_stream.buffer.append(&mut b"25\n".to_vec());
    odrive.io_stream.buffer.reverse();
    let result = odrive.read_int().unwrap().unwrap();
    assert_eq!(25, result);
}

#[test]
fn test_multiple_read_int() {
    let mut odrive = init_odrive();
    odrive.io_stream.buffer.append(&mut b"25\n78\n".to_vec());
    odrive.io_stream.buffer.reverse();
    let result = odrive.read_int().unwrap().unwrap();
    assert_eq!(25, result);
    let result = odrive.read_int().unwrap().unwrap();
    assert_eq!(78, result);
}

#[test]
fn test_read_float() {
    let mut odrive = init_odrive();
    odrive.io_stream.buffer.append(&mut b"25\n".to_vec());
    odrive.io_stream.buffer.reverse();
    let result = odrive.read_float().unwrap().unwrap();
    assert_eq!(25.0, result);
}
