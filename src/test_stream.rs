use std::io::{Error, Read, Write};

#[derive(Eq, PartialEq, Ord, PartialOrd, Default, Debug, Clone)]
pub struct MockStream {
    pub read_buffer: Vec<u8>,
    pub write_buffer: Vec<u8>,
    pub flushed: bool,
}

impl MockStream {
    pub fn new() -> Self {
        Self {
            read_buffer: Vec::new(),
            write_buffer: Vec::new(),
            flushed: false,
        }
    }
}

impl Write for MockStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        for e in buf {
            self.write_buffer.push(*e);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.flushed = true;
        Ok(())
    }
}

impl Read for MockStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let mut count = 0;
        while count < buf.len() {
            if let Some(res) = self.read_buffer.pop() {
                buf[count] = res;
            } else {
                break;
            }

            count += 1;
        }

        Ok(count)
    }
}