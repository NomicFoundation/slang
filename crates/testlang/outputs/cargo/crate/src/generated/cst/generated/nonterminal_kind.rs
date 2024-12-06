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
    /// ```ebnf
    /// (* Left-associative binary operator *)
    /// AdditionExpression = (* left_operand: *) Expression
    ///                      (* operator: *) PLUS
    ///                      (* right_operand: *) Expression;
    /// ```
    AdditionExpression,
    /// ```ebnf
    /// Expression = (* variant: *) AdditionExpression
    ///            | (* variant: *) NegationExpression
    ///            | (* variant: *) MemberAccessExpression
    ///            | (* variant: *) STRING_LITERAL
    ///            | (* variant: *) IDENTIFIER;
    /// ```
    Expression,
    /// ```ebnf
    /// Literal = (* variant: *) STRING_LITERAL;
    /// ```
    Literal,
    /// ```ebnf
    /// (* Postfix unary operator *)
    /// MemberAccessExpression = (* operand: *) Expression
    ///                          (* period: *) PERIOD
    ///                          (* member: *) IDENTIFIER;
    /// ```
    MemberAccessExpression,
    /// ```ebnf
    /// (* Prefix unary operator *)
    /// NegationExpression = (* operator: *) BANG
    ///                      (* operand: *) Expression;
    /// ```
    NegationExpression,
    /// ```ebnf
    /// (* Introduced in 1.0.0 *)
    /// SeparatedIdentifiers = (* item: *) IDENTIFIER ((* separator: *) PERIOD (* item: *) IDENTIFIER)*;
    /// ```
    SeparatedIdentifiers,
    /// ```ebnf
    /// SourceUnit = (* members: *) SourceUnitMembers;
    /// ```
    SourceUnit,
    /// ```ebnf
    /// SourceUnitMember = (* variant: *) Tree
    ///                  | (* variant: *) Expression
    ///                  | (* variant: *) SeparatedIdentifiers
    ///                  | (* variant: *) Literal;
    /// ```
    SourceUnitMember,
    /// ```ebnf
    /// SourceUnitMembers = (* item: *) SourceUnitMember+;
    /// ```
    SourceUnitMembers,
    /// ```ebnf
    /// Tree = (* keyword: *) TREE_KEYWORD
    ///        (* name: *) IDENTIFIER?
    ///        (* node: *) TreeNode
    ///        (* semicolon: *) SEMICOLON;
    /// ```
    Tree,
    /// ```ebnf
    /// TreeNode = (* open_bracket: *) OPEN_BRACKET
    ///            (* members: *) TreeNodeChildren
    ///            (* close_bracket: *) CLOSE_BRACKET;
    /// ```
    TreeNode,
    /// ```ebnf
    /// TreeNodeChild = (* variant: *) TreeNode
    ///               | (* variant: *) DELIMITED_IDENTIFIER;
    /// ```
    TreeNodeChild,
    /// ```ebnf
    /// TreeNodeChildren = (* item: *) TreeNodeChild+;
    /// ```
    TreeNodeChildren,
}

impl crate::cst::NonterminalKindExtensions for NonterminalKind {}
