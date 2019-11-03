use std::cell::RefCell;
use std::io::{Error, Read, Write};
use std::ops::Deref;
use std::rc::Rc;

pub struct CloneableSerial<T> {
    inner: Rc<RefCell<T>>
}

impl<T> Clone for CloneableSerial<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone()
        }
    }
}

impl<T> CloneableSerial<T> {
    pub fn new(inner: T) -> Self {
        let inner = Rc::new(RefCell::new(inner));
        Self { inner }
    }
}

impl<T> Read for CloneableSerial<T> where T: Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        self.inner.deref().borrow_mut().read(buf)
    }
}

impl<T> Write for CloneableSerial<T> where T: Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.inner.deref().borrow_mut().write(buf)
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.inner.deref().borrow_mut().flush()
    }
}