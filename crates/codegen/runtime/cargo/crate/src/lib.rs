// Disable dead-code checking in this stubs crate, as they are not necissarily used in tests.
// The final code (generated in output crates) is checked for dead-code anyways.
#![allow(dead_code, unused_imports)]

mod runtime;

pub use runtime::*;
