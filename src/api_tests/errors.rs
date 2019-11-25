use std::clone::Clone;
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::fmt::Debug;
use std::marker::Copy;

use crate::prelude::*;

assert_type_eq_all!(ODriveResult<u32>, Result<u32, ODriveError>);
assert_eq_size!(ControllerError, u8);
assert_eq_size!(EncoderError, u8);
assert_eq_size!(MotorError, u16);
assert_eq_size!(AxisError, u16);

assert_impl_all!(ODriveError: Debug);
assert_impl_all!(AxisError: Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone);
assert_impl_all!(MotorError: Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone);
assert_impl_all!(EncoderError: Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone);
assert_impl_all!(ControllerError: Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone);
