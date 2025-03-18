// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_lines)]

use std::rc::Rc;

#[allow(clippy::wildcard_imports)]
use super::nodes::*;

pub trait Mutator {
    //
    // Sequences:
    //

    fn mutate_source_unit(&mut self, source: &SourceUnit) -> SourceUnit {
        let members = self.mutate_source_unit_members(&source.members);

        Rc::new(SourceUnitStruct {
            cursor: source.cursor.clone(),
            members,
        })
    }

    fn mutate_tree(&mut self, source: &Tree) -> Tree {
        let name = source.name.as_ref().map(Rc::clone);
        let node = self.mutate_tree_node(&source.node);

        Rc::new(TreeStruct {
            cursor: source.cursor.clone(),
            name,
            node,
        })
    }

    fn mutate_tree_node(&mut self, source: &TreeNode) -> TreeNode {
        let members = self.mutate_tree_node_children(&source.members);

        Rc::new(TreeNodeStruct {
            cursor: source.cursor.clone(),
            members,
        })
    }

    fn mutate_addition_expression(&mut self, source: &AdditionExpression) -> AdditionExpression {
        let left_operand = self.mutate_expression(&source.left_operand);
        let right_operand = self.mutate_expression(&source.right_operand);

        Rc::new(AdditionExpressionStruct {
            cursor: source.cursor.clone(),
            left_operand,
            right_operand,
        })
    }

    fn mutate_negation_expression(&mut self, source: &NegationExpression) -> NegationExpression {
        let operand = self.mutate_expression(&source.operand);

        Rc::new(NegationExpressionStruct {
            cursor: source.cursor.clone(),
            operand,
        })
    }

    fn mutate_member_access_expression(
        &mut self,
        source: &MemberAccessExpression,
    ) -> MemberAccessExpression {
        let operand = self.mutate_expression(&source.operand);
        let member = Rc::clone(&source.member);

        Rc::new(MemberAccessExpressionStruct {
            cursor: source.cursor.clone(),
            operand,
            member,
        })
    }

    //
    // Choices:
    //

    fn mutate_source_unit_member(&mut self, source: &SourceUnitMember) -> SourceUnitMember {
        match source {
            SourceUnitMember::Tree(ref tree) => SourceUnitMember::Tree(self.mutate_tree(tree)),
            SourceUnitMember::Expression(ref expression) => {
                SourceUnitMember::Expression(self.mutate_expression(expression))
            }
            SourceUnitMember::SeparatedIdentifiers(ref separated_identifiers) => {
                SourceUnitMember::SeparatedIdentifiers(
                    self.mutate_separated_identifiers(separated_identifiers),
                )
            }
            SourceUnitMember::Literal(ref literal) => {
                SourceUnitMember::Literal(self.mutate_literal(literal))
            }
        }
    }

    fn mutate_tree_node_child(&mut self, source: &TreeNodeChild) -> TreeNodeChild {
        match source {
            TreeNodeChild::TreeNode(ref tree_node) => {
                TreeNodeChild::TreeNode(self.mutate_tree_node(tree_node))
            }
            TreeNodeChild::DelimitedIdentifier(node) => {
                TreeNodeChild::DelimitedIdentifier(Rc::clone(node))
            }
        }
    }

    fn mutate_expression(&mut self, source: &Expression) -> Expression {
        match source {
            Expression::AdditionExpression(ref addition_expression) => {
                Expression::AdditionExpression(self.mutate_addition_expression(addition_expression))
            }
            Expression::NegationExpression(ref negation_expression) => {
                Expression::NegationExpression(self.mutate_negation_expression(negation_expression))
            }
            Expression::MemberAccessExpression(ref member_access_expression) => {
                Expression::MemberAccessExpression(
                    self.mutate_member_access_expression(member_access_expression),
                )
            }
            Expression::StringLiteral(node) => Expression::StringLiteral(Rc::clone(node)),
            Expression::Identifier(node) => Expression::Identifier(Rc::clone(node)),
        }
    }

    fn mutate_literal(&mut self, source: &Literal) -> Literal {
        match source {
            Literal::StringLiteral(node) => Literal::StringLiteral(Rc::clone(node)),
        }
    }

    //
    // Repeated:
    //

    fn mutate_source_unit_members(&mut self, source: &SourceUnitMembers) -> SourceUnitMembers {
        source
            .iter()
            .map(|item| self.mutate_source_unit_member(item))
            .collect()
    }

    fn mutate_tree_node_children(&mut self, source: &TreeNodeChildren) -> TreeNodeChildren {
        source
            .iter()
            .map(|item| self.mutate_tree_node_child(item))
            .collect()
    }

    //
    // Separated:
    //

    fn mutate_separated_identifiers(
        &mut self,
        source: &SeparatedIdentifiers,
    ) -> SeparatedIdentifiers {
        source.iter().map(Rc::clone).collect()
    }
}
