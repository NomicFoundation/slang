pub mod cst;
pub mod diagnostic;
pub mod parser;

#[path = "generated/ast_selectors.rs"]
pub mod ast_selectors;

type RustCursor = crate::cst::Cursor;
type RustEdge = crate::cst::Edge;
type RustNode = crate::cst::Node;
type RustParseError = crate::parser::ParseError;
type RustParseOutput = crate::parser::ParseOutput;
type RustQuery = crate::cst::Query;
type RustQueryMatch = crate::cst::QueryMatch;
type RustQueryMatchIterator = crate::cst::QueryMatchIterator;
type RustNonterminalNode = crate::cst::NonterminalNode;
type RustTextIndex = crate::cst::TextIndex;
type RustTextRange = crate::cst::TextRange;
type RustTerminalNode = crate::cst::TerminalNode;

type NonterminalKind = crate::cst::NonterminalKind;
type TerminalKind = crate::cst::TerminalKind;
type EdgeLabel = crate::cst::EdgeLabel;