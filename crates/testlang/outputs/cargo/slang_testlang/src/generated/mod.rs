// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[macro_use]
pub(crate) mod parser_support;
pub(crate) mod lexer;

pub mod diagnostic;
#[path = "generated/kinds.rs"]
pub mod kinds;
#[path = "generated/language.rs"]
pub mod language;
pub mod parse_error;
pub mod parse_output;

#[cfg(feature = "slang_napi_interfaces")]
pub mod napi_interface;

mod metaslang_cst {
    #[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)]
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
    pub enum KindTypes {}

    impl metaslang_cst::KindTypes for KindTypes {
        type NonterminalKind = crate::kinds::NonterminalKind;
        type TerminalKind = crate::kinds::TerminalKind;
        type EdgeLabel = crate::kinds::EdgeLabel;
    }
}

pub mod cst {
    use metaslang_cst::cst;

    use super::metaslang_cst::KindTypes;

    pub type Node = cst::Node<KindTypes>;
    pub type NonterminalNode = cst::NonterminalNode<KindTypes>;
    pub type TerminalNode = cst::TerminalNode<KindTypes>;
    pub type Edge = cst::Edge<KindTypes>;
}

pub mod cursor {
    use metaslang_cst::cursor;

    use super::metaslang_cst::KindTypes;

    pub type Cursor = cursor::Cursor<KindTypes>;
    pub type CursorWithEdges = cursor::CursorWithEdges<KindTypes>;
}

pub mod query {
    use metaslang_cst::query;

    use super::metaslang_cst::KindTypes;

    pub type Query = query::Query<KindTypes>;
    pub type QueryMatch = query::QueryMatch<KindTypes>;
    pub type QueryMatchIterator = query::QueryMatchIterator<KindTypes>;
}

pub mod text_index {
    use metaslang_cst::text_index;
    pub use text_index::{TextIndex, TextRange, TextRangeExtensions};
}
