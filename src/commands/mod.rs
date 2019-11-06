use std::fmt::Display;
use std::io::{Error, Read, Write};
use std::io;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::enumerations::{Axis, AxisState, ControlMode, EncoderMode};
use crate::enumerations::errors::{ODriveError, ODriveResult};

#[cfg(test)]
#[cfg_attr(tarpaulin, skip)]
mod tests;

/// The `ODrive` struct manages a connection with an ODrive motor over the ASCII protocol.
/// It acts as a newtype around a connection stream.
/// This has been tested using serial types from `serialport-rs`.
#[derive(Debug, Default, Ord, PartialOrd, Eq, PartialEq, Clone)]
pub struct ODrive<T> {
    io_stream: T
}

impl<T> ODrive<T> {
    /// Although any type can be passed in here, it is suggested that the supplied type `T` be
    /// `Read + Write`. Doing so will unlock the full API.
    pub fn new(io_stream: T) -> Self {
        Self { io_stream }
    }
}

/// An implementation of `Write` has been provided as an escape hatch to enable the usage of
/// operations not yet supported by this library.
impl<T> Write for ODrive<T> where T: Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.io_stream.write(buf)
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.io_stream.flush()
    }
}

/// An implementation of `Write` has been provided as an escape hatch to enable the usage of
/// operations not yet supported by this library. Be advised that using this implementation may
/// place the connection into an inconsistent state.
impl<T> Read for ODrive<T> where T: Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        self.io_stream.read(buf)
    }
}

impl<T> ODrive<T> where T: Read {
    /// Reads the next message sent by the ODrive as a string.
    /// If their is no message, this function should return an empty string.
    pub fn read_string(&mut self) -> io::Result<Option<String>> {
        let mut string = String::with_capacity(20);
        let duration = Instant::now();
        loop {
            let mut buffer = [0; 1];
            while self.io_stream.read(&mut buffer).unwrap_or_default() == 0 {
                if duration.elapsed().as_millis() >= 1_000 {
                    return Ok(None);
                }
            }
            let ch = buffer[0];
            if ch as char == '\n' {
                break;
            }

            string.push(ch as char);
        }

        Ok(Some(string.trim().to_owned()))
    }

    /// Reads the next message as a float. This will return zero if the message is not a valid
    /// float.
    pub fn read_float(&mut self) -> io::Result<Option<f32>> {
        Ok(self.read_string()?.map(|s| s.parse().unwrap_or_default()))
    }

    /// Reads the next message as an int. This will return zero if the message is not a valid int.
    pub fn read_int(&mut self) -> io::Result<Option<i32>> {
        Ok(self.read_string()?.map(|s| s.parse().unwrap_or_default()))
    }
}

impl<T> ODrive<T> where T: Write {
    /// Move the motor to a position. Use this command if you have a real-time controller which
    /// is streaming setpoints and tracking a trajectory.
    /// `axis` The motor to be used for the operation.
    /// `position` is the desired position, in encoder counts.
    /// `velocity_feed_forward` is the velocity feed forward term, in encoder counts per second.
    /// `current_feed_forward` is the current feed forward term, in amps.
    /// If `None` is supplied for a feed forward input, zero will be provided as a default.
    pub fn set_position_p(&mut self, axis: Axis, position: f32, velocity_feed_forward: Option<f32>,
                          current_feed_forward: Option<f32>) -> io::Result<()> {
        let velocity_feed_forward = velocity_feed_forward.unwrap_or_default();
        let current_feed_forward = current_feed_forward.unwrap_or_default();
        writeln!(self, "p {} {} {} {}", axis as u8, position, velocity_feed_forward, current_feed_forward)?;
        self.flush()
    }

    /// Move the motor to a position. Use this command if you are sending one setpoint at a time.
    /// `axis` The motor to be used for the operation.
    /// `position` is the desired position, in encoder counts.
    /// `velocity_limit` is the velocity limit, in encoder counts per second.
    /// `current_limit` is the current limit, in amps.
    /// If `None` is supplied for a limit, zero will be provided as a default.
    pub fn set_position_q(&mut self, axis: Axis, position: f32, velocity_limit: Option<f32>,
                          current_limit: Option<f32>) -> io::Result<()> {
        let velocity_limit = velocity_limit.unwrap_or_default();
        let current_limit = current_limit.unwrap_or_default();
        writeln!(self, "q {} {} {} {}", axis as u8, position, velocity_limit, current_limit)?;
        self.flush()
    }

