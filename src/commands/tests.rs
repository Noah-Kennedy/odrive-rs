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
    println!("{:?}", odrive.io_stream.buffer.as_slice());
    assert_eq!(b"c 0 24\n".to_vec(), odrive.io_stream.buffer);
    assert!(odrive.io_stream.flushed)
}

#[test]
fn test_set_trajectory() {
    let mut odrive = init_odrive();
    odrive.set_trajectory(Axis::Zero, 24.0).unwrap();
    println!("{:?}", odrive.io_stream.buffer.as_slice());
    assert_eq!(b"t 0 24\n".to_vec(), odrive.io_stream.buffer);
    assert!(odrive.io_stream.flushed)
}