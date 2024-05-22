#[macro_export]
/// Builds a kernel with named arguments from a program, queue, kernel name and work size. Adds named arguments given as tuples of ("name", arg).
macro_rules! kernel_n {
    ($p:expr, $q:expr, $name:expr, $n:expr, $( $arg:expr),*) => {
        {
            let mut kernel_builder = Kernel::builder();
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
macro_rules! kernel {
    ($p:expr, $q:expr, $name:expr, $n:expr, $( $arg:expr),*) => {
        {
            let mut kernel_builder = Kernel::builder();
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
macro_rules! kernel_builder {
    ($p:expr, $q:expr, $name:expr, $n:expr) => {{
        let mut kernel_builder = Kernel::builder();
        kernel_builder
            .program(&$p)
            .name($name)
            .queue($q.clone())
            .global_work_size($n);
        kernel_builder
    }};
}

#[macro_export]
/// Appends named arguments given as tuples of ("name", arg).
macro_rules! kernel_args_n {
    ($kernel:expr, $( $arg:expr),*) => {
        $(
            $kernel.arg_named($arg.0, $arg.1);
        )*
    };
}

#[macro_export]
/// Appends unnamed arguments.
macro_rules! kernel_args {
    ($kernel:expr, $( $arg:expr),*) => {
        $(
            $kernel.arg($arg);
        )*
    };
}
