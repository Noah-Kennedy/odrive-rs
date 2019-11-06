//! This file was derived from [the python library](https://github.com/madcowswe/ODrive/blob/master/tools/odrive/enums.py)

/// Contains error enums that can be sent from the ODrive
pub mod errors;

/// Used to indicate one of the two motors controlled by the ODrive.
#[repr(u8)]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum Axis {
    Zero = 0,
    One = 1,
}

/// # State Machine
/// From the official docs:
/// > The current state of an axis is indicated by `<axis>.current_state`.
/// > The user can request a new state by assigning a new value to `<axis>.requested_state`.
/// > The default state after startup is `AXIS_STATE_IDLE`.
/// >
/// > 1. `AXIS_STATE_IDLE` Disable motor PWM and do nothing.
/// > 2. `AXIS_STATE_STARTUP_SEQUENCE` Run the startup procedure.
/// > 3. `AXIS_STATE_FULL_CALIBRATION_SEQUENCE` Run motor calibration and then encoder offset calibration (or encoder index search if `<axis>.encoder.config.use_index` is `true`).
/// > 4. `AXIS_STATE_MOTOR_CALIBRATION` Measure phase resistance and phase inductance of the motor.
/// >       * To store the results set <`axis>.motor.config.pre_calibrated` to `True` and save the configuration.
/// >       * After that you don’t have to run the motor calibration on the next start up.
/// >       * This modifies the variables `<axis>.motor.config.phase_resistance` and `<axis>.motor.config.phase_inductance`.
/// > 5. `AXIS_STATE_SENSORLESS_CONTROL` Run sensorless control.
/// >       * The motor must be calibrated (`<axis>.motor.is_calibrated`)
/// >       * <axis>.controller.control_mode must be `true`.
/// > 6. `AXIS_STATE_ENCODER_INDEX_SEARCH` Turn the motor in one direction until the encoder index is traversed.
/// >           This state can only be entered if `<axis>.encoder.config.use_index` is `true`.
/// > 7. `AXIS_STATE_ENCODER_OFFSET_CALIBRATION` Turn the motor in one direction for a few seconds and then back to measure the offset between the encoder position and the electrical phase.
/// >       * Can only be entered if the motor is calibrated (`<axis>.motor.is_calibrated`).
/// >       * A successful encoder calibration will make the `<axis>.encoder.is_ready` go to `true`.
/// > 8. `AXIS_STATE_CLOSED_LOOP_CONTROL` Run closed loop control.
/// >       * The action depends on the control mode.
/// >       * Can only be entered if the motor is calibrated (`<axis>.motor.is_calibrated`) and the encoder is ready (`<axis>.encoder.is_ready`).
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