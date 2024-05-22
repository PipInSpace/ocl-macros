use crate::*;
use ocl::Kernel;
use super::{init_program_queue, PROGRAM_SRC};

// 
// Tests for kernel macros
// 

const WORK_SIZE: i32 = 1;

#[test]
fn macro_kernel() {
    let (program, queue) = init_program_queue(PROGRAM_SRC);
    let _kernel = kernel!(program, queue, "build", WORK_SIZE, 0.0f32);
}

#[test]
fn macro_kernel_n() {
    let (program, queue) = init_program_queue(PROGRAM_SRC);
    let _kernel = kernel_n!(program, queue, "build", WORK_SIZE, ("test_float", 0.0f32));
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
    let mut kernel_builder = ocl::Kernel::builder();
    kernel_builder.program(&program).queue(queue).name("build").global_work_size(WORK_SIZE);
    kernel_args!(kernel_builder, 0.0f32);
    let _kernel = kernel_builder.build().unwrap();
}

#[test]
fn macro_kernel_args_n() {
    let (program, queue) = init_program_queue(PROGRAM_SRC);
    let mut kernel_builder = ocl::Kernel::builder();
    kernel_builder.program(&program).queue(queue).name("build").global_work_size(WORK_SIZE);
    kernel_args_n!(kernel_builder, ("test_float", 0.0f32));
    let _kernel = kernel_builder.build().unwrap();
}
