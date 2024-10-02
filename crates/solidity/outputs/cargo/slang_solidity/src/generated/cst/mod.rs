// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[path = "generated/kinds.rs"]
mod kinds;

pub use kinds::{EdgeLabel, NonterminalKind, TerminalKind};
pub(crate) use kinds::{IsLexicalContext, LexicalContext, LexicalContextType};
pub use metaslang_cst::kinds::{
    EdgeLabelExtensions, NonterminalKindExtensions, TerminalKindExtensions,
};

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

pub type Node = metaslang_cst::nodes::Node<KindTypes>;
pub type NonterminalNode = metaslang_cst::nodes::NonterminalNode<KindTypes>;
pub type TerminalNode = metaslang_cst::nodes::TerminalNode<KindTypes>;
pub type Edge = metaslang_cst::nodes::Edge<KindTypes>;

pub type Cursor = metaslang_cst::cursor::Cursor<KindTypes>;
pub type CursorWithEdges = metaslang_cst::cursor::CursorWithEdges<KindTypes>;

pub type Query = metaslang_cst::query::Query<KindTypes>;
pub type QueryMatch = metaslang_cst::query::QueryMatch<KindTypes>;
pub type QueryMatchIterator = metaslang_cst::query::QueryMatchIterator<KindTypes>;
pub use metaslang_cst::query::QueryError;
pub use metaslang_cst::text_index::{TextIndex, TextRange, TextRangeExtensions};
