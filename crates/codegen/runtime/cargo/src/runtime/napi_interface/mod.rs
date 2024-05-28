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
type RustEdge = crate::cst::Edge;
type RustNode = crate::cst::Node;
type RustParseError = crate::parse_error::ParseError;
type RustParseOutput = crate::parse_output::ParseOutput;
type RustQuery = crate::query::Query;
type RustQueryMatch = crate::query::QueryMatch;
type RustQueryMatchIterator = crate::query::QueryMatchIterator;
type RustNonterminalNode = crate::cst::NonterminalNode;
type RustTextIndex = crate::text_index::TextIndex;
type RustTextRange = crate::text_index::TextRange;
type RustTerminalNode = crate::cst::TerminalNode;

type NonterminalKind = crate::kinds::NonterminalKind;
type TerminalKind = crate::kinds::TerminalKind;
type EdgeLabel = crate::kinds::EdgeLabel;
