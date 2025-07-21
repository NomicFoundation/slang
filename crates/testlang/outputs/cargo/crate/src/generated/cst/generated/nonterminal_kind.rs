// This file is generated automatically by infrastructure scripts (crates/codegen/runner/src/main.rs:68:22). Please don't edit by hand.

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
    /// Represents a node with kind `AdditionExpression`, having the following structure:
    ///
    /// ```ebnf
    /// (* Left-associative binary operator *)
    /// AdditionExpression = (* left_operand: *) Expression
    ///                      (* operator: *) PLUS
    ///                      (* right_operand: *) Expression;
    /// ```
    AdditionExpression,
    /// Represents a node with kind `Expression`, having the following structure:
    ///
    /// ```ebnf
    /// Expression = (* variant: *) AdditionExpression
    ///            | (* variant: *) NegationExpression
    ///            | (* variant: *) MemberAccessExpression
    ///            | (* variant: *) STRING_LITERAL
    ///            | (* variant: *) IDENTIFIER;
    /// ```
    Expression,
    /// Represents a node with kind `Literal`, having the following structure:
    ///
    /// ```ebnf
    /// Literal = (* variant: *) STRING_LITERAL;
    /// ```
    Literal,
    /// Represents a node with kind `MemberAccessExpression`, having the following structure:
    ///
    /// ```ebnf
    /// (* Postfix unary operator *)
    /// MemberAccessExpression = (* operand: *) Expression
    ///                          (* period: *) PERIOD
    ///                          (* member: *) IDENTIFIER;
    /// ```
    MemberAccessExpression,
    /// Represents a node with kind `NegationExpression`, having the following structure:
    ///
    /// ```ebnf
    /// (* Prefix unary operator *)
    /// NegationExpression = (* operator: *) BANG
    ///                      (* operand: *) Expression;
    /// ```
    NegationExpression,
    /// Represents a node with kind `SeparatedIdentifiers`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 1.0.0 *)
    /// SeparatedIdentifiers = (* item: *) IDENTIFIER ((* separator: *) PERIOD (* item: *) IDENTIFIER)*;
    /// ```
    SeparatedIdentifiers,
    /// Represents a node with kind `SourceUnit`, having the following structure:
    ///
    /// ```ebnf
    /// SourceUnit = (* members: *) SourceUnitMembers;
    /// ```
    SourceUnit,
    /// Represents a node with kind `SourceUnitMember`, having the following structure:
    ///
    /// ```ebnf
    /// SourceUnitMember = (* variant: *) Tree
    ///                  | (* variant: *) Expression
    ///                  | (* variant: *) SeparatedIdentifiers
    ///                  | (* variant: *) Literal;
    /// ```
    SourceUnitMember,
    /// Represents a node with kind `SourceUnitMembers`, having the following structure:
    ///
    /// ```ebnf
    /// SourceUnitMembers = (* item: *) SourceUnitMember+;
    /// ```
    SourceUnitMembers,
    /// Represents a node with kind `Tree`, having the following structure:
    ///
    /// ```ebnf
    /// Tree = (* keyword: *) TREE_KEYWORD
    ///        (* name: *) IDENTIFIER?
    ///        (* node: *) TreeNode
    ///        (* semicolon: *) SEMICOLON;
    /// ```
    Tree,
    /// Represents a node with kind `TreeNode`, having the following structure:
    ///
    /// ```ebnf
    /// TreeNode = (* open_bracket: *) OPEN_BRACKET
    ///            (* members: *) TreeNodeChildren
    ///            (* close_bracket: *) CLOSE_BRACKET;
    /// ```
    TreeNode,
    /// Represents a node with kind `TreeNodeChild`, having the following structure:
    ///
    /// ```ebnf
    /// TreeNodeChild = (* variant: *) TreeNode
    ///               | (* variant: *) DELIMITED_IDENTIFIER;
    /// ```
    TreeNodeChild,
    /// Represents a node with kind `TreeNodeChildren`, having the following structure:
    ///
    /// ```ebnf
    /// TreeNodeChildren = (* item: *) TreeNodeChild+;
    /// ```
    TreeNodeChildren,
}

impl crate::cst::NonterminalKindExtensions for NonterminalKind {}
