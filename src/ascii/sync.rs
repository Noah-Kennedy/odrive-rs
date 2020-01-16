use serialport::{SerialPort, SerialPortSettings};
use crate::enumerations::Axis;
use std::io;
use std::io::{Write, Read, BufReader, BufRead, ErrorKind};
use serialport::posix::TTYPort;
use std::path::Path;

pub struct AsciiODrive<T> {
    inner: BufReader<T>
}

impl<T> AsciiODrive<T> where T: SerialPort {
    pub fn new(serial: T) -> Self {
        Self {
            inner: BufReader::new(serial)
        }
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }

    pub fn get_ref(&self) -> &T {
        self.inner.get_ref()
    }
}

impl AsciiODrive<TTYPort> {
    pub fn open_posix(path: &Path, settings: &SerialPortSettings) -> serialport::Result<Self> {
        let port = TTYPort::open(path, settings)?;
        Ok(Self::new(port))
    }
}

impl<T> AsciiODrive<T> where T: Write {
    pub fn trajectory(&mut self, motor: Axis, destination: i32) -> io::Result<()> {
        writeln!(self.inner.get_mut(), "t {} {}", motor as u8, destination)
    }

    pub fn velocity(&mut self, motor: Axis, velocity: f64, current_ff: Option<f64>) -> io::Result<()> {
        match current_ff {
            None => writeln!(self.inner.get_mut(), "v {} {}", motor as u8, velocity),
            Some(current) => writeln!(self.inner.get_mut(), "v {} {} {}", motor as u8, velocity, current),
        }
    }

    pub fn current(&mut self, motor: Axis, current: f64) -> io::Result<()> {
        writeln!(self.inner.get_mut(), "c {} {}", motor as u8, current)
    }

    pub fn position(&mut self, motor: Axis,
                    position: i32, velocity_lim: Option<f64>, current_lim: Option<f64>)
                    -> io::Result<()> {
        let vel = match velocity_lim {
            None => "".to_owned(),
            Some(val) => val.to_string(),
        };

        let cur = match current_lim {
            None => "".to_owned(),
            Some(val) => val.to_string(),
        };

        writeln!(self.inner.get_mut(), "q {} {} {} {}", motor as u8, position, vel, cur)
    }

    pub fn position_streaming(&mut self, motor: Axis,
                              position: i32, velocity_ff: Option<f64>, current_ff: Option<f64>)
                              -> io::Result<()> {
        let vel = match velocity_ff {
            None => "".to_owned(),
            Some(val) => val.to_string(),
        };

        let cur = match current_ff {
            None => "".to_owned(),
            Some(val) => val.to_string(),
        };

        writeln!(self.inner.get_mut(), "p {} {} {} {}", motor as u8, position, vel, cur)
    }

    pub fn update_watchdog(&mut self, motor: Axis) -> io::Result<()> {
        writeln!(self.inner.get_mut(), "u {}", motor as u8)
    }

    pub fn save_config(&mut self) -> io::Result<()> {
        writeln!(self.inner.get_mut(), "ss")
    }

    pub fn erase_config(&mut self) -> io::Result<()> {
        writeln!(self.inner.get_mut(), "se")
    }

    pub fn reboot(&mut self) -> io::Result<()> {
        writeln!(self.inner.get_mut(), "sr")
    }
}

impl<T> AsciiODrive<T> where T: Write + Read {
    pub fn feedback(&mut self, motor: Axis) -> io::Result<(f64, f64)> {
        writeln!(self.inner.get_mut(), "f {}", motor as u8)?;

        let mut buf = String::with_capacity(20);
        self.inner.read_line(&mut buf)?;

        let mut split = buf.trim().split(' ');

        let pos_str = split.next();
        let vel_str = split.next();

        let pos_str = match pos_str {
            None => Err(io::Error::from(ErrorKind::InvalidData)),
            Some(val) => Ok(val),
        }?;

        let vel_str = match vel_str {
            None => Err(io::Error::from(ErrorKind::InvalidData)),
            Some(val) => Ok(val),
        }?;

        let pos = match pos_str.parse() {
            Ok(res) => Ok(res),
            Err(_) => Err(io::Error::from(ErrorKind::InvalidData)),
        }?;

        let vel = match vel_str.parse() {
            Ok(res) => Ok(res),
            Err(_) => Err(io::Error::from(ErrorKind::InvalidData)),
        }?;

        Ok((pos, vel))
    }

    pub fn read_endpoint(&mut self, endpoint: &str) -> io::Result<String> {
        writeln!(self.inner.get_mut(), "r {}", endpoint)?;

        let mut buf = String::with_capacity(20);
        self.inner.read_line(&mut buf)?;

        Ok(buf)
    }

    pub fn write_endpoint(&mut self, endpoint: &str, value: &str) -> io::Result<Option<String>> {
        writeln!(self.inner.get_mut(), "w {} {}", endpoint, value)?;

        let mut buf = String::with_capacity(20);

        if let Err(error) = self.inner.read_line(&mut buf) {
            match error.kind() {
                ErrorKind::WouldBlock => Ok(None),
                _ => Err(error)
            }
        } else {
            Ok(Some(buf))
        }
    }
}