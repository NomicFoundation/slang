#[cfg(not(feature = "__private_napi_interfaces"))]
compile_error!("The whole point is to enable __private_napi_interfaces for this cdylib crate!");

fn main() {
    napi_build::setup();
}
