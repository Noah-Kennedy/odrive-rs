use super::*;

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
