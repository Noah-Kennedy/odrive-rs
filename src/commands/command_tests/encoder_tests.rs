use super::*;

#[test]
fn test_set_encoder_mode() {
    let mut odrive = init_odrive();
    odrive.set_encoder_mode(Axis::Zero, EncoderMode::EncoderModeHall).unwrap();
    assert_eq!(b"w axis0.encoder.config.mode 1\n".to_vec(), odrive.io_stream.write_buffer);
    assert!(odrive.io_stream.flushed)
}

#[test]
fn test_set_encoder_cpr() {
    let mut odrive = init_odrive();
    odrive.set_encoder_cpr(Axis::Zero, 50).unwrap();
    assert_eq!(b"w axis0.encoder.config.cpr 50\n".to_vec(), odrive.io_stream.write_buffer);
    assert!(odrive.io_stream.flushed)
}

#[test]
fn test_set_encoder_bandwidth() {
    let mut odrive = init_odrive();
    odrive.set_encoder_bandwidth(Axis::Zero, 50.0).unwrap();
    assert_eq!(b"w axis0.encoder.config.bandwidth 50\n".to_vec(), odrive.io_stream.write_buffer);
    assert!(odrive.io_stream.flushed)
}

#[test]
fn test_set_encoder_pre_calibration() {
    let mut odrive = init_odrive();
    odrive.set_encoder_pre_calibrated(Axis::Zero, true).unwrap();
    assert_eq!(b"w axis0.encoder.config.pre_calibrated 1\n".to_vec(), odrive.io_stream.write_buffer);
    assert!(odrive.io_stream.flushed)
}