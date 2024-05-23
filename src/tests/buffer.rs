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

#[test]
fn macro_bread() {
    let queue = init_queue();
    let buffer = buffer!(&queue, 100, 0.0f32);
    let mut vec: Vec<f32> = vec![1.0; 100];
    bread!(buffer, vec);
    assert!(vec[0] == 0.0)
}

#[test]
fn macro_bwrite() {
    let queue = init_queue();
    let buffer = buffer!(&queue, 100, 0.0f32);
    let vec: Vec<f32> = vec![1.0; 100];
    bwrite!(buffer, vec);
}

#[test]
fn macro_bget() {
    let l = 100;
    let queue = init_queue();
    let buffer = buffer!(&queue, l, 10.0f32);
    let vec = bget!(buffer);
    assert!(vec[0] == 10.0);
    assert!(vec.len() == l);
}
