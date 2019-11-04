use std::io::{BufRead, BufReader, BufWriter, Error, Read, Write};
use std::io;
use std::thread::sleep;
use std::time::{Duration, Instant};
use crate::enumerations::AxisState;

#[cfg(test)]
mod tests;

/// Represents a connection with an ODrive motor controller.
pub struct ODrive<T> {
    io_stream: T
}

impl<T> ODrive<T> {
    pub fn new(io_stream: T) -> Self {
        Self { io_stream }
    }
}

impl<T> Write for ODrive<T> where T: Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.io_stream.write(buf)
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.io_stream.flush()
    }
}

impl<T> Read for ODrive<T> where T: Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        self.io_stream.read(buf)
    }
}

impl<T> ODrive<T> where T: Read {
    pub fn read_string(&mut self) -> io::Result<String> {
        let mut string = String::with_capacity(20);
        let duration = Instant::now();
        loop {
            let mut buffer = [0; 1];
            while self.io_stream.read(&mut buffer).unwrap_or_default() == 0 {
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

        Ok(string.trim().to_owned())
    }

    pub fn read_float(&mut self) -> io::Result<f32> {
        Ok(self.read_string()?.parse().unwrap_or_default())
    }

    pub fn read_int(&mut self) -> io::Result<i32> {
        Ok(self.read_string()?.parse().unwrap_or_default())
    }
}

impl<T> ODrive<T> where T: Write {
    pub fn set_position(&mut self, motor_number: u8, position: f32, velocity_feed_forward: Option<f32>, current_feed_forward: Option<f32>) -> io::Result<()> {
        assert!(motor_number < 2);
        let velocity_feed_forward = velocity_feed_forward.unwrap_or_default();
        let current_feed_forward = current_feed_forward.unwrap_or_default();
        writeln!(self.io_stream, "p {} {} {} {}", motor_number, position, velocity_feed_forward, current_feed_forward)?;
        self.flush()
    }

    pub fn set_velocity(&mut self, motor_number: u8, position: f32, current_feed_forward: Option<f32>) -> io::Result<()> {
        assert!(motor_number < 2);
        let current_feed_forward = current_feed_forward.unwrap_or_default();
        writeln!(self.io_stream, "v {} {} {}", motor_number, position, current_feed_forward)?;
        self.flush()
    }

    pub fn set_current(&mut self, motor_number: u8, current: f32) -> io::Result<()> {
        assert!(motor_number < 2);
        writeln!(self.io_stream, "c {} {}", motor_number, current)?;
        self.flush()
    }

    pub fn trapezoidal_move(&mut self, motor_number: u8, position: f32) -> io::Result<()> {
        assert!(motor_number < 2);
        writeln!(self.io_stream, "t {} {}", motor_number, position)?;
        self.flush()
    }
}

impl<T> ODrive<T> where T: Read + Write {
    pub fn get_velocity(&mut self, motor_number: u8) -> io::Result<f32> {
        assert!(motor_number < 2);
        writeln!(self.io_stream, "r axis{} .encoder.vel_estimate", motor_number)?;
        self.flush()?;
        self.read_float()
    }

    pub fn run_state(&mut self, axis: u8, requested_state: AxisState, wait: bool) -> io::Result<bool> {
        let mut timeout_ctr = 100;
        writeln!(self.io_stream, "w axis{}.requested_state {}", axis, requested_state as u8)?;
        self.flush()?;
        if wait {
            while {
                sleep(Duration::from_millis(100));
                writeln!(self.io_stream, "r axis{}.current_state", axis)?;
                self.flush()?;
                timeout_ctr -= 1;
                self.read_int().unwrap_or_default() != AxisState::Idle as i32 && timeout_ctr > 0
            } {}
        }

        Ok(timeout_ctr > 0)
    }
}