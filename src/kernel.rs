#[macro_export]
/// Builds a kernel with arguments from a program, queue, kernel name and work size. Adds unnamed arguments or named arguments given as tuples of ("name", arg).
///
/// Syntax: `(program: Program, queue: Queue, name: &str, size: Into<ocl::SpatialDims>, $(args: T OR (&str, T)),*)`
macro_rules! kernel {
    ($p:expr, $q:expr, $name:expr, $n:expr, $( ($argname:expr, $arg:expr)),*) => { // Named arguments as tuples
        {
            let mut kernel_builder = ocl::Kernel::builder();
            kernel_builder.program(&$p).name($name).queue($q.clone()).global_work_size($n);
            $(
                kernel_builder.arg_named($argname, $arg);
            )*
            kernel_builder.build().unwrap()
        }
    };
    ($p:expr, $q:expr, $name:expr, $n:expr, $( $arg:expr),*) => { // Unnamed arguments
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
/// Adds unnamed arguments or named arguments given as tuples of ("name", arg) to Kernel.
///
/// Syntax: `(kernel: Kernel, $(args: T OR (&str, T)),*)`
macro_rules! kernel_args {
    ($kernel:expr, $( ($argname:expr, $arg:expr)),*) => { // Named arguments
        $(
            $kernel.arg_named($argname, $arg);
        )*
    };
    ($kernel:expr, $( $arg:expr),*) => { // Unnamed arguments
        $(
            $kernel.arg($arg);
        )*
    };
}
