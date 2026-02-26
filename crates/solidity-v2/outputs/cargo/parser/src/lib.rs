#![recursion_limit = "1024"] // for evaluating the 'logos' macro during build time

mod lexer;
mod parser;

pub use parser::{parser_error::ParserError, Parser, SourceUnitParser};

// We use the lexer and some of the parsers for testing purposes
#[cfg(feature = "__private_testing_utils")]
pub use lexer::{ContextKind, Lexer};
#[cfg(feature = "__private_testing_utils")]
pub use parser::{ContractDefinitionParser, ExpressionParser};
