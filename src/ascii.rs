use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::io;

use serialport::SerialPort;
use std::thread::sleep;
use std::time::Duration;

#[repr(C)]
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

pub struct ODrive<T> where T: Read + Write {
    writer: BufWriter<T>,
    reader: BufReader<T>,
    pub state: AxisState,
}

impl<T> ODrive<T> where T: SerialPort + Clone {
    pub fn new(serial: T) -> Self {
        let reader = BufReader::new(serial.clone());
        let writer = BufWriter::new(serial);
        let state = AxisState::Undefined;
        Self { writer, reader, state }
    }

    pub fn set_position(&mut self, motor_number: u8, position: f32, velocity_feed_forward: Option<f32>, current_feed_forward: Option<f32>) -> io::Result<()> {
        let velocity_feed_forward = velocity_feed_forward.unwrap_or_default();
        let current_feed_forward = current_feed_forward.unwrap_or_default();
        writeln!(self.writer, "p {} {} {} {}", motor_number, position, velocity_feed_forward, current_feed_forward)
    }

    pub fn set_velocity(&mut self, motor_number: u8, position: f32, current_feed_forward: Option<f32>) -> io::Result<()> {
        let current_feed_forward = current_feed_forward.unwrap_or_default();
        writeln!(self.writer, "v {} {} {}", motor_number, position, current_feed_forward)
    }

    pub fn set_current(&mut self, motor_number: u8, current: f32) -> io::Result<()> {
        writeln!(self.writer, "c {} {}", motor_number, current)
    }

    pub fn trapezoidal_move(&mut self, motor_number: u8, position: f32) -> io::Result<()> {
        writeln!(self.writer, "t {} {}", motor_number, position)
    }

    pub fn get_velocity(&mut self, motor_number: u8) -> io::Result<f32> {
        writeln!(self.writer, "r axis{} .encoder.vel_estimate", motor_number)?;
        self.read_float()
    }

    pub fn read_float(&mut self) -> io::Result<f32> {
        Ok(self.read_string()?.parse().unwrap_or_default())
    }

    pub fn read_int(&mut self) -> io::Result<i32> {
        Ok(self.read_string()?.parse().unwrap_or_default())
    }

    pub fn run_state(&mut self, axis: u8, requested_state: AxisState, wait: bool) -> io::Result<bool> {
        let mut timeout_ctr = 100;
        writeln!(self.writer, "w axis{}.requested_state", requested_state as u8)?;
        if wait {
            while {
                sleep(Duration::from_millis(100));
                writeln!(self.writer, "r axis{}.current_state", axis)?;
                timeout_ctr -= 1;
                self.read_int()? != AxisState::Idle as i32 && timeout_ctr > 0
            } {}
        }

        Ok(timeout_ctr > 0)
    }

    fn read_string(&mut self) -> io::Result<String> {
        let mut buffer = String::with_capacity(40);
        self.reader.read_line(&mut buffer)?;
        Ok(buffer)
    }
}