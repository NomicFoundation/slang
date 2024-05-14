#[macro_use]
pub(crate) mod parser_support;
pub(crate) mod lexer;

pub mod cst;
pub mod cursor;
pub mod diagnostic;
pub mod parse_error;
pub mod parse_output;
pub mod query;
pub mod text_index;

#[path = "generated/kinds.rs"]
pub mod kinds;
#[path = "generated/language.rs"]
pub mod language;

#[cfg(feature = "slang_napi_interfaces")]
pub mod napi_interface;
