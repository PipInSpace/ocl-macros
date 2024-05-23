//! Tests for ProQue macros

use crate::*;

#[test]
fn macro_proque() {
    let _proque = proque!(super::PROGRAM_SRC, 100);
}

#[test]
fn macro_pq_kernel() {
    let pq = ocl::ProQue::builder()
        .src(super::PROGRAM_SRC)
        .dims(100)
        .build()
        .expect("Build ProQue");
    
    // Create kernel with unnamed arguments
    let kernel = pq_kernel!(pq, "build", 0.0f32);

    // Create kernel with named arguments
    let kernel_n = pq_kernel!(pq, "build", ("name", 0.0f32));

    assert!(kernel.set_arg("name", 10.0f32).is_err()); // This should fail due to lack of named arguments
    kernel_n.set_arg("name", 10.0f32).unwrap();
}
