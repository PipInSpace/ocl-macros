//! Tests for Buffer macros

use super::init_queue;
use crate::*;

#[test]
fn macro_buffer() {
    let queue = init_queue();
    let _buffer = buffer!(&queue, 100, 0.0f32);
}

#[test]
fn macro_buffer_flags() {
    let queue = init_queue();
    let _buffer = buffer_flags!(&queue, 100, 0.0f32, ocl::flags::MEM_READ_WRITE);
}
