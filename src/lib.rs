/// The `commands` module contains the ODrive structure, which is used to interact with the ODrive
/// protocol.
pub mod commands;

/// The `enumerations` module contains enums and constants related to different properties and
/// errors.
mod enumerations;

pub mod properties;

#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
mod test_stream;