use serialport::SerialPort;

#[repr(C)]
pub enum AxisState {
    Undefined = 0,
    Idle = 1,
    StartupSequence = 2,
    FullCalibrationSequence = 3,
    MotorCalibration = 4,
    SensorlessControl = 5,
    EncoderIndexSearch = 6,
    EncoderOffsetCalibration = 7,
    ClosedLoopControl = 8
}

pub struct ODrive<T> {
    serial: T,
    pub state: AxisState
}

impl <T> ODrive<T> where T: SerialPort {
    pub fn set_position(&mut self, motor_number: u8, position: f32, velocity_feed_forward: Option<f32>, current_feed_forward: Option<f32>) {
        unimplemented!()
    }

    pub fn set_velocity(&mut self, motor_number: u8, position: f32, current_feed_forward: Option<f32>) {
        unimplemented!()
    }

    pub fn set_current(&mut self, motor_number: u8, current: f32) {
        unimplemented!()
    }

    pub fn get_velocity(&self, motor_number: u8) -> f32 {
        unimplemented!()
    }

    pub fn read_float(&mut self) -> f32 {
        unimplemented!()
    }

    pub fn read_int(&mut self) -> i32 {
        unimplemented!()
    }

    pub fn run_state(&mut self, axis: u8, requested_state: AxisState, wait: bool) -> bool {
        unimplemented!()
    }

    fn read_string(&mut self) -> String {
        unimplemented!()
    }
}