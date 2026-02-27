#![recursion_limit = "1024"] // for evaluating the 'logos' macro during build time

mod lexer;
mod parser;

// We use the lexer and some of the parsers for testing purposes
#[cfg(feature = "__private_testing_utils")]
pub use lexer::{ContextKind, Lexer};
pub use parser::parser_error::ParserError;
pub use parser::SourceUnitParser;
