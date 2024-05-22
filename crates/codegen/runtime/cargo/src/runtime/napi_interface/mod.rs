pub mod cst;
pub mod cursor;
pub mod diagnostic;
pub mod parse_error;
pub mod parse_output;
pub mod query;
pub mod text_index;

#[path = "generated/ast_selectors.rs"]
pub mod ast_selectors;

type RustCursor = crate::cursor::Cursor;
type RustLabeledNode = crate::cst::Edge;
type RustNode = crate::cst::Node;
type RustParseError = crate::parse_error::ParseError;
type RustParseOutput = crate::parse_output::ParseOutput;
type RustQuery = crate::query::Query;
type RustQueryResult = crate::query::QueryResult;
type RustQueryResultIterator = crate::query::QueryResultIterator;
type RustRuleNode = crate::cst::NonTerminalNode;
type RustTextIndex = crate::text_index::TextIndex;
type RustTextRange = crate::text_index::TextRange;
type RustTokenNode = crate::cst::TerminalNode;

type NonTerminalKind = crate::kinds::NonTerminalKind;
type TerminalKind = crate::kinds::TerminalKind;
type EdgeLabel = crate::kinds::EdgeLabel;
