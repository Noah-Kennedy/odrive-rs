use std::clone::Clone;
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::fmt::Debug;
use std::marker::Copy;

use crate::prelude::*;

assert_eq_size!(AxisID, u8);
assert_eq_size!(AxisState, u8);
assert_eq_size!(MotorType, u8);
assert_eq_size!(ControlMode, u8);
assert_eq_size!(EncoderMode, u8);

assert_impl_all!(AxisID: Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone);
assert_impl_all!(AxisState: Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone);
assert_impl_all!(MotorType: Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone);
assert_impl_all!(ControlMode: Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone);
assert_impl_all!(EncoderMode: Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone);
