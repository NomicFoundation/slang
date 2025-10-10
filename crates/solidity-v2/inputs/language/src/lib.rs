//! This crate is responsible for creating the Solidity language definition and exposing it to downstream crates.

mod bindings;
mod definition;

pub use bindings::render_built_ins;
pub use definition::SolidityDefinition;
