use std::io::{Read, Write};

use crate::commands::ODrive;
use crate::enumerations::AxisID;
use crate::enumerations::errors::{ODriveError, ODriveResult};

pub trait WriteableProperty<V, T> {
    fn set_value(&self, value: &V) -> ODriveResult<T>;
}

pub trait ReadableProperty<T> {
    fn get_value(&self) -> ODriveResult<T>;
}

pub struct ODriveProperties<'a, T: Read> {
    parent: &'a ODrive<T>,
}

pub struct AxisProperties<'a, T: Read> {
    parent: &'a ODriveProperties<'a, T>,
    pub motor: MotorProperties<'a, T>,
    pub encoder: Encoder,
    pub axis: AxisID,
}

impl ToString for AxisProperties {
    fn to_string(&self) -> String {
        unimplemented!()
    }
}

pub struct MotorProperties<'a, T: Read> {
    parent: &'a AxisProperties<'a, T>,
    pub config: MotorConfigProperties<'a, T>,
}

pub struct Encoder {}

pub struct MotorConfigProperties<'a, T: Read> {
    parent: &'a MotorProperties<'a, T>,
    pub pole_pairs: PolePairs<'a, T>,
}

pub struct PolePairs<'a, T: Read> {
    parent: &'a MotorConfigProperties<'a, T>,
}

impl<T: Read + Write> ReadableProperty<u32> for PolePairs<'a, T> {
    fn get_value(&self) -> Result<u32, ODriveError> {
        unimplemented!()
    }
}