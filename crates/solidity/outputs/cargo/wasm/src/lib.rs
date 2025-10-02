// We generate functions prefixed with `_` in order to avoid having to be forced to use them.
#![allow(clippy::used_underscore_items)]

mod bindgen;
mod utils;
mod wrappers;

struct World;

bindgen::export!(World with_types_in crate::bindgen);

use slang_solidity as rust_crate;
