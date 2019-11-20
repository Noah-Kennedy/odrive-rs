use std::io::{Read, Write};

use crate::commands::ODrive;
use crate::enumerations::{AxisID, AxisState, ControlMode, EncoderMode};
use crate::enumerations::errors::ODriveResult;
use std::fmt::Display;
use std::marker::PhantomData;

trait ToRelativePaths {
    fn to_relative_paths(&self) -> Vec<String>;
}

pub struct EncoderConfigurationUpdate {
    config_use_index: Option<bool>,
    config_pre_calibrated: Option<bool>,
    config_mode: Option<EncoderMode>,
    config_cpr: Option<u32>,
    config_bandwidth: Option<f64>,
}

pub struct MotorConfigurationUpdate {
    config_pre_calibrated: Option<bool>,
    config_direction: Option<bool>,
    config_pole_pairs: Option<u32>,
    config_resistance_calib_max_voltage: Option<f64>,
    config_requested_current_range: Option<f64>,
    config_current_control_bandwidth: Option<f64>
}

pub struct ControllerConfigurationUpdate {
    config_pos_gain: Option<f64>,
    config_vel_gain: Option<f64>,
    config_vel_limit: Option<f64>,
    config_vel_integrator_gain: Option<f64>,
    config_control_mode: Option<ControlMode>,
    vel_setpoint: Option<f64>,
}