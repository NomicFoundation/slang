// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[path = "./generated/bindgen.rs"]
mod bindgen;
mod utils;
mod wrappers;

struct World;

crate::wasm_crate::bindgen::export!(World with_types_in crate::wasm_crate::bindgen);
