#![recursion_limit = "1024"] // for evaluating the 'logos' macro during build time

pub(crate) mod lexer;
pub(crate) mod parser;

pub use lexer::temp_sourcify;
