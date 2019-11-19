use std::io::{Read, Write};

use crate::commands::ODrive;
use crate::enumerations::{AxisID, AxisState, ControlMode};
use crate::enumerations::errors::ODriveResult;

trait ToODrivePaths {
    fn to_required_partial_paths(&self) -> Vec<String>;
    fn to_all_partial_paths(&self) -> Vec<String>;
}

pub struct WriteableEncoderConfiguration {
    config_use_index: Option<bool>
}

pub struct EncoderStateQuery {
    pub config_use_index: bool,
    pub is_ready: bool,
    pub pos_estimate: f64,
    pub vel_estimate: f64,
}

pub struct WriteableMotorConfiguration {
    config_pre_calibrated: Option<bool>,
    config_direction: Option<bool>,
}

pub struct MotorStateQuery {
    pub is_calibrated: bool,
    pub config_phase_resistance: f64,
    pub config_phase_inductance: f64,
    pub current_control_iq_setpoint: f64,
    pub current_control_iq_measured: f64,
}

pub struct WriteableControllerConfiguration {
    config_vel_gain: Option<f64>,
    config_vel_integrator_gain: Option<f64>,
    config_control_mode: Option<ControlMode>,
    vel_setpoint: Option<f64>,
}

pub struct ControllerStateQuery {
    pub config_vel_gain: f64,
    pub config_vel_integrator_gain: f64,
    pub config_control_mode: ControlMode,
    pub vel_setpoint: f64,
}