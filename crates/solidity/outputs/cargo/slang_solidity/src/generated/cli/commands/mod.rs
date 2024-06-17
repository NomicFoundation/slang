// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use thiserror::Error;

#[cfg(feature = "__experimental_bindings_api")]
pub mod bindings;
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

    #[error("Unknown error: {0}")]
    Unknown(String),
}
