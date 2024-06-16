use metaslang_cst::cst;

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

impl metaslang_cst::KindTypes for KindTypes {
    type NonterminalKind = crate::kinds::NonterminalKind;
    type TerminalKind = crate::kinds::TerminalKind;
    type EdgeLabel = crate::kinds::EdgeLabel;
}

pub type Node = cst::Node<KindTypes>;
pub type NonterminalNode = cst::NonterminalNode<KindTypes>;
pub type TerminalNode = cst::TerminalNode<KindTypes>;
pub type Edge = cst::Edge<KindTypes>;
