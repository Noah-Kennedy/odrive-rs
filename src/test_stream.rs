use std::io::{Write, Error, Read};

#[derive(Eq, PartialEq, Ord, PartialOrd, Default, Debug, Clone)]
pub struct MockStream {
    pub buffer: Vec<u8>,
    pub flushed: bool
}

impl MockStream {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            flushed: false
        }
    }
}

impl Write for MockStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        for e in buf {
            self.buffer.push(*e);
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
        self.buffer = buf.to_vec();
        Ok(buf.len())
    }
}