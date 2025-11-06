#![recursion_limit = "1024"] // for evaluating the 'logos' macro during build time

mod lexer;

pub use lexer::temp_sourcify;
