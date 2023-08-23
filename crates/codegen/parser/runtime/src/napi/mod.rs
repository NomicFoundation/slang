pub mod napi_cst;
pub mod napi_cursor;
pub mod napi_parse_error;
pub mod napi_parse_output;
pub mod napi_text_index;

pub type RustCursor = super::cursor::Cursor;
pub type RustNode = super::cst::Node;
pub type RustParseOutput = super::parse_output::ParseOutput;
pub type RustParseError = super::parse_error::ParseError;
pub type RustRuleNode = super::cst::RuleNode;
pub type RustTokenNode = super::cst::TokenNode;
pub type RustTextIndex = super::text_index::TextIndex;
pub type RustTextRange = super::text_index::TextRange;

pub use super::kinds::*;
