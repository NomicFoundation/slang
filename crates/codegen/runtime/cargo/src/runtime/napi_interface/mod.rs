pub mod cst;
pub mod cursor;
pub mod parse_error;
pub mod parse_output;
pub mod query;
pub mod text_index;

#[path = "generated/ast_selectors.rs"]
pub mod ast_selectors;

type RustCursor = crate::cursor::Cursor;
type RustInvalidNode = crate::cst::InvalidNode;
type RustNode = crate::cst::Node;
type RustParseError = crate::parse_error::ParseError;
type RustParseOutput = crate::parse_output::ParseOutput;
type RustQuery = crate::query::Query;
type RustQueryResult = crate::query::QueryResult;
type RustQueryResultIterator = crate::query::QueryResultIterator;
type RustRuleNode = crate::cst::RuleNode;
type RustTextIndex = crate::text_index::TextIndex;
type RustTextRange = crate::text_index::TextRange;
type RustTokenNode = crate::cst::TokenNode;

type RuleKind = crate::kinds::RuleKind;
type TokenKind = crate::kinds::TokenKind;
type NodeLabel = crate::kinds::NodeLabel;
