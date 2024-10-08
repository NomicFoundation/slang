// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[path = "./generated/bindings.rs"]
mod bindings;
mod utils;
mod wrappers;

struct World;

crate::wasm_crate::bindings::export!(World with_types_in crate::wasm_crate::bindings);
