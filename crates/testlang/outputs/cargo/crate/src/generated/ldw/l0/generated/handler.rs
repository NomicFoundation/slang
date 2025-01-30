// Generated on 2025-01-30T14:52:17.622Z
use super::model::*;

pub trait Handler {
    #[allow(unused_variables)]
    fn handle_terminal_kind(self: &mut Self, node: &TerminalKind) {}

    #[allow(unused_variables)]
    fn handle_terminal_node(self: &mut Self, node: &TerminalNode) {}

    #[allow(unused_variables)]
    fn handle_source_unit(self: &mut Self, node: &Box<SourceUnit>) {}

    #[allow(unused_variables)]
    fn handle_tree(self: &mut Self, node: &Box<Tree>) {}

    #[allow(unused_variables)]
    fn handle_tree_node(self: &mut Self, node: &Box<TreeNode>) {}

    #[allow(unused_variables)]
    fn handle_addition_expression(self: &mut Self, node: &Box<AdditionExpression>) {}

    #[allow(unused_variables)]
    fn handle_negation_expression(self: &mut Self, node: &Box<NegationExpression>) {}

    #[allow(unused_variables)]
    fn handle_member_access_expression(self: &mut Self, node: &Box<MemberAccessExpression>) {}

    #[allow(unused_variables)]
    fn handle_source_unit_member(self: &mut Self, node: &SourceUnitMember) {
        match node {
            SourceUnitMember::Tree(value) => self.handle_tree(value),
            SourceUnitMember::Expression(value) => self.handle_expression(value),
            SourceUnitMember::SeparatedIdentifiers(value) => {
                self.handle_separated_identifiers(value)
            }
            SourceUnitMember::Literal(value) => self.handle_literal(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_tree_node_child(self: &mut Self, node: &TreeNodeChild) {
        match node {
            TreeNodeChild::TreeNode(value) => self.handle_tree_node(value),
            TreeNodeChild::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_expression(self: &mut Self, node: &Expression) {
        match node {
            Expression::AdditionExpression(value) => self.handle_addition_expression(value),
            Expression::NegationExpression(value) => self.handle_negation_expression(value),
            Expression::MemberAccessExpression(value) => {
                self.handle_member_access_expression(value)
            }
            Expression::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_literal(self: &mut Self, node: &Literal) {
        match node {
            Literal::TerminalNode(value) => self.handle_terminal_node(value),
        }
    }

    #[allow(unused_variables)]
    fn handle_source_unit_members(self: &mut Self, node: &Box<SourceUnitMembers>) {}

    #[allow(unused_variables)]
    fn handle_tree_node_children(self: &mut Self, node: &Box<TreeNodeChildren>) {}

    #[allow(unused_variables)]
    fn handle_separated_identifiers(self: &mut Self, node: &Box<SeparatedIdentifiers>) {}
}
