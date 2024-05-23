# ocl-macros

#### [Documentation](https://docs.rs/ocl-macros) | [Change Log](https://github.com/pipinspace/ocl-macros/blob/master/RELEASES.md)

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
ocl-macros = "0.0.3"
```
Import the macros into your project by adding the following to your project's root:
```rust
use ocl_macros::*;
```

## Example

Example use of ocl-macros:
```rust
use ocl::{Context, Device, Platform, Queue};
use ocl_macros::*;

// Initialize OpenCL context/queue
let platform = Platform::default();
let devices = Device::list_all(platform).expect("Cannot find devices");
if devices.is_empty() {
    panic!("No OpenCL Device detected")
}
let context = Context::builder()
    .platform(platform)
    .devices(devices[0])
    .build()
    .unwrap();
let queue = Queue::new(&context, devices[0], None).unwrap();

// Construct new ocl float buffer with 100 elements of starting value 0.0
let buffer = buffer!(&queue, 100, 0.0f32);
```

## License

This crate is licensed under the [MIT license](LICENSE) or the Apache-2.0 license.

## Contribution

Contributions are welcome! Any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.