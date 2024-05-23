#[macro_export]
/// Creates an ocl process queue struct.
///
/// Syntax: `(src: &str, size: Into<ocl::SpatialDims>)`
macro_rules! proque {
    ($src:expr, $size:expr) => {{
        ocl::ProQue::builder()
            .src($src)
            .dims($size)
            .build()
            .expect("Build ProQue");
    }};
}

#[macro_export]
/// Builds a kernel with named arguments from a ProQue and the kernel name. Adds named arguments given as tuples of ("name", arg).
///
/// Syntax: `(proque: ProQue, name: &str, $(args: (&str, T)),*)`
macro_rules! pq_kernel_n {
    ($pq:expr, $name:expr, $( $arg:expr),*) => {
        {
            let mut kernel_builder = $pq.kernel_builder($name);
            $(
                kernel_builder.arg_named($arg.0, $arg.1);
            )*
            kernel_builder.build().unwrap()
        }
    };
}

#[macro_export]
/// Builds a kernel with arguments from a ProQue and the kernel name. Adds unnamed arguments or named arguments given as tuples of ("name", arg).
///
/// Syntax: `(proque: ProQue, name: &str, $(args: T),*)`
macro_rules! pq_kernel {
    ($pq:expr, $name:expr, $( ($argname:expr, $arg:expr)),*) => {
        {
            let mut kernel_builder = $pq.kernel_builder($name);
            $(
                kernel_builder.arg_named($argname, $arg);
            )*
            kernel_builder.build().unwrap()
        }
    };
    ($pq:expr, $name:expr, $( $arg:expr),*) => {
        {
            let mut kernel_builder = $pq.kernel_builder($name);
            $(
                kernel_builder.arg($arg);
            )*
            kernel_builder.build().unwrap()
        }
    };
}
