// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[allow(clippy::doc_markdown)]
#[allow(clippy::doc_link_with_quotes)]
#[repr(u8)]
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
    strum_macros::IntoStaticStr,
    Clone,
    Copy,
)]
pub enum NonterminalKind {
    /// (* Left-associative binary operator *)
    /// AdditionExpression = (* left_operand: *) Expression
    ///                      (* operator: *) PLUS
    ///                      (* right_operand: *) Expression;
    AdditionExpression,
    /// Expression = (* variant: *) AdditionExpression
    ///            | (* variant: *) NegationExpression
    ///            | (* variant: *) MemberAccessExpression
    ///            | (* variant: *) STRING_LITERAL
    ///            | (* variant: *) IDENTIFIER;
    Expression,
    /// Literal = (* variant: *) STRING_LITERAL;
    Literal,
    /// (* Postfix unary operator *)
    /// MemberAccessExpression = (* operand: *) Expression
    ///                          (* period: *) PERIOD
    ///                          (* member: *) IDENTIFIER;
    MemberAccessExpression,
    /// (* Prefix unary operator *)
    /// NegationExpression = (* operator: *) BANG
    ///                      (* operand: *) Expression;
    NegationExpression,
    /// (* Introduced in 1.0.0 *)
    /// SeparatedIdentifiers = (* item: *) IDENTIFIER ((* separator: *) PERIOD (* item: *) IDENTIFIER)*;
    SeparatedIdentifiers,
    /// SourceUnit = (* members: *) SourceUnitMembers;
    SourceUnit,
    /// SourceUnitMember = (* variant: *) Tree
    ///                  | (* variant: *) Expression
    ///                  | (* variant: *) SeparatedIdentifiers
    ///                  | (* variant: *) Literal;
    SourceUnitMember,
    /// SourceUnitMembers = (* item: *) SourceUnitMember+;
    SourceUnitMembers,
    /// Tree = (* keyword: *) TREE_KEYWORD
    ///        (* name: *) IDENTIFIER?
    ///        (* node: *) TreeNode
    ///        (* semicolon: *) SEMICOLON;
    Tree,
    /// TreeNode = (* open_bracket: *) OPEN_BRACKET
    ///            (* members: *) TreeNodeChildren
    ///            (* close_bracket: *) CLOSE_BRACKET;
    TreeNode,
    /// TreeNodeChild = (* variant: *) TreeNode
    ///               | (* variant: *) DELIMITED_IDENTIFIER;
    TreeNodeChild,
    /// TreeNodeChildren = (* item: *) TreeNodeChild+;
    TreeNodeChildren,
}

impl crate::cst::NonterminalKindExtensions for NonterminalKind {}
