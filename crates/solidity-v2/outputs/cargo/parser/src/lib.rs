#![recursion_limit = "1024"] // for evaluating the 'logos' macro during build time

mod lexer;
mod parser;

pub use lexer::temp_sourcify;
pub use parser::{temp_testing, Parser};
