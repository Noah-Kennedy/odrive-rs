use super::*;

#[test]
fn test_set_current() {
    let mut odrive = init_odrive();
    odrive.set_current(Axis::Zero, 24.0).unwrap();
    assert_eq!(b"c 0 24\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}

#[test]
fn test_set_trajectory() {
    let mut odrive = init_odrive();
    odrive.set_trajectory(Axis::Zero, 24.0).unwrap();
    assert_eq!(b"t 0 24\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}

#[test]
fn test_set_velocity_default() {
    let mut odrive = init_odrive();
    odrive.set_velocity(Axis::Zero, 24.0, None).unwrap();
    assert_eq!(b"v 0 24 0\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}

#[test]
fn test_set_velocity_feed_forward() {
    let mut odrive = init_odrive();
    odrive.set_velocity(Axis::Zero, 24.0, Some(12.0)).unwrap();
    assert_eq!(b"v 0 24 12\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}

#[test]
fn test_set_position_p_default() {
    let mut odrive = init_odrive();
    odrive.set_position_p(Axis::Zero, 24.0, None, None).unwrap();
    assert_eq!(b"p 0 24 0 0\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}

#[test]
fn test_set_position_q_default() {
    let mut odrive = init_odrive();
    odrive.set_position_q(Axis::Zero, 24.0, None, None).unwrap();
    assert_eq!(b"q 0 24 0 0\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}

#[test]
fn test_read_string() {
    let mut odrive = init_odrive();
    odrive.io_stream.get_mut().read_buffer.append(&mut b"hello\n".to_vec());
    odrive.io_stream.get_mut().read_buffer.reverse();
    let result = odrive.read_string().unwrap().unwrap();
    assert_eq!("hello", result);
}

#[test]
fn test_read_int() {
    let mut odrive = init_odrive();
    odrive.io_stream.get_mut().read_buffer.append(&mut b"25\n".to_vec());
    odrive.io_stream.get_mut().read_buffer.reverse();
    let result = odrive.read_int().unwrap().unwrap();
    assert_eq!(25, result);
}

#[test]
fn test_multiple_read_int() {
    let mut odrive = init_odrive();
    odrive.io_stream.get_mut().read_buffer.append(&mut b"25\n78\n".to_vec());
    odrive.io_stream.get_mut().read_buffer.reverse();
    let result = odrive.read_int().unwrap().unwrap();
    assert_eq!(25, result);
    let result = odrive.read_int().unwrap().unwrap();
    assert_eq!(78, result);
}

#[test]
fn test_read_float() {
    let mut odrive = init_odrive();
    odrive.io_stream.get_mut().read_buffer.append(&mut b"25\n".to_vec());
    odrive.io_stream.get_mut().read_buffer.reverse();
    let result = odrive.read_float().unwrap().unwrap();
    assert_eq!(25.0, result);
}

#[test]
fn test_get_velocity() {
    let mut odrive = init_odrive();
    odrive.io_stream.get_mut().read_buffer.append(&mut b"25\n".to_vec());
    odrive.io_stream.get_mut().read_buffer.reverse();
    let result = odrive.get_velocity(Axis::Zero).unwrap().unwrap();
    assert_eq!(25.0, result);
    assert_eq!(b"r axis0 .encoder.vel_estimate\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}

#[test]
fn test_run_state_instant_switch() {
    let mut odrive = init_odrive();
    odrive.io_stream.get_mut().read_buffer.append(&mut b"1\n".to_vec());
    odrive.io_stream.get_mut().read_buffer.reverse();
    let result = odrive.run_state(Axis::Zero, AxisState::MotorCalibration, true).unwrap();
    assert_eq!(true, result);
    assert_eq!(b"w axis0.requested_state 4\nr axis0.current_state\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}

#[test]
fn test_run_state_delayed_switch() {
    let mut odrive = init_odrive();
    odrive.io_stream.get_mut().read_buffer.append(&mut b"4\n1\n".to_vec());
    odrive.io_stream.get_mut().read_buffer.reverse();
    let result = odrive.run_state(Axis::Zero, AxisState::MotorCalibration, true).unwrap();
    assert_eq!(true, result);
    assert_eq!(b"w axis0.requested_state 4\nr axis0.current_state\nr axis0.current_state\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}
