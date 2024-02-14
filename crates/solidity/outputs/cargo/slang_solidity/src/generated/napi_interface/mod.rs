// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

pub mod ast_selectors;
pub mod cst;
pub mod cursor;
pub mod parse_error;
pub mod parse_output;
pub mod text_index;

type RustCursor = crate::cursor::Cursor;
type RustNamedNode = crate::cst::NamedNode;
type RustNode = crate::cst::Node;
type RustParseError = crate::parse_error::ParseError;
type RustParseOutput = crate::parse_output::ParseOutput;
type RustRuleNode = crate::cst::RuleNode;
type RustTextIndex = crate::text_index::TextIndex;
type RustTextRange = crate::text_index::TextRange;
type RustTokenNode = crate::cst::TokenNode;

type RuleKind = crate::kinds::RuleKind;
type TokenKind = crate::kinds::TokenKind;
type FieldName = crate::kinds::FieldName;
