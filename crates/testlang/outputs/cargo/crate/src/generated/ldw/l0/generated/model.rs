// Generated on 2025-02-13T18:37:18.865Z
pub enum TerminalKind {
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

pub type TerminalNodeRef = Box<TerminalNode>;
pub struct TerminalNode {
    pub kind: TerminalKind,
    pub value: String,
}

pub type SourceUnitRef = Box<SourceUnit>;
pub struct SourceUnit {
    pub members: SourceUnitMembersRef,
}

pub type TreeRef = Box<Tree>;
pub struct Tree {
    pub keyword: TerminalNodeRef,
    pub name: Option<TerminalNodeRef>,
    pub node: TreeNodeRef,
    pub semicolon: TerminalNodeRef,
}

pub type TreeNodeRef = Box<TreeNode>;
pub struct TreeNode {
    pub open_bracket: TerminalNodeRef,
    pub members: TreeNodeChildrenRef,
    pub close_bracket: TerminalNodeRef,
}

pub type AdditionExpressionRef = Box<AdditionExpression>;
pub struct AdditionExpression {
    pub left_operand: Expression,
    pub operator: TerminalNodeRef,
    pub right_operand: Expression,
}

pub type NegationExpressionRef = Box<NegationExpression>;
pub struct NegationExpression {
    pub operator: TerminalNodeRef,
    pub operand: Expression,
}

pub type MemberAccessExpressionRef = Box<MemberAccessExpression>;
pub struct MemberAccessExpression {
    pub operand: Expression,
    pub period: TerminalNodeRef,
    pub member: TerminalNodeRef,
}

pub enum SourceUnitMember {
    Tree(TreeRef),
    Expression(Expression),
    SeparatedIdentifiers(SeparatedIdentifiersRef),
    Literal(Literal),
}

pub enum TreeNodeChild {
    TreeNode(TreeNodeRef),
    TerminalNode(TerminalNodeRef),
}

pub enum Expression {
    AdditionExpression(AdditionExpressionRef),
    NegationExpression(NegationExpressionRef),
    MemberAccessExpression(MemberAccessExpressionRef),
    TerminalNode(TerminalNodeRef),
}

pub enum Literal {
    TerminalNode(TerminalNodeRef),
}

pub type SourceUnitMembersRef = Box<SourceUnitMembers>;
pub struct SourceUnitMembers {
    pub item: std::collections::HashSet<SourceUnitMember>,
}

pub type TreeNodeChildrenRef = Box<TreeNodeChildren>;
pub struct TreeNodeChildren {
    pub item: std::collections::HashSet<TreeNodeChild>,
}

pub type SeparatedIdentifiersRef = Box<SeparatedIdentifiers>;
pub struct SeparatedIdentifiers {
    pub item: std::collections::HashSet<TerminalNodeRef>,
}
