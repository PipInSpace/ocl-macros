use ocl::{flags, Buffer, MemFlags, Queue};

#[allow(unused)]
/// Creates a buffer of the specified size and flags. The type is inferred from the fill value.
pub fn create_buffer<T: ocl::OclPrm, I: Into<ocl::SpatialDims> + Clone>(
    queue: &Queue,
    size: I,
    fill_value: T,
    flags: MemFlags,
) -> Buffer<T> {
    if (size.clone().into() as ocl::SpatialDims).to_len() >= 1 {
        return Buffer::<T>::builder()
            .queue(queue.clone())
            .len(size)
            .fill_val(fill_value)
            .flags(flags::MEM_READ_WRITE)
            .build()
            .unwrap();
    }
    // use size of 1 if invalid
    return Buffer::<T>::builder()
        .queue(queue.clone())
        .len([1])
        .fill_val(fill_value)
        .flags(flags::MEM_READ_WRITE)
        .build()
        .unwrap();
}

#[allow(unused)]
/// Creates a vector from a buffer with the same size and type.
pub fn create_vec_from_buffer<T: ocl::OclPrm>(buffer: &Buffer<T>) -> Vec<T> {
    let s = buffer.len();
    let v: Vec<T> = vec![T::default(); s];
    v
}

#[macro_export]
/// Creates a buffer of the specified size with MEM_READ_WRITE flags. The type is inferred from the fill value.
///
/// Syntax: `(queue: Queue, size: Into<ocl::SpatialDims>, fill: ocl::OclPrm)`
macro_rules! buffer {
    ($queue:expr, $size:expr, $fill:expr) => {{
        buffer::create_buffer($queue, $size, $fill, ocl::flags::MEM_READ_WRITE)
    }};
}

#[macro_export]
/// Creates a buffer of the specified size and flags. The type is inferred from the fill value.
///
/// Syntax: `(queue: Queue, size: Into<ocl::SpatialDims>, fill: ocl::OclPrm, flags: ocl::MemFlags)`
macro_rules! buffer_flags {
    ($queue:expr, $size:expr, $fill:expr, $flags:expr) => {{
        buffer::create_buffer($queue, $size, $fill, $flags)
    }};
}

#[macro_export]
/// Reads a buffer into a vector.
///
/// Syntax: `(buffer: Buffer<T>, vec: Vec<T>)`
macro_rules! bread {
    ($buffer:expr, $vec:expr) => {{
        $buffer.read(&mut $vec).enq().unwrap();
    }};
}

#[macro_export]
/// Writes a vector into a buffer.
///
/// Syntax: `(buffer: Buffer<T>, vec: Vec<T>)`
macro_rules! bwrite {
    ($buffer:expr, $vec:expr) => {{
        $buffer.write(&$vec).enq().unwrap();
    }};
}

#[macro_export]
/// Creates a new vector from a buffer.
///
/// Syntax: `(buffer: Buffer<T>)`
macro_rules! bget {
    ($buffer:expr) => {{
        let mut vec = buffer::create_vec_from_buffer(&$buffer);
        $buffer.read(&mut vec).enq().unwrap();
        vec
    }};
}
