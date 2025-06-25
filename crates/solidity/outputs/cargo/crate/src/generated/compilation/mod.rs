// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

//! The `compilation` module provides some basic utilities that users can use to combine multiple source files
//! into a single compilation unit. The compilation unit can then be used to get a complete [`BindingGraph`][`crate::bindings::BindingGraph`]
//! that can resolve references and defintions across all the added files.

mod compilation_builder;
mod file;
mod internal_builder;
mod unit;

pub use compilation_builder::{AddFileError, CompilationBuilder, CompilationBuilderConfig};
pub use file::File;
pub use internal_builder::{AddFileResponse, InternalCompilationBuilder};
pub use unit::CompilationUnit;
