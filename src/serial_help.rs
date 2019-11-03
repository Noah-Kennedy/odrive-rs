use std::cell::RefCell;
use std::rc::Rc;
use std::ops::Deref;
use std::io::{Read, Error, Write};
use std::borrow::BorrowMut;

#[derive(Clone)]
pub struct CloneableSerial<T> {
    inner: Rc<RefCell<T>>
}

impl <T> Read for CloneableSerial<T> where T: Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        self.inner.deref().borrow_mut().read(buf)
    }
}

impl <T> Write for CloneableSerial<T> where T: Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.inner.deref().borrow_mut().write(buf)
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.inner.deref().borrow_mut().flush()
    }
}