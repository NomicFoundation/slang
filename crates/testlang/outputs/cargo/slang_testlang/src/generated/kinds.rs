// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[cfg(feature = "slang_napi_interfaces")]
use napi_derive::napi;

#[derive(
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
)]
#[cfg_attr(feature = "slang_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "kinds"))]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
pub enum RuleKind {
    AdditionExpression,
    Expression,
    Literal,
    MemberAccessExpression,
    NegationExpression,
    SeparatedIdentifiers,
    SourceUnit,
    SourceUnitMember,
    SourceUnitMembers,
    Tree,
    TreeNode,
    TreeNodeChild,
    TreeNodeChildren,
}

impl metaslang_cst::NonTerminalKind for RuleKind {}

#[derive(
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
)]
#[strum(serialize_all = "snake_case")]
#[cfg_attr(feature = "slang_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "kinds"))]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
pub enum NodeLabel {
    // Built-in labels
    Item,
    Variant,
    Separator,
    Operand,
    LeftOperand,
    RightOperand,
    LeadingTrivia,
    TrailingTrivia,
    // Generated
    CloseBracket,
    Keyword,
    Member,
    Members,
    Name,
    Node,
    OpenBracket,
    Operator,
    Period,
    Semicolon,
}

impl metaslang_cst::EdgeKind for NodeLabel {}

#[derive(
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
)]
#[cfg_attr(feature = "slang_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "kinds"))]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
pub enum TokenKind {
    SKIPPED,
    Bang,
    CloseBracket,
    DelimitedIdentifier,
    EndOfLine,
    Identifier,
    MultiLineComment,
    OpenBracket,
    Period,
    Plus,
    Semicolon,
    SingleLineComment,
    StringLiteral,
    TreeKeyword,
    Whitespace,
}

impl metaslang_cst::TerminalKind for TokenKind {
    fn is_trivia(&self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match self {
            Self::EndOfLine => true,
            Self::MultiLineComment => true,
            Self::SingleLineComment => true,
            Self::Whitespace => true,
            _ => false,
        }
    }
}

/// The lexical context of the scanner.
#[derive(strum_macros::FromRepr, Clone, Copy)]
pub(crate) enum LexicalContext {
    Default,
    Tree,
}

/// Marker trait for type-level [`LexicalContext`] variants.
pub(crate) trait IsLexicalContext {
    /// Returns a run-time [`LexicalContext`] value.
    fn value() -> LexicalContext;
}

#[allow(non_snake_case)]
pub(crate) mod LexicalContextType {
    use super::{IsLexicalContext, LexicalContext};
    pub struct Default {}
    impl IsLexicalContext for Default {
        fn value() -> LexicalContext {
            LexicalContext::Default
        }
    }
    pub struct Tree {}
    impl IsLexicalContext for Tree {
        fn value() -> LexicalContext {
            LexicalContext::Tree
        }
    }
}
