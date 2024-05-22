
#[macro_export]
// Creates an ocl process queue struct.
macro_rules! proque {
    ($src:expr, $size:expr) => {
        {
            ProQue::builder()
                .src($src)
                .dims($size)
                .build()
                .expect("Build ProQue");
        }
    };
}

#[macro_export]
/// Builds a kernel with named arguments from a ProQue and the kernel name. Adds named arguments given as tuples of ("name", arg).
/// 
/// Syntax: `(proque: ProQue, name: &str, args: (&str, T))`
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
/// Builds a kernel with unnamed arguments from a ProQue and the kernel name. Adds unnamed arguments.
/// 
/// Syntax: `(proque: ProQue, name: &str, args: T)`
macro_rules! pq_kernel {
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