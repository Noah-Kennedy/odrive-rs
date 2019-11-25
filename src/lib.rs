#[cfg(test)]
#[macro_use]
extern crate static_assertions;

/// The `commands` module contains the ODrive structure, which is used to interact with the ODrive
/// protocol.
pub mod commands;

/// The `enumerations` module contains enums and constants related to different properties and
/// errors.
pub mod enumerations;

/// Tests that the API is stable using macros from the `static_assertions` crate.
#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
mod api_tests;

#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
mod test_stream;

pub mod prelude {
    pub use crate::commands::ODrive;
    pub use crate::enumerations::{AxisID, AxisState, EncoderMode, ControlMode, MotorType};
    pub use crate::enumerations::errors::{ODriveError, EncoderError, AxisError, ControllerError, MotorError, ODriveResult};
}