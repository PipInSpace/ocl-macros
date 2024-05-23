//! # [![](https://img.shields.io/crates/v/ocl-macros.svg)](https://crates.io/crates/ocl-macros) | [GitHub](https://github.com/pipinspace/ocl-macros)
//!
//! Macros for easier/faster working with the [`ocl`] crate. `ocl-macros` currently supports rust macros for easy ocl::Kernel and ocl::Buffer creation.
//!
//! This crate is still in early development. If you encounter any problems or have a suggestion, feel free to open an [issue].
//!
//! ## Simple Example
//!
//! This is an example use-case of ocl-macros:
//! ```rust
//! use ocl::{Context, Device, Platform, Program, Queue};
//! use ocl_macros::*;
//!
//! let PROGRAM_SRC: &str = r#"
//! kernel void build(global float* var) {
//!     const uint n = get_global_id(0);
//!     // This program adds 1.0f to each element in the buffer
//!     var[n] += 1.0f;
//! }
//! "#;
//! 
//! // Initialize OpenCL context/queue
//! let platform = Platform::default();
//! // Get first device on default platform
//! let device = default_device!();
//! let context = Context::builder()
//!     .platform(platform)
//!     .devices(device)
//!     .build()
//!     .unwrap();
//! let queue = Queue::new(&context, device, None).unwrap();
//! let program = Program::builder()
//!     .devices(device)
//!     .src(PROGRAM_SRC)
//!     .build(&context)
//!     .unwrap();
//! 
//! // Create new float buffer with 100 elements of starting value 0.0f32
//! let buffer = buffer!(&queue, 100, 0.0f32);
//! // Create the "build" kernel with work size 100 and buffer as first unnamed argument
//! let kernel = kernel!(program, queue, "build", 100, &buffer);
//! 
//! // Run kernel (This is unsafe)
//! unsafe { kernel.enq().unwrap(); }
//! // Get buffer content as new vector
//! let vec = bget!(buffer);
//! // The elements are now 1.0f32!
//! assert!(vec[0] == 1.0f32);
//! ```
//! This code uses the `buffer!()` macro to create a new `ocl::Buffer<f32>` with 100 elements of `0.0f32`.
//!
//!
//! [issue]: https://github.com/pipinspace/ocl-macros/issues
//! [`ocl`]: https://github.com/cogciprocate/ocl

extern crate ocl;

#[cfg(test)]
mod tests;

pub mod buffer;
pub mod device;
pub mod kernel;
pub mod proque;