    /// Specifies a velocity setpoint for the motor.
    /// `axis` The motor to be used for the operation.
    /// `velocity` is the velocity setpoint, in encoder counts per second.
    /// `current_feed_forward` is the current feed forward term, in amps.
    /// If `None` is supplied for a feed forward input, zero will be provided as a default.
    pub fn set_velocity(&mut self, axis: Axis, velocity: f32, current_feed_forward: Option<f32>) -> io::Result<()> {
        let current_feed_forward = current_feed_forward.unwrap_or_default();
        writeln!(self, "v {} {} {}", axis as u8, velocity, current_feed_forward)?;
        self.flush()
    }

    /// Specifies a velocity setpoint for the motor.
    /// `axis` The motor to be used for the operation.
    /// `current` is the current to be supplied, in amps.
    pub fn set_current(&mut self, axis: Axis, current: f32) -> io::Result<()> {
        writeln!(self, "c {} {}", axis as u8, current)?;
        self.flush()
    }

    /// Moves a motor to a given position
    /// For general movement, this is the best command.
    /// `axis` The motor to be used for the operation.
    /// `position` is the desired position, in encoder counts.
    pub fn set_trajectory(&mut self, axis: Axis, position: f32) -> io::Result<()> {
        writeln!(self, "t {} {}", axis as u8, position)?;
        self.flush()
    }
}

impl<T> ODrive<T> where T: Read + Write {
    /// Retrieves the velocity of a motor, in counts per second.
    pub fn get_velocity(&mut self, axis: Axis) -> io::Result<Option<f32>> {
        writeln!(self, "r axis{} .encoder.vel_estimate", axis as u8)?;
        self.flush()?;
        self.read_float()
    }

    /// Changes the state of an axis.
    /// The `wait` flag indicates whether this command should block until the state is updated.
    pub fn run_state(&mut self, axis: Axis, requested_state: AxisState, wait: bool) -> io::Result<bool> {
        let mut timeout_ctr = 100;
        writeln!(self, "w axis{}.requested_state {}", axis as u8, requested_state as u8)?;
        self.flush()?;
        if wait {
            while {
                sleep(Duration::from_millis(100));
                writeln!(self, "r axis{}.current_state", axis as u8)?;
                self.flush()?;
                timeout_ctr -= 1;
                self.read_int()?.unwrap_or_default() != AxisState::Idle as i32 && timeout_ctr > 0 // exit
            } {}
        }

        Ok(timeout_ctr > 0)
    }
}

// Implement private helper methods
impl<T> ODrive<T> where T: Read + Write {
    fn set_config_variable<D: Display>(&mut self, param: &str, value: D) -> ODriveResult<()> {
        writeln!(self, "w {} {}", param, value).map_err(ODriveError::Io)?;
        self.flush().map_err(ODriveError::Io)
    }

    fn set_axis_config<D: Display>(&mut self, axis: Axis, property: &str, value: D) -> ODriveResult<()> {
        let config = format!("axis{}.{}", axis as u8, property);
        self.set_config_variable(&config, value)
    }

    fn set_config_bool(&mut self, axis: Axis, name: &str, value: bool) -> ODriveResult<()> {
        let config = format!("axis{}.config.{}", axis as u8, name);
        self.set_config_variable(&config, value as u8)
    }
}

/// # Startup Configuration
/// The ODrive motor controllers have several optional startup procedures which can be enabled.
/// Each of them has an associated getter and setter which can be invoked to read to and write from
/// their value.
///
/// From the official documentation:
/// > By default the ODrive takes no action at startup and goes to idle immediately.
/// > In order to change what startup procedures are used, set the startup procedures you want to `true`.
/// > The ODrive will sequence all enabled startup actions selected in the order shown below.
///
/// > 1. `<axis>.config.startup_motor_calibration`
/// > 1. `<axis>.config.startup_encoder_index_search`
/// > 1. `<axis>.config.startup_encoder_offset_calibration`
/// > 1. `<axis>.config.startup_closed_loop_control`
/// > 1. `<axis>.config.startup_sensorless_control`
///
impl<T> ODrive<T> where T: Read + Write {
    /// This function sets the motor calibration to run
    pub fn set_startup_motor_calibration(&mut self, axis: Axis, value: bool) -> ODriveResult<()> {
        self.set_config_bool(axis, "startup_motor_calibration", value)
    }

