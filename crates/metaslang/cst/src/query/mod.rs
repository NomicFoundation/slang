//! Query module.

mod engine;
mod model;
mod parser;

pub use engine::{QueryMatch, QueryMatchIterator};
pub use model::Query;
pub use parser::{CaptureQuantifier, QueryError};
