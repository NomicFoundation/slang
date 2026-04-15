#![recursion_limit = "1024"] // for evaluating the 'logos' macro during build time

mod lexer;
mod parser;

pub use parser::parser_error::ParserError;
pub use parser::{ParseOutput, Parser};
