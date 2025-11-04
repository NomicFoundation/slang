//! Query module.

mod engine;
mod model;
mod parser;
mod syntax_engine;

pub use engine::{Capture, QueryMatch, QueryMatchIterator};
pub use model::Query;
pub use parser::{CaptureQuantifier, QueryError};
pub use syntax_engine::{SyntaxCapture, SyntaxQueryMatch, SyntaxQueryMatchIterator};
