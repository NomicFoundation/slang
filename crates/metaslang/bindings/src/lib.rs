#![deny(missing_docs)]

//! This module contains the core utilities for dealing with the binding graph for Slang.
//!
//! It's based on the [stack graph library](https://docs.rs/stack-graphs/latest/stack_graphs/index.html).

// This module is not part of the public API.
#[allow(missing_docs)]
#[cfg_attr(not(feature = "__metaslang_graph_builder"), allow(dead_code))]
mod builder;
mod graph;

#[cfg(not(feature = "__metaslang_graph_builder"))]
pub(crate) use builder::PathResolver;
#[cfg(feature = "__metaslang_graph_builder")]
pub use builder::{
    BindingGraphBuilder, FileGraphBuilder, GraphHandle, PathResolver, ScopeGraphBuilder,
};
pub use graph::{
    BindingGraph, BindingLocation, BuiltInLocation, Definition, Reference, UserFileLocation,
};
