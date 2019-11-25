use std::fmt::Debug;
use std::io::{Read, Write};

use serialport::posix::TTYPort;

use crate::prelude::*;

assert_impl_all!(ODrive<TTYPort>: Debug, Read, Write);
