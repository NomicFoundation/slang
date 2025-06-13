// Disable dead-code checking in this stubs crate, as they are not necissarily used in tests.
// The final code (generated in output crates) is checked for dead-code anyways.
// We generate functions prefixed with `_` in order to avoid having to be forced to use them.
#![allow(dead_code, unused_imports, clippy::used_underscore_items)]

mod runtime;

use {codegen_runtime_cargo_crate as rust_crate, runtime as wasm_crate};
