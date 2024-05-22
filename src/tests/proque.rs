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
    let _kernel = pq_kernel!(pq, "build", 0.0f32);
}

#[test]
fn macro_pq_kernel_n() {
    let pq = ocl::ProQue::builder()
        .src(super::PROGRAM_SRC)
        .dims(100)
        .build()
        .expect("Build ProQue");
    let _kernel = pq_kernel_n!(pq, "build", ("test_float", 0.0f32));
}
