// The `generate!()` macro below expands into the Rust bindings to be implemented by our wrappers.
// You can use 'cargo expand' to see its output, or run the 'wit-bindgen rust' command.

wit_bindgen::generate!(in "src/wasm_crate/interface/");
