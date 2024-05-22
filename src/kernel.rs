#[macro_export]
/// Builds a kernel with named arguments from a program, queue, kernel name and work size. Adds named arguments given as tuples of ("name", arg).
///
/// Syntax: `(program: Program, queue: Queue, name: &str, size: Into<ocl::SpatialDims>, $(args: (&str, T)),*)`
macro_rules! kernel_n {
    ($p:expr, $q:expr, $name:expr, $n:expr, $( $arg:expr),*) => {
        {
            let mut kernel_builder = ocl::Kernel::builder();
            kernel_builder.program(&$p).name($name).queue($q.clone()).global_work_size($n);
            $(
                kernel_builder.arg_named($arg.0, $arg.1);
            )*
            kernel_builder.build().unwrap()
        }
    };
}

#[macro_export]
/// Builds a kernel with unnamed arguments from a program, queue, kernel name and work size. Adds unnamed arguments.
///
/// Syntax: `(program: Program, queue: Queue, name: &str, size: Into<ocl::SpatialDims>, $(args: T),*)`
macro_rules! kernel {
    ($p:expr, $q:expr, $name:expr, $n:expr, $( $arg:expr),*) => {
        {
            let mut kernel_builder = ocl::Kernel::builder();
            kernel_builder.program(&$p).name($name).queue($q.clone()).global_work_size($n);
            $(
                kernel_builder.arg($arg);
            )*
            kernel_builder.build().unwrap()
        }
    };
}

#[macro_export]
/// Creates a KernelBuilder from a program, queue, kernel name and work size.
///
/// Syntax: `(program: Program, queue: Queue, name: &str, size: Into<ocl::SpatialDims>)`
macro_rules! kernel_builder {
    ($p:expr, $q:expr, $name:expr, $n:expr) => {{
        let mut kernel_builder = ocl::Kernel::builder();
        kernel_builder
            .program(&$p)
            .name($name)
            .queue($q.clone())
            .global_work_size($n);
        kernel_builder
    }};
}

#[macro_export]
/// Adds named arguments to Kernel given as tuples of ("name", arg).
///
/// Syntax: `(kernel: Kernel, args: (&str, T))`
macro_rules! kernel_args_n {
    ($kernel:expr, $( $arg:expr),*) => {
        $(
            $kernel.arg_named($arg.0, $arg.1);
        )*
    };
}

#[macro_export]
/// Adds unnamed arguments to Kernel.
///
/// Syntax: `(kernel: Kernel, args: T)`
macro_rules! kernel_args {
    ($kernel:expr, $( $arg:expr),*) => {
        $(
            $kernel.arg($arg);
        )*
    };
}
