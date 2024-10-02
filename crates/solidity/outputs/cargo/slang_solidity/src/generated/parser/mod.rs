// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

mod lexer;
mod parse_error;
mod parse_output;
#[allow(clippy::module_inception)]
#[path = "generated/parser.rs"]
mod parser;
mod parser_support;
mod scanner_macros;

pub use parse_error::ParseError;
pub use parse_output::ParseOutput;
pub use parser::{Parser, ParserInitializationError};
