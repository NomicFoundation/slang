// Generated on 2025-01-30T14:52:17.619Z
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

pub type TerminalNode = String;

pub type SourceUnitRef = Box<SourceUnit>;
pub struct SourceUnit {
    pub members: SourceUnitMembersRef,
}

pub type TreeRef = Box<Tree>;
pub struct Tree {
    pub keyword: TerminalNode,
    pub name: Option<TerminalNode>,
    pub node: TreeNodeRef,
    pub semicolon: TerminalNode,
}

pub type TreeNodeRef = Box<TreeNode>;
pub struct TreeNode {
    pub open_bracket: TerminalNode,
    pub members: TreeNodeChildrenRef,
    pub close_bracket: TerminalNode,
}

pub type AdditionExpressionRef = Box<AdditionExpression>;
pub struct AdditionExpression {
    pub left_operand: Expression,
    pub operator: TerminalNode,
    pub right_operand: Expression,
}

pub type NegationExpressionRef = Box<NegationExpression>;
pub struct NegationExpression {
    pub operator: TerminalNode,
    pub operand: Expression,
}

pub type MemberAccessExpressionRef = Box<MemberAccessExpression>;
pub struct MemberAccessExpression {
    pub operand: Expression,
    pub period: TerminalNode,
    pub member: TerminalNode,
}

pub enum SourceUnitMember {
    Tree(TreeRef),
    Expression(Expression),
    SeparatedIdentifiers(SeparatedIdentifiersRef),
    Literal(Literal),
}

pub enum TreeNodeChild {
    TreeNode(TreeNodeRef),
    TerminalNode(TerminalNode),
}

pub enum Expression {
    AdditionExpression(AdditionExpressionRef),
    NegationExpression(NegationExpressionRef),
    MemberAccessExpression(MemberAccessExpressionRef),
    TerminalNode(TerminalNode),
}

pub enum Literal {
    TerminalNode(TerminalNode),
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
    pub item: std::collections::HashSet<TerminalNode>,
}
