# ocl-macros

#### [Documentation](https://docs.rs/ocl-macros) | [Change Log](https://github.com/pipinspace/ocl-macros/blob/main/RELEASES.md)

[![](https://img.shields.io/crates/v/ocl-macros.svg)](https://crates.io/crates/ocl-macros) [![](https://docs.rs/ocl-macros/badge.svg)](https://docs.rs/ocl-macros)

Macros for easier/faster working with the [ocl](https://github.com/cogciprocate/ocl) crate. ocl-macros currently supports rust macros for easy ocl::Kernel and ocl::Buffer creation. 

This crate is still in early development.

## Features

- Simple Kernel creation in one line.
- Easy specifying of multiple kernel arguments, named or unnamed
- Buffer creation in one line

## Usage

If you want to use ocl-macros with ocl, add the following to your `Cargo.toml`:

```toml
[dependencies]
ocl-macros = "0.0.4"
```
Import the macros into your project by adding the following to your project's root:
```rust
use ocl_macros::*;
```

## Example

Example use of ocl-macros:
```rust
use ocl::{Context, Device, Platform, Program, Queue};
use ocl_macros::*;

let PROGRAM_SRC: &str = r#"
kernel void add_one(global float* var) {
    const uint n = get_global_id(0);
    // This program adds 1.0f to each element in the buffer
    var[n] += 1.0f;
}
"#;

// Initialize OpenCL context/queue
let platform = Platform::default();
// Get first device on default platform
let device = default_device!();
let context = Context::builder()
    .platform(platform)
    .devices(device)
    .build()
    .unwrap();
let queue = Queue::new(&context, device, None).unwrap();
let program = Program::builder()
    .devices(device)
    .src(PROGRAM_SRC)
    .build(&context)
    .unwrap();

// Create new float buffer with 100 elements of starting value 0.0f32
let buffer = buffer!(&queue, 100, 0.0f32);
// Create the "add_one" kernel with work size 100 and buffer as first unnamed argument
let kernel = kernel!(program, queue, "add_one", 100, &buffer);

// Run kernel (This is unsafe)
unsafe { kernel.enq().unwrap(); }
// Get buffer content as new vector
let vec = bget!(buffer);
// The elements are now 1.0f32!
assert!(vec[0] == 1.0f32);
```
This code uses various macros to create a new buffer and kernel with the created buffer as an unnamed argument.
The kernel is then executed. The buffer content is read into a new vector and verified. It has changed from 0.0 to 1.0!

## License

This crate is licensed under the [MIT license](LICENSE) or the Apache-2.0 license.

## Contribution

Contributions are welcome! Any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.