    pub fn set_startup_encoder_index_search(&mut self, axis: Axis, value: bool) -> ODriveResult<()> {
        self.set_config_bool(axis, "startup_encoder_index_search", value)
    }

    pub fn set_startup_encoder_offset_calibration(&mut self, axis: Axis, value: bool) -> ODriveResult<()> {
        self.set_config_bool(axis, "startup_encoder_offset_calibration", value)
    }

    pub fn set_startup_closed_loop_control(&mut self, axis: Axis, value: bool) -> ODriveResult<()> {
        self.set_config_bool(axis, "startup_closed_loop_control", value)
    }

    pub fn set_startup_sensorless_control(&mut self, axis: Axis, value: bool) -> ODriveResult<()> {
        self.set_config_bool(axis, "startup_sensorless_control", value)
    }
}

/// Configuration management.
impl<T> ODrive<T> where T: Read + Write {
    pub fn save_configuration(&mut self) -> ODriveResult<()> {
        writeln!(self, "ss").map_err(ODriveError::Io)?;
        self.flush().map_err(ODriveError::Io)
    }

    pub fn erase_configuration(&mut self) -> ODriveResult<()> {
        writeln!(self, "se").map_err(ODriveError::Io)?;
        self.flush().map_err(ODriveError::Io)
    }
}

/// Motor configuration
impl<T> ODrive<T> where T: Read + Write {
    pub fn set_motor_pole_pairs(&mut self, axis: Axis, value: u16) -> ODriveResult<()> {
        self.set_axis_config(axis, "motor.config.pole_pairs", value)
    }

    pub fn set_motor_resistance_calib_max_voltage(&mut self, axis: Axis, value: f32) -> ODriveResult<()> {
        self.set_axis_config(axis, "motor.config.resistance_calib_max_voltage", value)
    }

    pub fn set_motor_requested_current_range(&mut self, axis: Axis, value: f32) -> ODriveResult<()> {
        self.set_axis_config(axis, "motor.config.requested_current_range", value)
    }

    pub fn set_motor_current_control_bandwidth(&mut self, axis: Axis, value: f32) -> ODriveResult<()> {
        self.set_axis_config(axis, "motor.config.current_control_bandwidth", value)
    }
}

/// Encoder configuration
impl<T> ODrive<T> where T: Read + Write {
    pub fn set_encoder_mode(&mut self, axis: Axis, value: EncoderMode) -> ODriveResult<()> {
        self.set_axis_config(axis, "encoder.config.mode", value as u8)
    }

    pub fn set_encoder_cpr(&mut self, axis: Axis, value: u16) -> ODriveResult<()> {
        self.set_axis_config(axis, "encoder.config.cpr", value)
    }

    pub fn set_encoder_bandwidth(&mut self, axis: Axis, value: f32) -> ODriveResult<()> {
        self.set_axis_config(axis, "encoder.config.bandwidth", value)
    }

    pub fn set_encoder_pre_calibrated(&mut self, axis: Axis, value: bool) -> ODriveResult<()> {
        self.set_axis_config(axis, "encoder.config.pre_calibrated", value as u8)
    }
}

/// Controller configuration
impl<T> ODrive<T> where T: Read + Write {
    pub fn set_position_gain(&mut self, axis: Axis, value: f32) -> ODriveResult<()> {
        self.set_axis_config(axis, "controller.config.pos_gain", value)
    }

    pub fn set_velocity_gain(&mut self, axis: Axis, value: f32) -> ODriveResult<()> {
        self.set_axis_config(axis, "controller.config.vel_gain", value)
    }

    pub fn set_velocity_integrator_gain(&mut self, axis: Axis, value: f32) -> ODriveResult<()> {
        self.set_axis_config(axis, "encoder.config.bandwidth", value)
    }

    pub fn set_velocity_limit(&mut self, axis: Axis, value: f32) -> ODriveResult<()> {
        self.set_axis_config(axis, "encoder.config.bandwidth", value)
    }

    pub fn set_control_mode(&mut self, axis: Axis, mode: ControlMode) -> ODriveResult<()> {
        self.set_axis_config(axis, "controller.config.control_mode", mode as u8)
    }
}