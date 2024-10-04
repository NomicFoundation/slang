// Disable dead-code checking in this stubs crate, as they are not necissarily used in tests.
// The final code (generated in output crates) is checked for dead-code anyways.
#![allow(dead_code, unused_imports)]

mod runtime;

use {codegen_runtime_cargo_crate as rust_crate, runtime as wasm_crate};
