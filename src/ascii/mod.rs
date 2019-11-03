use std::io::{BufRead, BufReader, BufWriter, Error, Read, Write};
use std::io;
use std::thread::sleep;
use std::time::Duration;

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
    writer: BufWriter<T>,
    reader: BufReader<T>,
}

impl<T> ODrive<ReadWriteCloningDecorator<T>> where T: Read + Write {
    /// Constructs a new ODrive connection using the `ReadWriteCloningDecorator`.
    /// This method of construction will have consequences in overhead.
    /// It should only be used when it is not possible to clone the type `T`.
    pub fn new(serial: T) -> Self {
        let serial = ReadWriteCloningDecorator::new(serial);
        Self::from_cloneable(serial)
    }
}

impl<T> ODrive<T> where T: Read + Write + Clone {
    pub fn from_cloneable(serial: T) -> Self {
        let reader = BufReader::new(serial.clone());
        let writer = BufWriter::new(serial);
        Self { writer, reader }
    }
}

impl<T> Write for ODrive<T> where T: Write + Read {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.writer.flush()
    }
}

impl<T> Read for ODrive<T> where T: Read + Write {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        self.reader.read(buf)
    }
}

impl<T> BufRead for ODrive<T> where T: Read + Write {
    fn fill_buf(&mut self) -> Result<&[u8], Error> {
        self.reader.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.reader.consume(amt)
    }
}

impl<T> ODrive<T> where T: Read + Write {
    #[cfg(test)]
    fn from_pair(reader: BufReader<T>, writer: BufWriter<T>) -> Self {
        Self { writer, reader }
    }

    pub fn set_position(&mut self, motor_number: u8, position: f32, velocity_feed_forward: Option<f32>, current_feed_forward: Option<f32>) -> io::Result<()> {
        assert!(motor_number < 2);
        let velocity_feed_forward = velocity_feed_forward.unwrap_or_default();
        let current_feed_forward = current_feed_forward.unwrap_or_default();
        writeln!(self.writer, "p {} {} {} {}", motor_number, position, velocity_feed_forward, current_feed_forward)?;
        self.flush()
    }

    pub fn set_velocity(&mut self, motor_number: u8, position: f32, current_feed_forward: Option<f32>) -> io::Result<()> {
        assert!(motor_number < 2);
        let current_feed_forward = current_feed_forward.unwrap_or_default();
        writeln!(self.writer, "v {} {} {}", motor_number, position, current_feed_forward)?;
        self.flush()
    }

    pub fn set_current(&mut self, motor_number: u8, current: f32) -> io::Result<()> {
        assert!(motor_number < 2);
        writeln!(self.writer, "c {} {}", motor_number, current)?;
        self.flush()
    }

    pub fn trapezoidal_move(&mut self, motor_number: u8, position: f32) -> io::Result<()> {
        assert!(motor_number < 2);
        writeln!(self.writer, "t {} {}", motor_number, position)?;
        self.flush()
    }

    pub fn get_velocity(&mut self, motor_number: u8) -> io::Result<f32> {
        assert!(motor_number < 2);
        writeln!(self.writer, "r axis{} .encoder.vel_estimate", motor_number)?;
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
        writeln!(self.writer, "w axis{}.requested_state", requested_state as u8)?;
        self.flush()?;
        if wait {
            while {
                sleep(Duration::from_millis(100));
                writeln!(self.writer, "r axis{}.current_state", axis)?;
                self.flush()?;
                timeout_ctr -= 1;
                self.read_int()? != AxisState::Idle as i32 && timeout_ctr > 0
            } {}
        }

        Ok(timeout_ctr > 0)
    }

    fn read_string(&mut self) -> io::Result<String> {
        let mut buffer = String::with_capacity(40);
        self.reader.read_line(&mut buffer)?;
        Ok(buffer.trim().to_owned())
    }
}