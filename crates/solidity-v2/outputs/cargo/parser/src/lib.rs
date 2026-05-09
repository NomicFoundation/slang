#![recursion_limit = "1024"] // for evaluating the 'logos' macro during build time

pub mod diagnostics;
mod lexer;
mod parser;

pub use parser::{ParseOutput, Parser};
