use crate::*;
use super::init_queue;

// 
// Tests for buffer macros
// 

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