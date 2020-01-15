use serialport::SerialPort;
use crate::enumerations::MotorAxis;
use std::io;

pub mod sync;

pub struct AsciiODrive<T> {
    inner: T
}

impl <T> AsciiODrive<T> {
    pub fn new(serial: T) -> Self {
        Self {
            inner: serial
        }
    }
}

impl <T> AsciiODrive<T> where T: SerialPort {
    pub fn trajectory(&mut self, motor: MotorAxis, destination: i32) -> io::Result<()> {
        writeln!(self.inner, "t {} {}", motor as u8, destination)
    }
}