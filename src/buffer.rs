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
