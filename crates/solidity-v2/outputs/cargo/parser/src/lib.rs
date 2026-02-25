#![recursion_limit = "1024"] // for evaluating the 'logos' macro during build time

mod lexer;
mod parser;

pub use parser::{parser_error::ParserError, Parser};

// We use the lexer for testing purposes
#[cfg(feature = "__private_testing_utils")]
pub use lexer::{ContextKind, Lexer};
#[cfg(feature = "__private_testing_utils")]
pub use parser::{ContractDefinitionParser, ExpressionParser, SourceUnitParser};
