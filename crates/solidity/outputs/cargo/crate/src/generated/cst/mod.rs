// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

//! This module defines the types that represent a concrete syntax tree (CST), such as [`Node`] and
//! [`Edge`]. It also defines utilities that are used for navigating and searching a CST, [`Cursor`] and
//! [`Query`].

#[path = "generated/edge_label.rs"]
mod edge_label;
#[path = "generated/lexical_context.rs"]
mod lexical_context;
#[path = "generated/nonterminal_kind.rs"]
mod nonterminal_kind;
#[path = "generated/rewriter.rs"]
mod rewriter;
#[path = "generated/terminal_kind.rs"]
mod terminal_kind;

pub use edge_label::EdgeLabel;
pub(crate) use lexical_context::{IsLexicalContext, LexicalContext, LexicalContextType};
pub use metaslang_cst::kinds::TerminalKindExtensions;
pub(crate) use metaslang_cst::kinds::{EdgeLabelExtensions, NonterminalKindExtensions};
pub use nonterminal_kind::NonterminalKind;
pub use rewriter::BaseRewriter;
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
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
pub enum KindTypes {}

impl metaslang_cst::kinds::KindTypes for KindTypes {
    type NonterminalKind = NonterminalKind;
    type TerminalKind = TerminalKind;
    type EdgeLabel = EdgeLabel;
}

/// The super type of terminal and nonterminal kinds.
pub type NodeKind = metaslang_cst::kinds::NodeKind<KindTypes>;

/// The identifier of a node in a tree.
pub type NodeId = metaslang_cst::nodes::NodeId;

/// The super type of all nodes in a tree.
pub type Node = metaslang_cst::nodes::Node<KindTypes>;
/// Represents a nonterminal node in the syntax tree.
/// These nodes can have child nodes and represent language constructs.
pub type NonterminalNode = metaslang_cst::nodes::NonterminalNode<KindTypes>;
/// Represents a terminal node in the syntax tree.
/// These are leaf nodes that represent actual tokens from the source code.
pub type TerminalNode = metaslang_cst::nodes::TerminalNode<KindTypes>;
/// Represents a connection between nodes in the syntax tree.
pub type Edge = metaslang_cst::nodes::Edge<KindTypes>;

/// A cursor that can traverse a CST.
///
/// Nodes are visited in a DFS pre-order traversal.
pub type Cursor = metaslang_cst::cursor::Cursor<KindTypes>;
/// Iterator over all the remaining nodes in the current tree, moving in pre-order traversal, until the tree is completed.
pub type CursorIterator = metaslang_cst::cursor::CursorIterator<KindTypes>;
/// Iterator over all ancestors of the current node, starting with the immediate parent, and moving upwards, ending with the root node.
pub type AncestorsIterator = metaslang_cst::cursor::AncestorsIterator<KindTypes>;

/// The declarative `Query` API is a convenient alternative to the [`Cursor`][`metaslang_cst::cursor::Cursor`]
/// API for navigating the CST.
///
/// The query engine performs pattern matching, and the execution semantics are closer to
/// unification than to regular expression matching. A query returns all possible matches,
/// not just the longest/shortest/first/last match.
///
/// Please refer to [our documentation](https://nomicfoundation.github.io/slang/latest/user-guide/06-query-language/01-query-syntax/)
/// for detailed information about the query syntax and how to use queries to find matches.
pub type Query = metaslang_cst::query::Query<KindTypes>;

pub use metaslang_cst::query::QueryError;
/// Represents a match found by executing queries on a cursor.
pub type QueryMatch = metaslang_cst::query::QueryMatch<KindTypes>;
/// Iterator over query matches in the syntax tree.
pub type QueryMatchIterator = metaslang_cst::query::QueryMatchIterator<KindTypes>;

/// Representation of a position within text.
pub type TextIndex = metaslang_cst::text_index::TextIndex;
/// A [`Range`][`core::ops::Range`] of [`TextIndex`].
pub type TextRange = metaslang_cst::text_index::TextRange;

pub use metaslang_cst::text_index::TextRangeExtensions;
