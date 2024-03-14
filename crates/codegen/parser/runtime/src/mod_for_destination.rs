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

mod treestuff {
    #[derive(Clone, Debug, PartialEq, Eq, serde::Serialize)] // But why?
    pub struct ModuleInputs;
    impl codegen_parser_treestuff::ModuleInputs for ModuleInputs {
        type NonTerminalKind = crate::kinds::RuleKind;
        type TerminalKind = crate::kinds::TokenKind;
        type LabelKind = crate::kinds::NodeLabel;
    }
}

pub mod cst {
    use codegen_parser_treestuff::cst;
    pub type Node = cst::Node<super::treestuff::ModuleInputs>;
    pub type RuleNode = cst::NonTerminalNode<super::treestuff::ModuleInputs>;
    pub type TokenNode = cst::TerminalNode<super::treestuff::ModuleInputs>;
    pub type LabeledNode = cst::LabeledNode<super::treestuff::ModuleInputs>;
}

pub mod cursor {
    use codegen_parser_treestuff::cursor;
    pub type Cursor = cursor::Cursor<super::treestuff::ModuleInputs>;
    pub type CursorWithLabels = cursor::CursorWithLabels<super::treestuff::ModuleInputs>;
}

pub mod query {
    use codegen_parser_treestuff::query;
    pub type Query = query::Query<super::treestuff::ModuleInputs>;
    pub type QueryResult = query::QueryResult<super::treestuff::ModuleInputs>;
    pub type QueryResultIterator = query::QueryResultIterator<super::treestuff::ModuleInputs>;
}

pub mod text_index {
    use codegen_parser_treestuff::text_index;
    pub use text_index::{TextIndex, TextRange, TextRangeExtensions};
}
