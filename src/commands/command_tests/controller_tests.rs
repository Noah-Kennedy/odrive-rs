use super::*;

#[test]
fn test_set_pos_gain() {
    let mut odrive = init_odrive();
    odrive.set_position_gain(MotorAxis::Zero, 24.0).unwrap();
    assert_eq!(b"w axis0.controller.config.pos_gain 24\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}

#[test]
fn test_set_vel_gain() {
    let mut odrive = init_odrive();
    odrive.set_velocity_gain(MotorAxis::Zero, 24.0).unwrap();
    assert_eq!(b"w axis0.controller.config.vel_gain 24\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}

#[test]
fn test_set_vel_integrator_gain() {
    let mut odrive = init_odrive();
    odrive.set_velocity_integrator_gain(MotorAxis::Zero, 24.0).unwrap();
    assert_eq!(b"w axis0.controller.config.vel_integrator_gain 24\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}

#[test]
fn test_set_vel_limit() {
    let mut odrive = init_odrive();
    odrive.set_velocity_limit(MotorAxis::Zero, 24.0).unwrap();
    assert_eq!(b"w axis0.controller.config.vel_limit 24\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}

#[test]
fn test_set_control_mode() {
    let mut odrive = init_odrive();
    odrive.set_control_mode(MotorAxis::Zero, ControlMode::VelocityControl).unwrap();
    assert_eq!(b"w axis0.controller.config.control_mode 2\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}
