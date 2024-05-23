//! Tests for Kernel macros

use super::{init_program_queue, PROGRAM_SRC};
use crate::*;

const WORK_SIZE: i32 = 1;

#[test]
fn macro_kernel() {
    let (program, queue) = init_program_queue(PROGRAM_SRC);
    // Create kernel with unnamed arguments
    let kernel = kernel!(program, queue, "build", WORK_SIZE, 0.0f32);
    // Create kernel with named arguments
    let kernel_n = kernel!(program, queue, "build", WORK_SIZE, ("name", 0.0f32));

    assert!(kernel.set_arg("name", 10.0f32).is_err()); // This should fail due to lack of named arguments
    kernel_n.set_arg("name", 10.0f32).unwrap();
    
}

#[test]
fn macro_kernel_builder() {
    let (program, queue) = init_program_queue(PROGRAM_SRC);
    let mut kernel_builder = kernel_builder!(program, queue, "build", WORK_SIZE);
    let _kernel = kernel_builder.arg(0.0f32).build().unwrap();
}

#[test]
fn macro_kernel_args() {
    let (program, queue) = init_program_queue(PROGRAM_SRC);
    // Unnamed argument
    let mut kernel_builder = ocl::Kernel::builder();
    kernel_builder
        .program(&program)
        .queue(queue.clone())
        .name("build")
        .global_work_size(WORK_SIZE);
    // Add unnamed argument to kernel
    kernel_args!(kernel_builder, 0.0f32); 
    let _kernel = kernel_builder.build().unwrap();

    // Named argument
    let mut kernel_builder_n = ocl::Kernel::builder();
    kernel_builder_n
        .program(&program)
        .queue(queue.clone())
        .name("build")
        .global_work_size(WORK_SIZE);
    // Add named argument to kernel
    kernel_args!(kernel_builder_n, ("name", 0.0f32));
    let kernel_n = kernel_builder_n.build().unwrap();
    // Set named argument
    kernel_n.set_arg("name", 10.0f32).unwrap();
}