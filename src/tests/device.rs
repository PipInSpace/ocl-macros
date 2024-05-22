//! Tests for Device macros

use crate::*;

#[test]
fn macro_device_vec() {
    let _device_vec = device_vec!();
}

#[test]
fn macro_default_device() {
    let _default_device = default_device!();
}
