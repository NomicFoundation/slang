mod generated;

pub use crate::legacy::generated::{
    cst::{Node, RuleNode, TokenNode},
    cursor::Cursor,
    text_index::{TextIndex, TextRange, TextRangeExtensions},
    visitor::{Visitor, VisitorEntryResponse, VisitorExitResponse},
};

pub use generated::{production_kind::ProductionKind, rule_kind::RuleKind, token_kind::TokenKind};
