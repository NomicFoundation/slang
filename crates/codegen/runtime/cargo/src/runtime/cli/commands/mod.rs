use thiserror::Error;

#[cfg(feature = "__graph_builder")]
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

    #[cfg(feature = "__graph_builder")]
    #[error(transparent)]
    ExecutionFailed(#[from] crate::graph_builder::ExecutionError),
}
