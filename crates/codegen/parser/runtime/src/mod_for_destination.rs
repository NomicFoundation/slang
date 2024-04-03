#[macro_use]
pub mod parser_support;

pub mod kinds;
pub mod language;
pub(crate) mod lexer;
pub mod parse_error;
pub mod parse_output;
pub mod queries;

#[cfg(feature = "slang_napi_interfaces")]
pub mod napi_interface;

mod metaslang_cst {
    #[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)] // But why?
    pub struct KindTypes;
    impl metaslang_cst::KindTypes for KindTypes {
        type NonTerminalKind = crate::kinds::RuleKind;
        type TerminalKind = crate::kinds::TokenKind;
        type LabelKind = crate::kinds::NodeLabel;
    }
}

pub mod cst {
    use metaslang_cst::cst;
    pub type Node = cst::Node<super::metaslang_cst::KindTypes>;
    pub type RuleNode = cst::NonTerminalNode<super::metaslang_cst::KindTypes>;
    pub type TokenNode = cst::TerminalNode<super::metaslang_cst::KindTypes>;
    pub type LabeledNode = cst::LabeledNode<super::metaslang_cst::KindTypes>;
}

pub mod cursor {
    use metaslang_cst::cursor;
    pub type Cursor = cursor::Cursor<super::metaslang_cst::KindTypes>;
    pub type CursorWithLabels = cursor::CursorWithLabels<super::metaslang_cst::KindTypes>;
}

pub mod query {
    use metaslang_cst::query;
    pub type Query = query::Query<super::metaslang_cst::KindTypes>;
    pub type QueryResult = query::QueryResult<super::metaslang_cst::KindTypes>;
    pub type QueryResultIterator = query::QueryResultIterator<super::metaslang_cst::KindTypes>;
}

pub mod text_index {
    use metaslang_cst::text_index;
    pub use text_index::{TextIndex, TextRange, TextRangeExtensions};
}
