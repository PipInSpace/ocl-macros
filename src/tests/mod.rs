use ocl::{Context, Device, Platform, Program, Queue};

// 
// Tests for ocl-macros
// 

mod kernel;
mod buffer;

const PROGRAM_SRC: &str = r#"
kernel void build(const float var) {
    printf("works");
}
"#;

fn init_queue() -> Queue {
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

    queue
}

fn init_program_queue(src: &str) -> (Program, Queue) {
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
    let program = Program::builder()
        .devices(devices[0])
        .src(src)
        .build(&context)
        .unwrap();

    (program, queue)
}