// We generate functions prefixed with `_` in order to avoid having to be forced to use them.
#![allow(clippy::used_underscore_items)]
mod generated;

use {generated as wasm_crate, slang_testlang as rust_crate};
