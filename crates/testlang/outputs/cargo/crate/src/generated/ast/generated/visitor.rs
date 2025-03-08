// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::rc::Rc;

#[allow(clippy::wildcard_imports)]
use super::nodes::*;
use crate::cst::TerminalNode;

pub trait Visitor {
    fn visit_source_unit(&self, _target: &SourceUnit) {}
    fn visit_tree(&self, _target: &Tree) {}
    fn visit_tree_node(&self, _target: &TreeNode) {}
    fn visit_addition_expression(&self, _target: &AdditionExpression) {}
    fn visit_negation_expression(&self, _target: &NegationExpression) {}
    fn visit_member_access_expression(&self, _target: &MemberAccessExpression) {}
}

//
// Sequences:
//

impl SourceUnitStruct {
    pub fn accept(self: &Rc<Self>, visitor: &dyn Visitor) {
        accept_source_unit_members(&self.members, visitor);
        visitor.visit_source_unit(self);
    }
}

impl TreeStruct {
    pub fn accept(self: &Rc<Self>, visitor: &dyn Visitor) {
        self.node.accept(visitor);
        visitor.visit_tree(self);
    }
}

impl TreeNodeStruct {
    pub fn accept(self: &Rc<Self>, visitor: &dyn Visitor) {
        accept_tree_node_children(&self.members, visitor);
        visitor.visit_tree_node(self);
    }
}

impl AdditionExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &dyn Visitor) {
        self.left_operand.accept(visitor);

        self.right_operand.accept(visitor);
        visitor.visit_addition_expression(self);
    }
}

impl NegationExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &dyn Visitor) {
        self.operand.accept(visitor);
        visitor.visit_negation_expression(self);
    }
}

impl MemberAccessExpressionStruct {
    pub fn accept(self: &Rc<Self>, visitor: &dyn Visitor) {
        self.operand.accept(visitor);
        visitor.visit_member_access_expression(self);
    }
}

//
// Choices:
//

impl SourceUnitMember {
    pub fn accept(&self, visitor: &dyn Visitor) {
        match self {
            Self::Tree(ref tree) => {
                tree.accept(visitor);
            }
            Self::Expression(ref expression) => {
                expression.accept(visitor);
            }
            Self::SeparatedIdentifiers(ref separated_identifiers) => {
                accept_separated_identifiers(separated_identifiers, visitor);
            }
            Self::Literal(ref literal) => {
                literal.accept(visitor);
            }
        }
    }
}

impl TreeNodeChild {
    pub fn accept(&self, visitor: &dyn Visitor) {
        match self {
            Self::TreeNode(ref tree_node) => {
                tree_node.accept(visitor);
            }
            Self::TerminalNode(_) => {}
        }
    }
}

impl Expression {
    pub fn accept(&self, visitor: &dyn Visitor) {
        match self {
            Self::AdditionExpression(ref addition_expression) => {
                addition_expression.accept(visitor);
            }
            Self::NegationExpression(ref negation_expression) => {
                negation_expression.accept(visitor);
            }
            Self::MemberAccessExpression(ref member_access_expression) => {
                member_access_expression.accept(visitor);
            }
            Self::TerminalNode(_) => {}
        }
    }
}

impl Literal {
    pub fn accept(&self, _visitor: &dyn Visitor) {}
}

//
// Repeated:
//

#[inline]
fn accept_source_unit_members(items: &[SourceUnitMember], visitor: &dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

#[inline]
fn accept_tree_node_children(items: &[TreeNodeChild], visitor: &dyn Visitor) {
    for item in items {
        item.accept(visitor);
    }
}

//
// Separated:
//

#[inline]
fn accept_separated_identifiers(_items: &[Rc<TerminalNode>], _visitor: &dyn Visitor) {}
