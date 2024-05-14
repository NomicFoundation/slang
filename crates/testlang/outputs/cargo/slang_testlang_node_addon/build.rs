#[cfg(not(feature = "slang_napi_interfaces"))]
compile_error!("The whole point is to enable slang_napi_interfaces for this cdylib crate!");

fn main() {
    napi_build::setup();
}
