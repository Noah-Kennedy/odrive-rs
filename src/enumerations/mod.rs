//! This file was derived from [the python library](https://github.com/madcowswe/ODrive/blob/master/tools/odrive/enums.py)

pub mod errors;

#[repr(u8)]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum Axis {
    Zero = 0,
    One = 1,
}

#[repr(u8)]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum AxisState {
    Undefined = 0,
    Idle = 1,
    StartupSequence = 2,
    FullCalibrationSequence = 3,
    MotorCalibration = 4,
    SensorlessControl = 5,
    EncoderIndexSearch = 6,
    EncoderOffsetCalibration = 7,
    ClosedLoopControl = 8,
}

#[repr(u8)]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum MotorType {
    HighCurrent = 0,
    //LowCurrent = 1,
    MotorTypeGimbal = 2,
}

#[repr(u8)]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum ControlMode {
    VoltageControl = 0,
    CurrentControl = 1,
    VelocityControl = 2,
    PositionControl = 3,
    TrajectoryControl = 4,
}

#[repr(u8)]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum EncoderMode {
    EncoderModeIncremental = 0,
    EncoderModeHall = 1,
}