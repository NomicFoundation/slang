use thiserror::Error;

#[cfg(feature = "__experimental_bindings_api")]
pub mod bindings;
#[cfg(feature = "__experimental_bindings_api")]
pub mod build_graph;
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
    ExecutionFailed(#[from] metaslang_graph_builder::ExecutionError),

    #[cfg(feature = "__experimental_bindings_api")]
    #[error(transparent)]
    BuildError(#[from] metaslang_bindings::builder::BuildError),

    #[cfg(feature = "__experimental_bindings_api")]
    #[error(transparent)]
    BindingsError(#[from] crate::bindings::BindingsError),
}
