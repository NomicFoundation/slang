#![allow(dead_code)]

#[macro_use]
mod parser_support;

pub mod binding;
pub mod cst;
pub mod cursor;
pub mod kinds;
pub(crate) mod lexer;
pub mod parse_error;
pub mod parse_output;
pub mod query;
pub mod text_index;
mod user_defined;

#[cfg(feature = "slang_napi_interfaces")]
pub mod napi_interface;
