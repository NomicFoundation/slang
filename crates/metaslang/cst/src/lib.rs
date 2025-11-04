#![deny(missing_docs)]

//! `metaslang` contains the core utilities that make Slang.

pub mod cursor;
pub mod kinds;
pub mod nodes;
pub mod query;
pub mod text_index;

#[cfg(feature = "syntax")]
#[allow(missing_docs)]
pub mod syntax_node;

#[cfg(feature = "syntax")]
#[allow(missing_docs)]
pub mod syntax_cursor;
