// Generated on 2025-01-30T14:52:17.627Z
use super::model::*;

pub trait InPlaceTransformer {
    #[allow(unused_mut)]
    fn transform_terminal_kind(self: &mut Self, mut node: TerminalKind) -> TerminalKind {
        node
    }

    #[allow(unused_mut)]
    fn transform_terminal_node(self: &mut Self, mut node: TerminalNode) -> TerminalNode {
        node
    }

    #[allow(unused_mut)]
    fn transform_source_unit(self: &mut Self, mut node: Box<SourceUnit>) -> Box<SourceUnit> {
        node.members = self.transform_source_unit_members(node.members);
        node
    }

    #[allow(unused_mut)]
    fn transform_tree(self: &mut Self, mut node: Box<Tree>) -> Box<Tree> {
        node.keyword = self.transform_terminal_node(node.keyword);
        node.name = node.name.map(|value| self.transform_terminal_node(value));
        node.node = self.transform_tree_node(node.node);
        node.semicolon = self.transform_terminal_node(node.semicolon);
        node
    }

    #[allow(unused_mut)]
    fn transform_tree_node(self: &mut Self, mut node: Box<TreeNode>) -> Box<TreeNode> {
        node.open_bracket = self.transform_terminal_node(node.open_bracket);
        node.members = self.transform_tree_node_children(node.members);
        node.close_bracket = self.transform_terminal_node(node.close_bracket);
        node
    }

    #[allow(unused_mut)]
    fn transform_addition_expression(
        self: &mut Self,
        mut node: Box<AdditionExpression>,
    ) -> Box<AdditionExpression> {
        node.left_operand = self.transform_expression(node.left_operand);
        node.operator = self.transform_terminal_node(node.operator);
        node.right_operand = self.transform_expression(node.right_operand);
        node
    }

    #[allow(unused_mut)]
    fn transform_negation_expression(
        self: &mut Self,
        mut node: Box<NegationExpression>,
    ) -> Box<NegationExpression> {
        node.operator = self.transform_terminal_node(node.operator);
        node.operand = self.transform_expression(node.operand);
        node
    }

    #[allow(unused_mut)]
    fn transform_member_access_expression(
        self: &mut Self,
        mut node: Box<MemberAccessExpression>,
    ) -> Box<MemberAccessExpression> {
        node.operand = self.transform_expression(node.operand);
        node.period = self.transform_terminal_node(node.period);
        node.member = self.transform_terminal_node(node.member);
        node
    }

    #[allow(unused_mut)]
    fn transform_source_unit_member(
        self: &mut Self,
        mut node: SourceUnitMember,
    ) -> SourceUnitMember {
        match node {
            SourceUnitMember::Tree(value) => SourceUnitMember::Tree(self.transform_tree(value)),
            SourceUnitMember::Expression(value) => {
                SourceUnitMember::Expression(self.transform_expression(value))
            }
            SourceUnitMember::SeparatedIdentifiers(value) => {
                SourceUnitMember::SeparatedIdentifiers(self.transform_separated_identifiers(value))
            }
            SourceUnitMember::Literal(value) => {
                SourceUnitMember::Literal(self.transform_literal(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_tree_node_child(self: &mut Self, mut node: TreeNodeChild) -> TreeNodeChild {
        match node {
            TreeNodeChild::TreeNode(value) => {
                TreeNodeChild::TreeNode(self.transform_tree_node(value))
            }
            TreeNodeChild::TerminalNode(value) => {
                TreeNodeChild::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_expression(self: &mut Self, mut node: Expression) -> Expression {
        match node {
            Expression::AdditionExpression(value) => {
                Expression::AdditionExpression(self.transform_addition_expression(value))
            }
            Expression::NegationExpression(value) => {
                Expression::NegationExpression(self.transform_negation_expression(value))
            }
            Expression::MemberAccessExpression(value) => {
                Expression::MemberAccessExpression(self.transform_member_access_expression(value))
            }
            Expression::TerminalNode(value) => {
                Expression::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_literal(self: &mut Self, mut node: Literal) -> Literal {
        match node {
            Literal::TerminalNode(value) => {
                Literal::TerminalNode(self.transform_terminal_node(value))
            }
        }
    }

    #[allow(unused_mut)]
    fn transform_source_unit_members(
        self: &mut Self,
        mut node: Box<SourceUnitMembers>,
    ) -> Box<SourceUnitMembers> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_source_unit_member(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_tree_node_children(
        self: &mut Self,
        mut node: Box<TreeNodeChildren>,
    ) -> Box<TreeNodeChildren> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_tree_node_child(key))
            .collect();
        node
    }

    #[allow(unused_mut)]
    fn transform_separated_identifiers(
        self: &mut Self,
        mut node: Box<SeparatedIdentifiers>,
    ) -> Box<SeparatedIdentifiers> {
        node.item = node
            .item
            .into_iter()
            .map(|key| self.transform_terminal_node(key))
            .collect();
        node
    }
}
