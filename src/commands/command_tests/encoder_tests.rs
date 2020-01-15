use super::*;

#[test]
fn test_set_encoder_mode() {
    let mut odrive = init_odrive();
    odrive.set_encoder_mode(MotorAxis::Zero, EncoderMode::EncoderModeHall).unwrap();
    assert_eq!(b"w axis0.encoder.config.mode 1\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}

#[test]
fn test_set_encoder_cpr() {
    let mut odrive = init_odrive();
    odrive.set_encoder_cpr(MotorAxis::Zero, 50).unwrap();
    assert_eq!(b"w axis0.encoder.config.cpr 50\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}

#[test]
fn test_set_encoder_bandwidth() {
    let mut odrive = init_odrive();
    odrive.set_encoder_bandwidth(MotorAxis::Zero, 50.0).unwrap();
    assert_eq!(b"w axis0.encoder.config.bandwidth 50\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}

#[test]
fn test_set_encoder_pre_calibration() {
    let mut odrive = init_odrive();
    odrive.set_encoder_pre_calibrated(MotorAxis::Zero, true).unwrap();
    assert_eq!(b"w axis0.encoder.config.pre_calibrated 1\n".to_vec(), odrive.io_stream.get_mut().write_buffer);
    assert!(odrive.io_stream.get_mut().flushed)
}