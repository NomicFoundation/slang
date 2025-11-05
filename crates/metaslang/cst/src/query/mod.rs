//! Query module.

mod engine;
mod model;
mod parser;

pub use engine::{Capture, QueryMatch, QueryMatchIterator};
pub use model::Query;
pub use parser::{CaptureQuantifier, QueryError};

#[cfg(feature = "syntax")]
mod syntax_engine;

#[cfg(feature = "syntax")]
pub use syntax_engine::{SyntaxCapture, SyntaxQueryMatch, SyntaxQueryMatchIterator};
