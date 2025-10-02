//! Source code parsing utilities. See [`parser::Parser`] for more info on parsing
//! source code.

mod lexer;
mod parse_error;
mod parse_output;
#[allow(clippy::module_inception)]
#[path = "parser.generated.rs"]
mod parser;
mod parser_support;
mod scanner_macros;

pub use parse_error::ParseError;
pub use parse_output::ParseOutput;
pub use parser::{Parser, ParserInitializationError};
