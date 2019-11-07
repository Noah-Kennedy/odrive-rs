use super::*;
use crate::test_stream::MockStream;

#[cfg(test)]
mod startup_tests;

#[cfg(test)]
mod configuration_tests;

#[cfg(test)]
mod base_tests;

#[cfg(test)]
mod controller_tests;

fn init_odrive() -> ODrive<MockStream> {
    let stream = MockStream::default();
    ODrive::new(stream)
}