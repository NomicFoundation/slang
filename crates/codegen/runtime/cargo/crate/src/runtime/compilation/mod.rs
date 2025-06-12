//! The `compilation` module provides some basic utilities that users can use to combine multiple source files
//! into a single compilation unit. The compilation unit can then be used to get a complete [`BindingGraph`][`crate::bindings::BindingGraph`]
//! that can resolve references and defintions across all the added files.
//!
//! This API is a work in progress. For now, it's recommended to use our [Typescript API](https://nomicfoundation.github.io/slang/latest/user-guide/07-semantic-analysis/01-compilation-units/#71-compilation-units)
//! to build a [`CompilationUnit`] and resolve binding graphs.

mod file;
mod internal_builder;
mod unit;

pub use file::File;
pub use internal_builder::{AddFileResponse, InternalCompilationBuilder};
pub use unit::CompilationUnit;
