use std::io::{BufRead, BufReader, BufWriter, Error, Read, Write};
use std::io;
use std::thread::sleep;
use std::time::{Duration, Instant};

use crate::cloning_help::ReadWriteCloningDecorator;

#[cfg(test)]
mod tests;

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

/// Represents a connection with an ODrive motor controller.
pub struct ODrive<T> where T: Read + Write {
    remote: T
}

impl<T> ODrive<T> where T: Read + Write {
    /// Constructs a new ODrive connection using the `ReadWriteCloningDecorator`.
    /// This method of construction will have consequences in overhead.
    /// It should only be used when it is not possible to clone the type `T`.
    pub fn new(serial: T) -> Self {
        Self { remote: serial }
    }
}

impl<T> Write for ODrive<T> where T: Write + Read {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.remote.write(buf)
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.remote.flush()
    }
}

impl<T> Read for ODrive<T> where T: Read + Write {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        self.remote.read(buf)
    }
}

impl<T> ODrive<T> where T: Read + Write {
    pub fn set_position(&mut self, motor_number: u8, position: f32, velocity_feed_forward: Option<f32>, current_feed_forward: Option<f32>) -> io::Result<()> {
        assert!(motor_number < 2);
        let velocity_feed_forward = velocity_feed_forward.unwrap_or_default();
        let current_feed_forward = current_feed_forward.unwrap_or_default();
        writeln!(self.remote, "p {} {} {} {}", motor_number, position, velocity_feed_forward, current_feed_forward)?;
        self.flush()
    }

    pub fn set_velocity(&mut self, motor_number: u8, position: f32, current_feed_forward: Option<f32>) -> io::Result<()> {
        assert!(motor_number < 2);
        let current_feed_forward = current_feed_forward.unwrap_or_default();
        writeln!(self.remote, "v {} {} {}", motor_number, position, current_feed_forward)?;
        self.flush()
    }

    pub fn set_current(&mut self, motor_number: u8, current: f32) -> io::Result<()> {
        assert!(motor_number < 2);
        writeln!(self.remote, "c {} {}", motor_number, current)?;
        self.flush()
    }

    pub fn trapezoidal_move(&mut self, motor_number: u8, position: f32) -> io::Result<()> {
        assert!(motor_number < 2);
        writeln!(self.remote, "t {} {}", motor_number, position)?;
        self.flush()
    }

    pub fn get_velocity(&mut self, motor_number: u8) -> io::Result<f32> {
        assert!(motor_number < 2);
        writeln!(self.remote, "r axis{} .encoder.vel_estimate", motor_number)?;
        self.flush()?;
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
        writeln!(self.remote, "w axis{}.requested_state {}", axis, requested_state as u8)?;
        self.flush()?;
        if wait {
            while {
                sleep(Duration::from_millis(100));
                writeln!(self.remote, "r axis{}.current_state", axis)?;
                self.flush()?;
                timeout_ctr -= 1;
                self.read_int()? != AxisState::Idle as i32 && timeout_ctr > 0
            } {}
        }

        Ok(timeout_ctr > 0)
    }

    fn read_string(&mut self) -> io::Result<String> {
        let mut string = String::with_capacity(20);
        let duration = Instant::now();
        loop {
            let mut buffer = [0; 1];
            while self.remote.read(&mut buffer).unwrap_or_default() == 0 {
                if duration.elapsed().as_millis() >= 1_000 {
                    return Ok(string);
                }
            }
            let ch = buffer[0];
            if ch as char == '\n' {
                break;
            }

            string.push(ch as char);
        }

        println!("{}", string);
        Ok(string)
    }
}