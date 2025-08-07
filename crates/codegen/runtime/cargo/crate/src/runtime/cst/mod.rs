#[path = "generated/edge_label.rs"]
mod edge_label;
#[path = "generated/lexical_context.rs"]
mod lexical_context;
#[path = "generated/nonterminal_kind.rs"]
mod nonterminal_kind;
#[path = "generated/terminal_kind.rs"]
mod terminal_kind;

pub use edge_label::EdgeLabel;
pub(crate) use lexical_context::{IsLexicalContext, LexicalContext, LexicalContextType};
pub use metaslang_cst::kinds::{
    EdgeLabelExtensions, NonterminalKindExtensions, TerminalKindExtensions,
};
pub use nonterminal_kind::NonterminalKind;
pub use terminal_kind::TerminalKind;

// These derives are because default #[derive(...)] on a generic type implements only the trait
// with default bounds also implied for the generic types as well, i.e.
//
// #[derive(Clone)] // expands to `impl<T: Clone> Clone for MyOption<T> { ... }` (notice the `T: Clone`)
// struct MyOption<T>(Option<T>);
//
// This assumes that the underlying data type uses this internally, however it's only used as a
// type container/marker.
//
// A slightly more "correct" approach would be to implement the traits while skipping the bounds for
// the type marker, however this can be more noisy
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub enum KindTypes {}

impl metaslang_cst::kinds::KindTypes for KindTypes {
    type NonterminalKind = NonterminalKind;
    type TerminalKind = TerminalKind;
    type EdgeLabel = EdgeLabel;
}

pub type NodeKind = metaslang_cst::kinds::NodeKind<KindTypes>;
pub type NodeId = metaslang_cst::nodes::NodeId;

pub type Node = metaslang_cst::nodes::Node<KindTypes>;
pub type NonterminalNode = metaslang_cst::nodes::NonterminalNode<KindTypes>;
pub type TerminalNode = metaslang_cst::nodes::TerminalNode<KindTypes>;
pub type Edge = metaslang_cst::nodes::Edge<KindTypes>;

pub type Cursor = metaslang_cst::cursor::Cursor<KindTypes>;
pub type CursorIterator = metaslang_cst::cursor::CursorIterator<KindTypes>;
pub type AncestorsIterator = metaslang_cst::cursor::AncestorsIterator<KindTypes>;

pub type Query = metaslang_cst::query::Query<KindTypes>;
pub use metaslang_cst::query::QueryError;
pub type QueryMatch = metaslang_cst::query::QueryMatch<KindTypes>;
pub type QueryMatchIterator = metaslang_cst::query::QueryMatchIterator<KindTypes>;

pub use metaslang_cst::text_index::{TextIndex, TextRange, TextRangeExtensions};
