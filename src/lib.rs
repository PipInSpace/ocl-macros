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
//! use ocl::{Context, Device, Platform, Queue};
//! use ocl_macros::*;
//! 
//! // Initialize OpenCL context/queue
//! let platform = Platform::default();
//! let devices = Device::list_all(platform).expect("Cannot find devices");
//! if devices.is_empty() {
//!     panic!("No OpenCL Device detected")
//! }
//! let context = Context::builder()
//!     .platform(platform)
//!     .devices(devices[0])
//!     .build()
//!     .unwrap();
//! let queue = Queue::new(&context, devices[0], None).unwrap();
//! 
//! // Construct new ocl float buffer with 100 elements of starting value 0.0
//! let buffer = buffer!(&queue, 100, 0.0f32);
//! ```
//! This code uses the `buffer!()` macro to create a new `ocl::Buffer<f32>` with 100 elements of `0.0f32`.
//!
//!
//! [issue]: https://github.com/pipinspace/ocl-macros/issues
//! [`ocl`]: https://github.com/cogciprocate/ocl

extern crate ocl;

#[cfg(test)]
mod tests;

pub mod kernel;
pub mod buffer;
pub mod proque;
