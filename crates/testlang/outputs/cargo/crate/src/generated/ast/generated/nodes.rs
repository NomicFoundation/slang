// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::rc::Rc;
use std::vec::Vec;

use crate::cst::{Cursor, TerminalNode};

//
// Sequences:
//

/**
 * This node represents a `SourceUnit` nonterminal, with the following structure:
 *
 * ```ebnf
 * SourceUnit = (* members: *) SourceUnitMembers;
 * ```
 */
pub type SourceUnit = Rc<SourceUnitStruct>;

#[derive(Debug)]
pub struct SourceUnitStruct {
    pub cursor: Cursor,
    pub members: SourceUnitMembers,
}

/**
 * This node represents a `Tree` nonterminal, with the following structure:
 *
 * ```ebnf
 * Tree = (* keyword: *) TREE_KEYWORD
 *        (* name: *) IDENTIFIER?
 *        (* node: *) TreeNode
 *        (* semicolon: *) SEMICOLON;
 * ```
 */
pub type Tree = Rc<TreeStruct>;

#[derive(Debug)]
pub struct TreeStruct {
    pub cursor: Cursor,
    pub keyword: Rc<TerminalNode>,
    pub name: Option<Rc<TerminalNode>>,
    pub node: TreeNode,
    pub semicolon: Rc<TerminalNode>,
}

/**
 * This node represents a `TreeNode` nonterminal, with the following structure:
 *
 * ```ebnf
 * TreeNode = (* open_bracket: *) OPEN_BRACKET
 *            (* members: *) TreeNodeChildren
 *            (* close_bracket: *) CLOSE_BRACKET;
 * ```
 */
pub type TreeNode = Rc<TreeNodeStruct>;

#[derive(Debug)]
pub struct TreeNodeStruct {
    pub cursor: Cursor,
    pub open_bracket: Rc<TerminalNode>,
    pub members: TreeNodeChildren,
    pub close_bracket: Rc<TerminalNode>,
}

/**
 * This node represents a `AdditionExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Left-associative binary operator *)
 * AdditionExpression = (* left_operand: *) Expression
 *                      (* operator: *) PLUS
 *                      (* right_operand: *) Expression;
 * ```
 */
pub type AdditionExpression = Rc<AdditionExpressionStruct>;

#[derive(Debug)]
pub struct AdditionExpressionStruct {
    pub cursor: Cursor,
    pub left_operand: Expression,
    pub operator: Rc<TerminalNode>,
    pub right_operand: Expression,
}

/**
 * This node represents a `NegationExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Prefix unary operator *)
 * NegationExpression = (* operator: *) BANG
 *                      (* operand: *) Expression;
 * ```
 */
pub type NegationExpression = Rc<NegationExpressionStruct>;

#[derive(Debug)]
pub struct NegationExpressionStruct {
    pub cursor: Cursor,
    pub operator: Rc<TerminalNode>,
    pub operand: Expression,
}

/**
 * This node represents a `MemberAccessExpression` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Postfix unary operator *)
 * MemberAccessExpression = (* operand: *) Expression
 *                          (* period: *) PERIOD
 *                          (* member: *) IDENTIFIER;
 * ```
 */
pub type MemberAccessExpression = Rc<MemberAccessExpressionStruct>;

#[derive(Debug)]
pub struct MemberAccessExpressionStruct {
    pub cursor: Cursor,
    pub operand: Expression,
    pub period: Rc<TerminalNode>,
    pub member: Rc<TerminalNode>,
}

//
// Choices:
//

/**
 * This node represents a `SourceUnitMember` nonterminal, with the following structure:
 *
 * ```ebnf
 * SourceUnitMember = (* variant: *) Tree
 *                  | (* variant: *) Expression
 *                  | (* variant: *) SeparatedIdentifiers
 *                  | (* variant: *) Literal;
 * ```
 */

#[derive(Debug)]
pub enum SourceUnitMember {
    Tree(Tree),
    Expression(Expression),
    SeparatedIdentifiers(SeparatedIdentifiers),
    Literal(Literal),
}

/**
 * This node represents a `TreeNodeChild` nonterminal, with the following structure:
 *
 * ```ebnf
 * TreeNodeChild = (* variant: *) TreeNode
 *               | (* variant: *) DELIMITED_IDENTIFIER;
 * ```
 */

#[derive(Debug)]
pub enum TreeNodeChild {
    TreeNode(TreeNode),
    TerminalNode(Rc<TerminalNode>),
}

/**
 * This node represents a `Expression` nonterminal, with the following structure:
 *
 * ```ebnf
 * Expression = (* variant: *) AdditionExpression
 *            | (* variant: *) NegationExpression
 *            | (* variant: *) MemberAccessExpression
 *            | (* variant: *) STRING_LITERAL
 *            | (* variant: *) IDENTIFIER;
 * ```
 */

#[derive(Debug)]
pub enum Expression {
    AdditionExpression(AdditionExpression),
    NegationExpression(NegationExpression),
    MemberAccessExpression(MemberAccessExpression),
    TerminalNode(Rc<TerminalNode>),
}

/**
 * This node represents a `Literal` nonterminal, with the following structure:
 *
 * ```ebnf
 * Literal = (* variant: *) STRING_LITERAL;
 * ```
 */

pub type Literal = Rc<TerminalNode>;

//
// Repeated:
//

/**
 * This node represents a `SourceUnitMembers` nonterminal, with the following structure:
 *
 * ```ebnf
 * SourceUnitMembers = (* item: *) SourceUnitMember+;
 * ```
 */
pub type SourceUnitMembers = Vec<SourceUnitMember>;

/**
 * This node represents a `TreeNodeChildren` nonterminal, with the following structure:
 *
 * ```ebnf
 * TreeNodeChildren = (* item: *) TreeNodeChild+;
 * ```
 */
pub type TreeNodeChildren = Vec<TreeNodeChild>;

//
// Separated:
//

/**
 * This node represents a `SeparatedIdentifiers` nonterminal, with the following structure:
 *
 * ```ebnf
 * (* Introduced in 1.0.0 *)
 * SeparatedIdentifiers = (* item: *) IDENTIFIER ((* separator: *) PERIOD (* item: *) IDENTIFIER)*;
 * ```
 */
pub type SeparatedIdentifiers = Vec<Rc<TerminalNode>>;
