// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use thiserror::Error;

#[cfg(feature = "__experimental_bindings_api")]
pub mod build_graph;
#[cfg(feature = "__experimental_bindings_api")]
pub mod build_stack_graph;
pub mod parse;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("File not found: {0:?}")]
    FileNotFound(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    LanguageError(#[from] crate::language::Error),

    #[error("Parsing failed: {0}")]
    ParseFailed(String),

    #[cfg(feature = "__experimental_bindings_api")]
    #[error(transparent)]
    ExecutionFailed(#[from] crate::bindings::ExecutionError),

    #[cfg(feature = "__experimental_bindings_api")]
    #[error(transparent)]
    BuildError(#[from] crate::bindings::stack_graph::BuildError),
}
