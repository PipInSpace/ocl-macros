#[macro_export]
/// Returns a vector of all available devices on the default platform.
macro_rules! device_vec {
    () => {{
        ocl::Device::list_all(ocl::Platform::default()).expect("Cannot find devices")
    }};
}

#[macro_export]
/// Returns the first available device on the default platform.
macro_rules! default_device {
    () => {{
        let vec: Vec<ocl::Device> =
            ocl::Device::list_all(ocl::Platform::default()).expect("Cannot find devices");
        vec[0]
    }};
}
