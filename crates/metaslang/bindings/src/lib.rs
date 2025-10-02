#![deny(missing_docs)]

//! This module contains the core utilities for dealing with the binding graph for Slang.
//!
//! It's based on the [stack graph library](https://docs.rs/stack-graphs/latest/stack_graphs/index.html).

#[doc(hidden)]
mod builder;
mod graph;

#[doc(hidden)]
pub use builder::{
    BindingGraphBuilder, FileGraphBuilder, GraphHandle, PathResolver, ScopeGraphBuilder,
};
pub use graph::{
    BindingGraph, BindingLocation, BuiltInLocation, Definition, Reference, UserFileLocation,
};
