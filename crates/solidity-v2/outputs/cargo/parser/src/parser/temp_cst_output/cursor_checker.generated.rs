// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::marker::PhantomData;

use slang_solidity::cst::{Edge, EdgeLabel, Node, NodeKind, NonterminalKind};
use slang_solidity_v2_ast::ast::lexemes::LexemeKind;
use slang_solidity_v2_ast::ast::nodes::*;

use crate::temp_sourcify::Comparator;

#[derive(Clone, Debug)]
pub struct NodeCheckerError {
    pub err: String,
}

impl NodeCheckerError {
    fn new(err: String) -> NodeCheckerError {
        NodeCheckerError { err }
    }
}

pub trait NodeChecker {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError>;
}

/// Extract the first element that satisfies the predicate, and remove it from the vector.
fn extract_first<T>(v: &mut Vec<T>, finder: impl Fn(&T) -> bool) -> Option<T> {
    if let Some(idx) = v.iter().position(finder) {
        return Some(v.remove(idx));
    }
    None
}

/// Remove all trivia nodes from the vector.
fn remove_trivia(v: &mut Vec<Edge>) {
    v.retain(|edge| !edge.node.is_trivia());
    // Also remove separators
    v.retain(|edge| edge.label != EdgeLabel::Separator);
}

//
// Sequences:
//

impl<'arena> NodeChecker for AbicoderPragma<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::AbicoderPragma) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::AbicoderPragma,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // abicoder_keyword

        {
            let abicoder_keyword = &self.abicoder_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::AbicoderKeyword
            }) {
                let child_errors = abicoder_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected abicoder_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // version

        {
            let version = &self.version;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Version
            }) {
                let child_errors = version.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected version to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for AdditiveExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::AdditiveExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::AdditiveExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // left_operand

        {
            let left_operand = &self.left_operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::LeftOperand
            }) {
                let child_errors = left_operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected left_operand to be present in the CST, but it was not"
                )));
            }
        }

        // operator

        {
            let operator = &self.operator;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operator
            }) {
                let child_errors = operator.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operator to be present in the CST, but it was not"
                )));
            }
        }

        // right_operand

        {
            let right_operand = &self.right_operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::RightOperand
            }) {
                let child_errors = right_operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected right_operand to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for AddressType<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::AddressType) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::AddressType,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // address_keyword

        {
            let address_keyword = &self.address_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::AddressKeyword
            }) {
                let child_errors = address_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected address_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // payable_keyword
        if let Some(payable_keyword) = &self.payable_keyword {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::PayableKeyword
            }) {
                let child_errors = payable_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected payable_keyword to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::PayableKeyword
            }) {
                errors.push(NodeCheckerError::new(format!("Expected payable_keyword to not be present in the CST, but it was there: {:#?}", child)));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for AndExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::AndExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::AndExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // left_operand

        {
            let left_operand = &self.left_operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::LeftOperand
            }) {
                let child_errors = left_operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected left_operand to be present in the CST, but it was not"
                )));
            }
        }

        // operator

        {
            let operator = &self.operator;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operator
            }) {
                let child_errors = operator.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operator to be present in the CST, but it was not"
                )));
            }
        }

        // right_operand

        {
            let right_operand = &self.right_operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::RightOperand
            }) {
                let child_errors = right_operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected right_operand to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ArrayExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ArrayExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ArrayExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // open_bracket

        {
            let open_bracket = &self.open_bracket;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenBracket
            }) {
                let child_errors = open_bracket.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_bracket to be present in the CST, but it was not"
                )));
            }
        }

        // items

        {
            let items = &self.items;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Items
            }) {
                let child_errors = items.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected items to be present in the CST, but it was not"
                )));
            }
        }

        // close_bracket

        {
            let close_bracket = &self.close_bracket;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseBracket
            }) {
                let child_errors = close_bracket.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_bracket to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ArrayTypeName<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ArrayTypeName) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ArrayTypeName,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // operand

        {
            let operand = &self.operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operand
            }) {
                let child_errors = operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operand to be present in the CST, but it was not"
                )));
            }
        }

        // open_bracket

        {
            let open_bracket = &self.open_bracket;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenBracket
            }) {
                let child_errors = open_bracket.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_bracket to be present in the CST, but it was not"
                )));
            }
        }

        // index
        if let Some(index) = &self.index {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Index
            }) {
                let child_errors = index.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected index to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Index
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected index to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        // close_bracket

        {
            let close_bracket = &self.close_bracket;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseBracket
            }) {
                let child_errors = close_bracket.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_bracket to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for AssemblyFlagsDeclaration<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::AssemblyFlagsDeclaration) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::AssemblyFlagsDeclaration,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // open_paren

        {
            let open_paren = &self.open_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenParen
            }) {
                let child_errors = open_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_paren to be present in the CST, but it was not"
                )));
            }
        }

        // flags

        {
            let flags = &self.flags;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Flags
            }) {
                let child_errors = flags.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected flags to be present in the CST, but it was not"
                )));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseParen
            }) {
                let child_errors = close_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_paren to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for AssemblyStatement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::AssemblyStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::AssemblyStatement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // assembly_keyword

        {
            let assembly_keyword = &self.assembly_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::AssemblyKeyword
            }) {
                let child_errors = assembly_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected assembly_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // label
        if let Some(label) = &self.label {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Label
            }) {
                let child_errors = label.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected label to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Label
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected label to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        // flags
        if let Some(flags) = &self.flags {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Flags
            }) {
                let child_errors = flags.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected flags to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Flags
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected flags to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        // body

        {
            let body = &self.body;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Body)
            {
                let child_errors = body.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected body to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for AssignmentExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::AssignmentExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::AssignmentExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // left_operand

        {
            let left_operand = &self.left_operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::LeftOperand
            }) {
                let child_errors = left_operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected left_operand to be present in the CST, but it was not"
                )));
            }
        }

        // operator

        {
            let operator = &self.operator;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operator
            }) {
                let child_errors = operator.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operator to be present in the CST, but it was not"
                )));
            }
        }

        // right_operand

        {
            let right_operand = &self.right_operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::RightOperand
            }) {
                let child_errors = right_operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected right_operand to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for BitwiseAndExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::BitwiseAndExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::BitwiseAndExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // left_operand

        {
            let left_operand = &self.left_operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::LeftOperand
            }) {
                let child_errors = left_operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected left_operand to be present in the CST, but it was not"
                )));
            }
        }

        // operator

        {
            let operator = &self.operator;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operator
            }) {
                let child_errors = operator.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operator to be present in the CST, but it was not"
                )));
            }
        }

        // right_operand

        {
            let right_operand = &self.right_operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::RightOperand
            }) {
                let child_errors = right_operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected right_operand to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for BitwiseOrExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::BitwiseOrExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::BitwiseOrExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // left_operand

        {
            let left_operand = &self.left_operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::LeftOperand
            }) {
                let child_errors = left_operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected left_operand to be present in the CST, but it was not"
                )));
            }
        }

        // operator

        {
            let operator = &self.operator;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operator
            }) {
                let child_errors = operator.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operator to be present in the CST, but it was not"
                )));
            }
        }

        // right_operand

        {
            let right_operand = &self.right_operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::RightOperand
            }) {
                let child_errors = right_operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected right_operand to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for BitwiseXorExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::BitwiseXorExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::BitwiseXorExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // left_operand

        {
            let left_operand = &self.left_operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::LeftOperand
            }) {
                let child_errors = left_operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected left_operand to be present in the CST, but it was not"
                )));
            }
        }

        // operator

        {
            let operator = &self.operator;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operator
            }) {
                let child_errors = operator.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operator to be present in the CST, but it was not"
                )));
            }
        }

        // right_operand

        {
            let right_operand = &self.right_operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::RightOperand
            }) {
                let child_errors = right_operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected right_operand to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for Block<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::Block) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::Block,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // open_brace

        {
            let open_brace = &self.open_brace;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenBrace
            }) {
                let child_errors = open_brace.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_brace to be present in the CST, but it was not"
                )));
            }
        }

        // statements

        {
            let statements = &self.statements;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Statements
            }) {
                let child_errors = statements.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected statements to be present in the CST, but it was not"
                )));
            }
        }

        // close_brace

        {
            let close_brace = &self.close_brace;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseBrace
            }) {
                let child_errors = close_brace.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_brace to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for BreakStatement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::BreakStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::BreakStatement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // break_keyword

        {
            let break_keyword = &self.break_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::BreakKeyword
            }) {
                let child_errors = break_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected break_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Semicolon
            }) {
                let child_errors = semicolon.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected semicolon to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for CallOptionsExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::CallOptionsExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::CallOptionsExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // operand

        {
            let operand = &self.operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operand
            }) {
                let child_errors = operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operand to be present in the CST, but it was not"
                )));
            }
        }

        // open_brace

        {
            let open_brace = &self.open_brace;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenBrace
            }) {
                let child_errors = open_brace.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_brace to be present in the CST, but it was not"
                )));
            }
        }

        // options

        {
            let options = &self.options;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Options
            }) {
                let child_errors = options.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected options to be present in the CST, but it was not"
                )));
            }
        }

        // close_brace

        {
            let close_brace = &self.close_brace;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseBrace
            }) {
                let child_errors = close_brace.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_brace to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

// We skip NodeChecker for CallOptionsNew

impl<'arena> NodeChecker for CatchClause<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::CatchClause) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::CatchClause,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // catch_keyword

        {
            let catch_keyword = &self.catch_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CatchKeyword
            }) {
                let child_errors = catch_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected catch_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // error
        if let Some(error) = &self.error {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Error
            }) {
                let child_errors = error.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected error to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Error
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected error to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        // body

        {
            let body = &self.body;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Body)
            {
                let child_errors = body.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected body to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for CatchClauseError<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::CatchClauseError) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::CatchClauseError,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // name
        if let Some(name) = &self.name {
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Parameters
            }) {
                let child_errors = parameters.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected parameters to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ConditionalExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ConditionalExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ConditionalExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // operand

        {
            let operand = &self.operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operand
            }) {
                let child_errors = operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operand to be present in the CST, but it was not"
                )));
            }
        }

        // question_mark

        {
            let question_mark = &self.question_mark;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::QuestionMark
            }) {
                let child_errors = question_mark.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected question_mark to be present in the CST, but it was not"
                )));
            }
        }

        // true_expression

        {
            let true_expression = &self.true_expression;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::TrueExpression
            }) {
                let child_errors = true_expression.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected true_expression to be present in the CST, but it was not"
                )));
            }
        }

        // colon

        {
            let colon = &self.colon;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Colon
            }) {
                let child_errors = colon.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected colon to be present in the CST, but it was not"
                )));
            }
        }

        // false_expression

        {
            let false_expression = &self.false_expression;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::FalseExpression
            }) {
                let child_errors = false_expression.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected false_expression to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ConstantDefinition<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ConstantDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ConstantDefinition,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // type_name

        {
            let type_name = &self.type_name;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::TypeName
            }) {
                let child_errors = type_name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected type_name to be present in the CST, but it was not"
                )));
            }
        }

        // constant_keyword

        {
            let constant_keyword = &self.constant_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::ConstantKeyword
            }) {
                let child_errors = constant_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected constant_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // name

        {
            let name = &self.name;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        }

        // equal

        {
            let equal = &self.equal;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Equal
            }) {
                let child_errors = equal.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected equal to be present in the CST, but it was not"
                )));
            }
        }

        // value

        {
            let value = &self.value;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Value
            }) {
                let child_errors = value.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected value to be present in the CST, but it was not"
                )));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Semicolon
            }) {
                let child_errors = semicolon.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected semicolon to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ConstructorDefinition<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ConstructorDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ConstructorDefinition,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // constructor_keyword

        {
            let constructor_keyword = &self.constructor_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::ConstructorKeyword
            }) {
                let child_errors = constructor_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected constructor_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Parameters
            }) {
                let child_errors = parameters.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected parameters to be present in the CST, but it was not"
                )));
            }
        }

        // attributes

        {
            let attributes = &self.attributes;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Attributes
            }) {
                let child_errors = attributes.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected attributes to be present in the CST, but it was not"
                )));
            }
        }

        // body

        {
            let body = &self.body;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Body)
            {
                let child_errors = body.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected body to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ContinueStatement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ContinueStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ContinueStatement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // continue_keyword

        {
            let continue_keyword = &self.continue_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::ContinueKeyword
            }) {
                let child_errors = continue_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected continue_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Semicolon
            }) {
                let child_errors = semicolon.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected semicolon to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ContractDefinition<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ContractDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ContractDefinition,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // abstract_keyword
        if let Some(abstract_keyword) = &self.abstract_keyword {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::AbstractKeyword
            }) {
                let child_errors = abstract_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected abstract_keyword to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::AbstractKeyword
            }) {
                errors.push(NodeCheckerError::new(format!("Expected abstract_keyword to not be present in the CST, but it was there: {:#?}", child)));
            }
        }

        // contract_keyword

        {
            let contract_keyword = &self.contract_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::ContractKeyword
            }) {
                let child_errors = contract_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected contract_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // name

        {
            let name = &self.name;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        }

        // specifiers

        {
            let specifiers = &self.specifiers;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Specifiers
            }) {
                let child_errors = specifiers.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected specifiers to be present in the CST, but it was not"
                )));
            }
        }

        // open_brace

        {
            let open_brace = &self.open_brace;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenBrace
            }) {
                let child_errors = open_brace.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_brace to be present in the CST, but it was not"
                )));
            }
        }

        // members

        {
            let members = &self.members;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Members
            }) {
                let child_errors = members.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected members to be present in the CST, but it was not"
                )));
            }
        }

        // close_brace

        {
            let close_brace = &self.close_brace;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseBrace
            }) {
                let child_errors = close_brace.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_brace to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for DecimalNumberExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::DecimalNumberExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::DecimalNumberExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // literal

        {
            let literal = &self.literal;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Literal
            }) {
                let child_errors = literal.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected literal to be present in the CST, but it was not"
                )));
            }
        }

        // unit
        if let Some(unit) = &self.unit {
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Unit)
            {
                let child_errors = unit.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected unit to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Unit)
            {
                errors.push(NodeCheckerError::new(format!(
                    "Expected unit to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for DoWhileStatement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::DoWhileStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::DoWhileStatement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // do_keyword

        {
            let do_keyword = &self.do_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::DoKeyword
            }) {
                let child_errors = do_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected do_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // body

        {
            let body = &self.body;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Body)
            {
                let child_errors = body.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected body to be present in the CST, but it was not"
                )));
            }
        }

        // while_keyword

        {
            let while_keyword = &self.while_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::WhileKeyword
            }) {
                let child_errors = while_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected while_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // open_paren

        {
            let open_paren = &self.open_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenParen
            }) {
                let child_errors = open_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_paren to be present in the CST, but it was not"
                )));
            }
        }

        // condition

        {
            let condition = &self.condition;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Condition
            }) {
                let child_errors = condition.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected condition to be present in the CST, but it was not"
                )));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseParen
            }) {
                let child_errors = close_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_paren to be present in the CST, but it was not"
                )));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Semicolon
            }) {
                let child_errors = semicolon.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected semicolon to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ElseBranch<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ElseBranch) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ElseBranch,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // else_keyword

        {
            let else_keyword = &self.else_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::ElseKeyword
            }) {
                let child_errors = else_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected else_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // body

        {
            let body = &self.body;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Body)
            {
                let child_errors = body.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected body to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for EmitStatement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::EmitStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::EmitStatement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // emit_keyword

        {
            let emit_keyword = &self.emit_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::EmitKeyword
            }) {
                let child_errors = emit_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected emit_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // event

        {
            let event = &self.event;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Event
            }) {
                let child_errors = event.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected event to be present in the CST, but it was not"
                )));
            }
        }

        // arguments

        {
            let arguments = &self.arguments;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Arguments
            }) {
                let child_errors = arguments.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected arguments to be present in the CST, but it was not"
                )));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Semicolon
            }) {
                let child_errors = semicolon.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected semicolon to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for EnumDefinition<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::EnumDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::EnumDefinition,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // enum_keyword

        {
            let enum_keyword = &self.enum_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::EnumKeyword
            }) {
                let child_errors = enum_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected enum_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // name

        {
            let name = &self.name;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        }

        // open_brace

        {
            let open_brace = &self.open_brace;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenBrace
            }) {
                let child_errors = open_brace.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_brace to be present in the CST, but it was not"
                )));
            }
        }

        // members

        {
            let members = &self.members;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Members
            }) {
                let child_errors = members.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected members to be present in the CST, but it was not"
                )));
            }
        }

        // close_brace

        {
            let close_brace = &self.close_brace;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseBrace
            }) {
                let child_errors = close_brace.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_brace to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for EqualityExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::EqualityExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::EqualityExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // left_operand

        {
            let left_operand = &self.left_operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::LeftOperand
            }) {
                let child_errors = left_operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected left_operand to be present in the CST, but it was not"
                )));
            }
        }

        // operator

        {
            let operator = &self.operator;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operator
            }) {
                let child_errors = operator.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operator to be present in the CST, but it was not"
                )));
            }
        }

        // right_operand

        {
            let right_operand = &self.right_operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::RightOperand
            }) {
                let child_errors = right_operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected right_operand to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ErrorDefinition<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ErrorDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ErrorDefinition,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // error_keyword

        {
            let error_keyword = &self.error_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::ErrorKeyword
            }) {
                let child_errors = error_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected error_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // name

        {
            let name = &self.name;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        }

        // members

        {
            let members = &self.members;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Members
            }) {
                let child_errors = members.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected members to be present in the CST, but it was not"
                )));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Semicolon
            }) {
                let child_errors = semicolon.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected semicolon to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ErrorParameter<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ErrorParameter) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ErrorParameter,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // type_name

        {
            let type_name = &self.type_name;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::TypeName
            }) {
                let child_errors = type_name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected type_name to be present in the CST, but it was not"
                )));
            }
        }

        // name
        if let Some(name) = &self.name {
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ErrorParametersDeclaration<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ErrorParametersDeclaration) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ErrorParametersDeclaration,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // open_paren

        {
            let open_paren = &self.open_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenParen
            }) {
                let child_errors = open_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_paren to be present in the CST, but it was not"
                )));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Parameters
            }) {
                let child_errors = parameters.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected parameters to be present in the CST, but it was not"
                )));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseParen
            }) {
                let child_errors = close_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_paren to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for EventDefinition<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::EventDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::EventDefinition,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // event_keyword

        {
            let event_keyword = &self.event_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::EventKeyword
            }) {
                let child_errors = event_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected event_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // name

        {
            let name = &self.name;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Parameters
            }) {
                let child_errors = parameters.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected parameters to be present in the CST, but it was not"
                )));
            }
        }

        // anonymous_keyword
        if let Some(anonymous_keyword) = &self.anonymous_keyword {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::AnonymousKeyword
            }) {
                let child_errors = anonymous_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected anonymous_keyword to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::AnonymousKeyword
            }) {
                errors.push(NodeCheckerError::new(format!("Expected anonymous_keyword to not be present in the CST, but it was there: {:#?}", child)));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Semicolon
            }) {
                let child_errors = semicolon.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected semicolon to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for EventParameter<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::EventParameter) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::EventParameter,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // type_name

        {
            let type_name = &self.type_name;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::TypeName
            }) {
                let child_errors = type_name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected type_name to be present in the CST, but it was not"
                )));
            }
        }

        // indexed_keyword
        if let Some(indexed_keyword) = &self.indexed_keyword {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::IndexedKeyword
            }) {
                let child_errors = indexed_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected indexed_keyword to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::IndexedKeyword
            }) {
                errors.push(NodeCheckerError::new(format!("Expected indexed_keyword to not be present in the CST, but it was there: {:#?}", child)));
            }
        }

        // name
        if let Some(name) = &self.name {
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for EventParametersDeclaration<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::EventParametersDeclaration) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::EventParametersDeclaration,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // open_paren

        {
            let open_paren = &self.open_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenParen
            }) {
                let child_errors = open_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_paren to be present in the CST, but it was not"
                )));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Parameters
            }) {
                let child_errors = parameters.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected parameters to be present in the CST, but it was not"
                )));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseParen
            }) {
                let child_errors = close_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_paren to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ExperimentalPragma<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ExperimentalPragma) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ExperimentalPragma,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // experimental_keyword

        {
            let experimental_keyword = &self.experimental_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::ExperimentalKeyword
            }) {
                let child_errors = experimental_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected experimental_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // feature

        {
            let feature = &self.feature;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Feature
            }) {
                let child_errors = feature.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected feature to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ExponentiationExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ExponentiationExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ExponentiationExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // left_operand

        {
            let left_operand = &self.left_operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::LeftOperand
            }) {
                let child_errors = left_operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected left_operand to be present in the CST, but it was not"
                )));
            }
        }

        // operator

        {
            let operator = &self.operator;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operator
            }) {
                let child_errors = operator.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operator to be present in the CST, but it was not"
                )));
            }
        }

        // right_operand

        {
            let right_operand = &self.right_operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::RightOperand
            }) {
                let child_errors = right_operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected right_operand to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ExpressionStatement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ExpressionStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ExpressionStatement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // expression

        {
            let expression = &self.expression;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Expression
            }) {
                let child_errors = expression.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected expression to be present in the CST, but it was not"
                )));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Semicolon
            }) {
                let child_errors = semicolon.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected semicolon to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for FallbackFunctionDefinition<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FallbackFunctionDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::FallbackFunctionDefinition,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // fallback_keyword

        {
            let fallback_keyword = &self.fallback_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::FallbackKeyword
            }) {
                let child_errors = fallback_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected fallback_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Parameters
            }) {
                let child_errors = parameters.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected parameters to be present in the CST, but it was not"
                )));
            }
        }

        // attributes

        {
            let attributes = &self.attributes;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Attributes
            }) {
                let child_errors = attributes.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected attributes to be present in the CST, but it was not"
                )));
            }
        }

        // returns
        if let Some(returns) = &self.returns {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Returns
            }) {
                let child_errors = returns.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected returns to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Returns
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected returns to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        // body

        {
            let body = &self.body;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Body)
            {
                let child_errors = body.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected body to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ForStatement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ForStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ForStatement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // for_keyword

        {
            let for_keyword = &self.for_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::ForKeyword
            }) {
                let child_errors = for_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected for_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // open_paren

        {
            let open_paren = &self.open_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenParen
            }) {
                let child_errors = open_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_paren to be present in the CST, but it was not"
                )));
            }
        }

        // initialization

        {
            let initialization = &self.initialization;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Initialization
            }) {
                let child_errors = initialization.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected initialization to be present in the CST, but it was not"
                )));
            }
        }

        // condition

        {
            let condition = &self.condition;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Condition
            }) {
                let child_errors = condition.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected condition to be present in the CST, but it was not"
                )));
            }
        }

        // iterator
        if let Some(iterator) = &self.iterator {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Iterator
            }) {
                let child_errors = iterator.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected iterator to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Iterator
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected iterator to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseParen
            }) {
                let child_errors = close_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_paren to be present in the CST, but it was not"
                )));
            }
        }

        // body

        {
            let body = &self.body;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Body)
            {
                let child_errors = body.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected body to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for FunctionCallExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FunctionCallExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::FunctionCallExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // operand

        {
            let operand = &self.operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operand
            }) {
                let child_errors = operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operand to be present in the CST, but it was not"
                )));
            }
        }

        // arguments

        {
            let arguments = &self.arguments;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Arguments
            }) {
                let child_errors = arguments.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected arguments to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for FunctionDefinition<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FunctionDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::FunctionDefinition,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // function_keyword

        {
            let function_keyword = &self.function_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::FunctionKeyword
            }) {
                let child_errors = function_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected function_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // name

        {
            let name = &self.name;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Parameters
            }) {
                let child_errors = parameters.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected parameters to be present in the CST, but it was not"
                )));
            }
        }

        // attributes

        {
            let attributes = &self.attributes;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Attributes
            }) {
                let child_errors = attributes.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected attributes to be present in the CST, but it was not"
                )));
            }
        }

        // returns
        if let Some(returns) = &self.returns {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Returns
            }) {
                let child_errors = returns.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected returns to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Returns
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected returns to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        // body

        {
            let body = &self.body;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Body)
            {
                let child_errors = body.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected body to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for FunctionType<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FunctionType) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::FunctionType,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // function_keyword

        {
            let function_keyword = &self.function_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::FunctionKeyword
            }) {
                let child_errors = function_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected function_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Parameters
            }) {
                let child_errors = parameters.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected parameters to be present in the CST, but it was not"
                )));
            }
        }

        // attributes

        {
            let attributes = &self.attributes;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Attributes
            }) {
                let child_errors = attributes.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected attributes to be present in the CST, but it was not"
                )));
            }
        }

        // returns
        if let Some(returns) = &self.returns {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Returns
            }) {
                let child_errors = returns.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected returns to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Returns
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected returns to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for HexNumberExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::HexNumberExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::HexNumberExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // literal

        {
            let literal = &self.literal;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Literal
            }) {
                let child_errors = literal.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected literal to be present in the CST, but it was not"
                )));
            }
        }

        // unit
        if let Some(unit) = &self.unit {
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Unit)
            {
                let child_errors = unit.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected unit to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Unit)
            {
                errors.push(NodeCheckerError::new(format!(
                    "Expected unit to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for IfStatement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::IfStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::IfStatement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // if_keyword

        {
            let if_keyword = &self.if_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::IfKeyword
            }) {
                let child_errors = if_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected if_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // open_paren

        {
            let open_paren = &self.open_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenParen
            }) {
                let child_errors = open_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_paren to be present in the CST, but it was not"
                )));
            }
        }

        // condition

        {
            let condition = &self.condition;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Condition
            }) {
                let child_errors = condition.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected condition to be present in the CST, but it was not"
                )));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseParen
            }) {
                let child_errors = close_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_paren to be present in the CST, but it was not"
                )));
            }
        }

        // body

        {
            let body = &self.body;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Body)
            {
                let child_errors = body.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected body to be present in the CST, but it was not"
                )));
            }
        }

        // else_branch
        if let Some(else_branch) = &self.else_branch {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::ElseBranch
            }) {
                let child_errors = else_branch.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected else_branch to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::ElseBranch
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected else_branch to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ImportAlias<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ImportAlias) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ImportAlias,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // as_keyword

        {
            let as_keyword = &self.as_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::AsKeyword
            }) {
                let child_errors = as_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected as_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // identifier

        {
            let identifier = &self.identifier;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Identifier
            }) {
                let child_errors = identifier.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected identifier to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ImportDeconstruction<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ImportDeconstruction) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ImportDeconstruction,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // open_brace

        {
            let open_brace = &self.open_brace;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenBrace
            }) {
                let child_errors = open_brace.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_brace to be present in the CST, but it was not"
                )));
            }
        }

        // symbols

        {
            let symbols = &self.symbols;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Symbols
            }) {
                let child_errors = symbols.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected symbols to be present in the CST, but it was not"
                )));
            }
        }

        // close_brace

        {
            let close_brace = &self.close_brace;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseBrace
            }) {
                let child_errors = close_brace.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_brace to be present in the CST, but it was not"
                )));
            }
        }

        // from_keyword

        {
            let from_keyword = &self.from_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::FromKeyword
            }) {
                let child_errors = from_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected from_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // path

        {
            let path = &self.path;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Path)
            {
                let child_errors = path.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected path to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ImportDeconstructionSymbol<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ImportDeconstructionSymbol) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ImportDeconstructionSymbol,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // name

        {
            let name = &self.name;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        }

        // alias
        if let Some(alias) = &self.alias {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Alias
            }) {
                let child_errors = alias.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected alias to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Alias
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected alias to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ImportDirective<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ImportDirective) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ImportDirective,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // import_keyword

        {
            let import_keyword = &self.import_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::ImportKeyword
            }) {
                let child_errors = import_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected import_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // clause

        {
            let clause = &self.clause;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Clause
            }) {
                let child_errors = clause.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected clause to be present in the CST, but it was not"
                )));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Semicolon
            }) {
                let child_errors = semicolon.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected semicolon to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for IndexAccessEnd<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::IndexAccessEnd) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::IndexAccessEnd,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // colon

        {
            let colon = &self.colon;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Colon
            }) {
                let child_errors = colon.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected colon to be present in the CST, but it was not"
                )));
            }
        }

        // end
        if let Some(end) = &self.end {
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::End)
            {
                let child_errors = end.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected end to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::End)
            {
                errors.push(NodeCheckerError::new(format!(
                    "Expected end to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for IndexAccessExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::IndexAccessExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::IndexAccessExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // operand

        {
            let operand = &self.operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operand
            }) {
                let child_errors = operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operand to be present in the CST, but it was not"
                )));
            }
        }

        // open_bracket

        {
            let open_bracket = &self.open_bracket;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenBracket
            }) {
                let child_errors = open_bracket.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_bracket to be present in the CST, but it was not"
                )));
            }
        }

        // start
        if let Some(start) = &self.start {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Start
            }) {
                let child_errors = start.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected start to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Start
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected start to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        // end
        if let Some(end) = &self.end {
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::End)
            {
                let child_errors = end.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected end to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::End)
            {
                errors.push(NodeCheckerError::new(format!(
                    "Expected end to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        // close_bracket

        {
            let close_bracket = &self.close_bracket;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseBracket
            }) {
                let child_errors = close_bracket.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_bracket to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for InequalityExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::InequalityExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::InequalityExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // left_operand

        {
            let left_operand = &self.left_operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::LeftOperand
            }) {
                let child_errors = left_operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected left_operand to be present in the CST, but it was not"
                )));
            }
        }

        // operator

        {
            let operator = &self.operator;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operator
            }) {
                let child_errors = operator.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operator to be present in the CST, but it was not"
                )));
            }
        }

        // right_operand

        {
            let right_operand = &self.right_operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::RightOperand
            }) {
                let child_errors = right_operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected right_operand to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for InheritanceSpecifier<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::InheritanceSpecifier) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::InheritanceSpecifier,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // is_keyword

        {
            let is_keyword = &self.is_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::IsKeyword
            }) {
                let child_errors = is_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected is_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // types

        {
            let types = &self.types;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Types
            }) {
                let child_errors = types.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected types to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for InheritanceType<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::InheritanceType) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::InheritanceType,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // type_name

        {
            let type_name = &self.type_name;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::TypeName
            }) {
                let child_errors = type_name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected type_name to be present in the CST, but it was not"
                )));
            }
        }

        // arguments
        if let Some(arguments) = &self.arguments {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Arguments
            }) {
                let child_errors = arguments.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected arguments to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Arguments
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected arguments to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for InterfaceDefinition<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::InterfaceDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::InterfaceDefinition,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // interface_keyword

        {
            let interface_keyword = &self.interface_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::InterfaceKeyword
            }) {
                let child_errors = interface_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected interface_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // name

        {
            let name = &self.name;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        }

        // inheritance
        if let Some(inheritance) = &self.inheritance {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Inheritance
            }) {
                let child_errors = inheritance.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected inheritance to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Inheritance
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected inheritance to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        // open_brace

        {
            let open_brace = &self.open_brace;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenBrace
            }) {
                let child_errors = open_brace.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_brace to be present in the CST, but it was not"
                )));
            }
        }

        // members

        {
            let members = &self.members;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Members
            }) {
                let child_errors = members.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected members to be present in the CST, but it was not"
                )));
            }
        }

        // close_brace

        {
            let close_brace = &self.close_brace;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseBrace
            }) {
                let child_errors = close_brace.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_brace to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for LibraryDefinition<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::LibraryDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::LibraryDefinition,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // library_keyword

        {
            let library_keyword = &self.library_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::LibraryKeyword
            }) {
                let child_errors = library_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected library_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // name

        {
            let name = &self.name;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        }

        // open_brace

        {
            let open_brace = &self.open_brace;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenBrace
            }) {
                let child_errors = open_brace.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_brace to be present in the CST, but it was not"
                )));
            }
        }

        // members

        {
            let members = &self.members;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Members
            }) {
                let child_errors = members.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected members to be present in the CST, but it was not"
                )));
            }
        }

        // close_brace

        {
            let close_brace = &self.close_brace;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseBrace
            }) {
                let child_errors = close_brace.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_brace to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for MappingKey<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::MappingKey) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::MappingKey,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // key_type

        {
            let key_type = &self.key_type;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::KeyType
            }) {
                let child_errors = key_type.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected key_type to be present in the CST, but it was not"
                )));
            }
        }

        // name
        if let Some(name) = &self.name {
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for MappingType<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::MappingType) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::MappingType,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // mapping_keyword

        {
            let mapping_keyword = &self.mapping_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::MappingKeyword
            }) {
                let child_errors = mapping_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected mapping_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // open_paren

        {
            let open_paren = &self.open_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenParen
            }) {
                let child_errors = open_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_paren to be present in the CST, but it was not"
                )));
            }
        }

        // key_type

        {
            let key_type = &self.key_type;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::KeyType
            }) {
                let child_errors = key_type.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected key_type to be present in the CST, but it was not"
                )));
            }
        }

        // equal_greater_than

        {
            let equal_greater_than = &self.equal_greater_than;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::EqualGreaterThan
            }) {
                let child_errors = equal_greater_than.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected equal_greater_than to be present in the CST, but it was not"
                )));
            }
        }

        // value_type

        {
            let value_type = &self.value_type;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::ValueType
            }) {
                let child_errors = value_type.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected value_type to be present in the CST, but it was not"
                )));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseParen
            }) {
                let child_errors = close_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_paren to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for MappingValue<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::MappingValue) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::MappingValue,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // type_name

        {
            let type_name = &self.type_name;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::TypeName
            }) {
                let child_errors = type_name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected type_name to be present in the CST, but it was not"
                )));
            }
        }

        // name
        if let Some(name) = &self.name {
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for MemberAccessExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::MemberAccessExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::MemberAccessExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // operand

        {
            let operand = &self.operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operand
            }) {
                let child_errors = operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operand to be present in the CST, but it was not"
                )));
            }
        }

        // period

        {
            let period = &self.period;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Period
            }) {
                let child_errors = period.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected period to be present in the CST, but it was not"
                )));
            }
        }

        // member

        {
            let member = &self.member;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Member
            }) {
                let child_errors = member.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected member to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ModifierDefinition<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ModifierDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ModifierDefinition,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // modifier_keyword

        {
            let modifier_keyword = &self.modifier_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::ModifierKeyword
            }) {
                let child_errors = modifier_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected modifier_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // name

        {
            let name = &self.name;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        }

        // parameters
        if let Some(parameters) = &self.parameters {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Parameters
            }) {
                let child_errors = parameters.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected parameters to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Parameters
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected parameters to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        // attributes

        {
            let attributes = &self.attributes;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Attributes
            }) {
                let child_errors = attributes.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected attributes to be present in the CST, but it was not"
                )));
            }
        }

        // body

        {
            let body = &self.body;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Body)
            {
                let child_errors = body.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected body to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ModifierInvocation<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ModifierInvocation) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ModifierInvocation,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // name

        {
            let name = &self.name;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        }

        // arguments
        if let Some(arguments) = &self.arguments {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Arguments
            }) {
                let child_errors = arguments.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected arguments to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Arguments
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected arguments to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for MultiplicativeExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::MultiplicativeExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::MultiplicativeExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // left_operand

        {
            let left_operand = &self.left_operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::LeftOperand
            }) {
                let child_errors = left_operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected left_operand to be present in the CST, but it was not"
                )));
            }
        }

        // operator

        {
            let operator = &self.operator;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operator
            }) {
                let child_errors = operator.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operator to be present in the CST, but it was not"
                )));
            }
        }

        // right_operand

        {
            let right_operand = &self.right_operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::RightOperand
            }) {
                let child_errors = right_operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected right_operand to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for NamedArgument<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::NamedArgument) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::NamedArgument,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // name

        {
            let name = &self.name;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        }

        // colon

        {
            let colon = &self.colon;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Colon
            }) {
                let child_errors = colon.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected colon to be present in the CST, but it was not"
                )));
            }
        }

        // value

        {
            let value = &self.value;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Value
            }) {
                let child_errors = value.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected value to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for NamedArgumentGroup<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::NamedArgumentGroup) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::NamedArgumentGroup,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // open_brace

        {
            let open_brace = &self.open_brace;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenBrace
            }) {
                let child_errors = open_brace.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_brace to be present in the CST, but it was not"
                )));
            }
        }

        // arguments

        {
            let arguments = &self.arguments;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Arguments
            }) {
                let child_errors = arguments.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected arguments to be present in the CST, but it was not"
                )));
            }
        }

        // close_brace

        {
            let close_brace = &self.close_brace;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseBrace
            }) {
                let child_errors = close_brace.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_brace to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for NamedArgumentsDeclaration<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::NamedArgumentsDeclaration) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::NamedArgumentsDeclaration,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // open_paren

        {
            let open_paren = &self.open_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenParen
            }) {
                let child_errors = open_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_paren to be present in the CST, but it was not"
                )));
            }
        }

        // arguments

        {
            let arguments = &self.arguments;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Arguments
            }) {
                let child_errors = arguments.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected arguments to be present in the CST, but it was not"
                )));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseParen
            }) {
                let child_errors = close_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_paren to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for NamedImport<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::NamedImport) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::NamedImport,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // asterisk

        {
            let asterisk = &self.asterisk;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Asterisk
            }) {
                let child_errors = asterisk.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected asterisk to be present in the CST, but it was not"
                )));
            }
        }

        // alias

        {
            let alias = &self.alias;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Alias
            }) {
                let child_errors = alias.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected alias to be present in the CST, but it was not"
                )));
            }
        }

        // from_keyword

        {
            let from_keyword = &self.from_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::FromKeyword
            }) {
                let child_errors = from_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected from_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // path

        {
            let path = &self.path;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Path)
            {
                let child_errors = path.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected path to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

// NodeChecker for NewExpression is done by hand, since in V2 is represented a bit differently

impl<'arena> NodeChecker for NewExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        // what we parse as a new expression on V2 is parsed as a CallExpression, of an optional
        // call options, and a new expression
        // TODO(v2): This is not correct , I'm not checking call options because i'm lazy for now
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FunctionCallExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::FunctionCallExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // Extract operand into expression_node
        let expression_node = if let Some(child) = extract_first(&mut children, |child: &Edge| {
            child.label == EdgeLabel::Operand
        }) {
            child.node
        } else {
            errors.push(NodeCheckerError::new(format!(
                "Expected operand to be present in the CST, but it was not"
            )));
            return errors;
        };

        if expression_node.kind() != NodeKind::Nonterminal(NonterminalKind::Expression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::Expression,
                node.kind()
            ))];
        }

        let mut expression_children = expression_node.children().to_vec();
        remove_trivia(&mut expression_children);

        if expression_children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::Expression,
                expression_children
            ))];
        }

        let expression_child = &expression_children[0];

        if expression_child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                expression_child.label
            ))];
        }

        let new_expression_node = &expression_child.node;

        if new_expression_node.kind() != NodeKind::Nonterminal(NonterminalKind::NewExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::NewExpression,
                new_expression_node.kind()
            ))];
        }

        let mut new_expression_children = new_expression_node.children().to_vec();
        remove_trivia(&mut new_expression_children);

        // new_keyword

        {
            let new_keyword = &self.new_keyword;
            if let Some(child) = extract_first(&mut new_expression_children, |child: &Edge| {
                child.label == EdgeLabel::NewKeyword
            }) {
                let child_errors = new_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected new_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // type_name

        {
            let type_name = &self.type_name;
            if let Some(child) = extract_first(&mut new_expression_children, |child: &Edge| {
                child.label == EdgeLabel::TypeName
            }) {
                let child_errors = type_name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected type_name to be present in the CST, but it was not"
                )));
            }
        }

        // options
        // if let Some(options) = &self.options {
        //     if let Some(child) = extract_first(&mut children, |child: &Edge| {
        //         child.label == EdgeLabel::Options
        //     }) {
        //         let child_errors = options.check_node(&child.node);
        //         errors.extend(child_errors);
        //     } else {
        //         errors.push(NodeCheckerError::new(format!(
        //             "Expected options to be present in the CST, but it was not"
        //         )));
        //     }
        // } else {
        //     // If it's not there on the AST, it shouldn't be in the CST
        //     if let Some(child) = extract_first(&mut children, |child: &Edge| {
        //         child.label == EdgeLabel::Options
        //     }) {
        //         errors.push(NodeCheckerError::new(format!(
        //             "Expected options to not be present in the CST, but it was there: {:#?}",
        //             child
        //         )));
        //     }
        // }

        // arguments

        {
            let arguments = &self.arguments;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Arguments
            }) {
                let child_errors = arguments.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected arguments to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        if !new_expression_children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                new_expression_children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for OrExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::OrExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::OrExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // left_operand

        {
            let left_operand = &self.left_operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::LeftOperand
            }) {
                let child_errors = left_operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected left_operand to be present in the CST, but it was not"
                )));
            }
        }

        // operator

        {
            let operator = &self.operator;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operator
            }) {
                let child_errors = operator.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operator to be present in the CST, but it was not"
                )));
            }
        }

        // right_operand

        {
            let right_operand = &self.right_operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::RightOperand
            }) {
                let child_errors = right_operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected right_operand to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for OverridePathsDeclaration<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::OverridePathsDeclaration) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::OverridePathsDeclaration,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // open_paren

        {
            let open_paren = &self.open_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenParen
            }) {
                let child_errors = open_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_paren to be present in the CST, but it was not"
                )));
            }
        }

        // paths

        {
            let paths = &self.paths;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Paths
            }) {
                let child_errors = paths.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected paths to be present in the CST, but it was not"
                )));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseParen
            }) {
                let child_errors = close_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_paren to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for OverrideSpecifier<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::OverrideSpecifier) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::OverrideSpecifier,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // override_keyword

        {
            let override_keyword = &self.override_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OverrideKeyword
            }) {
                let child_errors = override_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected override_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // overridden
        if let Some(overridden) = &self.overridden {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Overridden
            }) {
                let child_errors = overridden.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected overridden to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Overridden
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected overridden to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for Parameter<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::Parameter) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::Parameter,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // type_name

        {
            let type_name = &self.type_name;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::TypeName
            }) {
                let child_errors = type_name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected type_name to be present in the CST, but it was not"
                )));
            }
        }

        // storage_location
        if let Some(storage_location) = &self.storage_location {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::StorageLocation
            }) {
                let child_errors = storage_location.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected storage_location to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::StorageLocation
            }) {
                errors.push(NodeCheckerError::new(format!("Expected storage_location to not be present in the CST, but it was there: {:#?}", child)));
            }
        }

        // name
        if let Some(name) = &self.name {
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ParametersDeclaration<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ParametersDeclaration) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ParametersDeclaration,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // open_paren

        {
            let open_paren = &self.open_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenParen
            }) {
                let child_errors = open_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_paren to be present in the CST, but it was not"
                )));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Parameters
            }) {
                let child_errors = parameters.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected parameters to be present in the CST, but it was not"
                )));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseParen
            }) {
                let child_errors = close_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_paren to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for PathImport<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::PathImport) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::PathImport,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // path

        {
            let path = &self.path;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Path)
            {
                let child_errors = path.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected path to be present in the CST, but it was not"
                )));
            }
        }

        // alias
        if let Some(alias) = &self.alias {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Alias
            }) {
                let child_errors = alias.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected alias to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Alias
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected alias to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for PositionalArgumentsDeclaration<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::PositionalArgumentsDeclaration) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::PositionalArgumentsDeclaration,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // open_paren

        {
            let open_paren = &self.open_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenParen
            }) {
                let child_errors = open_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_paren to be present in the CST, but it was not"
                )));
            }
        }

        // arguments

        {
            let arguments = &self.arguments;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Arguments
            }) {
                let child_errors = arguments.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected arguments to be present in the CST, but it was not"
                )));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseParen
            }) {
                let child_errors = close_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_paren to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for PostfixExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::PostfixExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::PostfixExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // operand

        {
            let operand = &self.operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operand
            }) {
                let child_errors = operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operand to be present in the CST, but it was not"
                )));
            }
        }

        // operator

        {
            let operator = &self.operator;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operator
            }) {
                let child_errors = operator.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operator to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for PragmaDirective<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::PragmaDirective) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::PragmaDirective,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // pragma_keyword

        {
            let pragma_keyword = &self.pragma_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::PragmaKeyword
            }) {
                let child_errors = pragma_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected pragma_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // pragma

        {
            let pragma = &self.pragma;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Pragma
            }) {
                let child_errors = pragma.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected pragma to be present in the CST, but it was not"
                )));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Semicolon
            }) {
                let child_errors = semicolon.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected semicolon to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for PrefixExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::PrefixExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::PrefixExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // operator

        {
            let operator = &self.operator;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operator
            }) {
                let child_errors = operator.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operator to be present in the CST, but it was not"
                )));
            }
        }

        // operand

        {
            let operand = &self.operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operand
            }) {
                let child_errors = operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operand to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ReceiveFunctionDefinition<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ReceiveFunctionDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ReceiveFunctionDefinition,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // receive_keyword

        {
            let receive_keyword = &self.receive_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::ReceiveKeyword
            }) {
                let child_errors = receive_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected receive_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Parameters
            }) {
                let child_errors = parameters.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected parameters to be present in the CST, but it was not"
                )));
            }
        }

        // attributes

        {
            let attributes = &self.attributes;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Attributes
            }) {
                let child_errors = attributes.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected attributes to be present in the CST, but it was not"
                )));
            }
        }

        // body

        {
            let body = &self.body;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Body)
            {
                let child_errors = body.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected body to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ReturnStatement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ReturnStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ReturnStatement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // return_keyword

        {
            let return_keyword = &self.return_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::ReturnKeyword
            }) {
                let child_errors = return_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected return_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // expression
        if let Some(expression) = &self.expression {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Expression
            }) {
                let child_errors = expression.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected expression to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Expression
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected expression to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Semicolon
            }) {
                let child_errors = semicolon.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected semicolon to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ReturnsDeclaration<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ReturnsDeclaration) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ReturnsDeclaration,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // returns_keyword

        {
            let returns_keyword = &self.returns_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::ReturnsKeyword
            }) {
                let child_errors = returns_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected returns_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // variables

        {
            let variables = &self.variables;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Variables
            }) {
                let child_errors = variables.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected variables to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for RevertStatement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::RevertStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::RevertStatement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // revert_keyword

        {
            let revert_keyword = &self.revert_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::RevertKeyword
            }) {
                let child_errors = revert_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected revert_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // error
        if let Some(error) = &self.error {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Error
            }) {
                let child_errors = error.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected error to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Error
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected error to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        // arguments

        {
            let arguments = &self.arguments;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Arguments
            }) {
                let child_errors = arguments.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected arguments to be present in the CST, but it was not"
                )));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Semicolon
            }) {
                let child_errors = semicolon.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected semicolon to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ShiftExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ShiftExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ShiftExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // left_operand

        {
            let left_operand = &self.left_operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::LeftOperand
            }) {
                let child_errors = left_operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected left_operand to be present in the CST, but it was not"
                )));
            }
        }

        // operator

        {
            let operator = &self.operator;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operator
            }) {
                let child_errors = operator.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operator to be present in the CST, but it was not"
                )));
            }
        }

        // right_operand

        {
            let right_operand = &self.right_operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::RightOperand
            }) {
                let child_errors = right_operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected right_operand to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for SourceUnit<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::SourceUnit) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::SourceUnit,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // members

        {
            let members = &self.members;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Members
            }) {
                let child_errors = members.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected members to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for StateVariableDefinition<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::StateVariableDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::StateVariableDefinition,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // type_name

        {
            let type_name = &self.type_name;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::TypeName
            }) {
                let child_errors = type_name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected type_name to be present in the CST, but it was not"
                )));
            }
        }

        // attributes

        {
            let attributes = &self.attributes;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Attributes
            }) {
                let child_errors = attributes.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected attributes to be present in the CST, but it was not"
                )));
            }
        }

        // name

        {
            let name = &self.name;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        }

        // value
        if let Some(value) = &self.value {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Value
            }) {
                let child_errors = value.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected value to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Value
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected value to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Semicolon
            }) {
                let child_errors = semicolon.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected semicolon to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for StateVariableDefinitionValue<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::StateVariableDefinitionValue) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::StateVariableDefinitionValue,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // equal

        {
            let equal = &self.equal;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Equal
            }) {
                let child_errors = equal.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected equal to be present in the CST, but it was not"
                )));
            }
        }

        // value

        {
            let value = &self.value;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Value
            }) {
                let child_errors = value.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected value to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for StorageLayoutSpecifier<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::StorageLayoutSpecifier) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::StorageLayoutSpecifier,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // layout_keyword

        {
            let layout_keyword = &self.layout_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::LayoutKeyword
            }) {
                let child_errors = layout_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected layout_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // at_keyword

        {
            let at_keyword = &self.at_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::AtKeyword
            }) {
                let child_errors = at_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected at_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // expression

        {
            let expression = &self.expression;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Expression
            }) {
                let child_errors = expression.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected expression to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for StructDefinition<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::StructDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::StructDefinition,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // struct_keyword

        {
            let struct_keyword = &self.struct_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::StructKeyword
            }) {
                let child_errors = struct_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected struct_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // name

        {
            let name = &self.name;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        }

        // open_brace

        {
            let open_brace = &self.open_brace;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenBrace
            }) {
                let child_errors = open_brace.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_brace to be present in the CST, but it was not"
                )));
            }
        }

        // members

        {
            let members = &self.members;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Members
            }) {
                let child_errors = members.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected members to be present in the CST, but it was not"
                )));
            }
        }

        // close_brace

        {
            let close_brace = &self.close_brace;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseBrace
            }) {
                let child_errors = close_brace.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_brace to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for StructMember<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::StructMember) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::StructMember,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // type_name

        {
            let type_name = &self.type_name;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::TypeName
            }) {
                let child_errors = type_name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected type_name to be present in the CST, but it was not"
                )));
            }
        }

        // name

        {
            let name = &self.name;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Semicolon
            }) {
                let child_errors = semicolon.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected semicolon to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for ThrowStatement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ThrowStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ThrowStatement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // throw_keyword

        {
            let throw_keyword = &self.throw_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::ThrowKeyword
            }) {
                let child_errors = throw_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected throw_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Semicolon
            }) {
                let child_errors = semicolon.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected semicolon to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for TryStatement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TryStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::TryStatement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // try_keyword

        {
            let try_keyword = &self.try_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::TryKeyword
            }) {
                let child_errors = try_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected try_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // expression

        {
            let expression = &self.expression;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Expression
            }) {
                let child_errors = expression.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected expression to be present in the CST, but it was not"
                )));
            }
        }

        // returns
        if let Some(returns) = &self.returns {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Returns
            }) {
                let child_errors = returns.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected returns to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Returns
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected returns to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        // body

        {
            let body = &self.body;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Body)
            {
                let child_errors = body.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected body to be present in the CST, but it was not"
                )));
            }
        }

        // catch_clauses

        {
            let catch_clauses = &self.catch_clauses;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CatchClauses
            }) {
                let child_errors = catch_clauses.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected catch_clauses to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for TupleDeconstructionElement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TupleDeconstructionElement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::TupleDeconstructionElement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // member
        if let Some(member) = &self.member {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Member
            }) {
                let child_errors = member.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected member to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Member
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected member to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for TupleDeconstructionStatement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TupleDeconstructionStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::TupleDeconstructionStatement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // var_keyword
        if let Some(var_keyword) = &self.var_keyword {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::VarKeyword
            }) {
                let child_errors = var_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected var_keyword to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::VarKeyword
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected var_keyword to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        // open_paren

        {
            let open_paren = &self.open_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenParen
            }) {
                let child_errors = open_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_paren to be present in the CST, but it was not"
                )));
            }
        }

        // elements

        {
            let elements = &self.elements;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Elements
            }) {
                let child_errors = elements.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected elements to be present in the CST, but it was not"
                )));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseParen
            }) {
                let child_errors = close_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_paren to be present in the CST, but it was not"
                )));
            }
        }

        // equal

        {
            let equal = &self.equal;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Equal
            }) {
                let child_errors = equal.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected equal to be present in the CST, but it was not"
                )));
            }
        }

        // expression

        {
            let expression = &self.expression;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Expression
            }) {
                let child_errors = expression.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected expression to be present in the CST, but it was not"
                )));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Semicolon
            }) {
                let child_errors = semicolon.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected semicolon to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for TupleExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TupleExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::TupleExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // open_paren

        {
            let open_paren = &self.open_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenParen
            }) {
                let child_errors = open_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_paren to be present in the CST, but it was not"
                )));
            }
        }

        // items

        {
            let items = &self.items;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Items
            }) {
                let child_errors = items.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected items to be present in the CST, but it was not"
                )));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseParen
            }) {
                let child_errors = close_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_paren to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for TupleValue<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TupleValue) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::TupleValue,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // expression
        if let Some(expression) = &self.expression {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Expression
            }) {
                let child_errors = expression.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected expression to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Expression
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected expression to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for TypeExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TypeExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::TypeExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // type_keyword

        {
            let type_keyword = &self.type_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::TypeKeyword
            }) {
                let child_errors = type_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected type_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // open_paren

        {
            let open_paren = &self.open_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenParen
            }) {
                let child_errors = open_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_paren to be present in the CST, but it was not"
                )));
            }
        }

        // type_name

        {
            let type_name = &self.type_name;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::TypeName
            }) {
                let child_errors = type_name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected type_name to be present in the CST, but it was not"
                )));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseParen
            }) {
                let child_errors = close_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_paren to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for TypedTupleMember<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TypedTupleMember) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::TypedTupleMember,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // type_name

        {
            let type_name = &self.type_name;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::TypeName
            }) {
                let child_errors = type_name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected type_name to be present in the CST, but it was not"
                )));
            }
        }

        // storage_location
        if let Some(storage_location) = &self.storage_location {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::StorageLocation
            }) {
                let child_errors = storage_location.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected storage_location to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::StorageLocation
            }) {
                errors.push(NodeCheckerError::new(format!("Expected storage_location to not be present in the CST, but it was there: {:#?}", child)));
            }
        }

        // name

        {
            let name = &self.name;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for UncheckedBlock<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UncheckedBlock) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::UncheckedBlock,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // unchecked_keyword

        {
            let unchecked_keyword = &self.unchecked_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::UncheckedKeyword
            }) {
                let child_errors = unchecked_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected unchecked_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // block

        {
            let block = &self.block;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Block
            }) {
                let child_errors = block.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected block to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for UnnamedFunctionDefinition<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UnnamedFunctionDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::UnnamedFunctionDefinition,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // function_keyword

        {
            let function_keyword = &self.function_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::FunctionKeyword
            }) {
                let child_errors = function_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected function_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Parameters
            }) {
                let child_errors = parameters.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected parameters to be present in the CST, but it was not"
                )));
            }
        }

        // attributes

        {
            let attributes = &self.attributes;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Attributes
            }) {
                let child_errors = attributes.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected attributes to be present in the CST, but it was not"
                )));
            }
        }

        // body

        {
            let body = &self.body;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Body)
            {
                let child_errors = body.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected body to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for UntypedTupleMember<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UntypedTupleMember) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::UntypedTupleMember,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // storage_location
        if let Some(storage_location) = &self.storage_location {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::StorageLocation
            }) {
                let child_errors = storage_location.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected storage_location to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::StorageLocation
            }) {
                errors.push(NodeCheckerError::new(format!("Expected storage_location to not be present in the CST, but it was there: {:#?}", child)));
            }
        }

        // name

        {
            let name = &self.name;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for UserDefinedValueTypeDefinition<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UserDefinedValueTypeDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::UserDefinedValueTypeDefinition,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // type_keyword

        {
            let type_keyword = &self.type_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::TypeKeyword
            }) {
                let child_errors = type_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected type_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // name

        {
            let name = &self.name;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        }

        // is_keyword

        {
            let is_keyword = &self.is_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::IsKeyword
            }) {
                let child_errors = is_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected is_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // value_type

        {
            let value_type = &self.value_type;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::ValueType
            }) {
                let child_errors = value_type.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected value_type to be present in the CST, but it was not"
                )));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Semicolon
            }) {
                let child_errors = semicolon.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected semicolon to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for UsingAlias<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UsingAlias) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::UsingAlias,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // as_keyword

        {
            let as_keyword = &self.as_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::AsKeyword
            }) {
                let child_errors = as_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected as_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // operator

        {
            let operator = &self.operator;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operator
            }) {
                let child_errors = operator.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operator to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for UsingDeconstruction<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UsingDeconstruction) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::UsingDeconstruction,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // open_brace

        {
            let open_brace = &self.open_brace;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenBrace
            }) {
                let child_errors = open_brace.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_brace to be present in the CST, but it was not"
                )));
            }
        }

        // symbols

        {
            let symbols = &self.symbols;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Symbols
            }) {
                let child_errors = symbols.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected symbols to be present in the CST, but it was not"
                )));
            }
        }

        // close_brace

        {
            let close_brace = &self.close_brace;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseBrace
            }) {
                let child_errors = close_brace.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_brace to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for UsingDeconstructionSymbol<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UsingDeconstructionSymbol) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::UsingDeconstructionSymbol,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // name

        {
            let name = &self.name;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        }

        // alias
        if let Some(alias) = &self.alias {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Alias
            }) {
                let child_errors = alias.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected alias to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Alias
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected alias to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for UsingDirective<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UsingDirective) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::UsingDirective,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // using_keyword

        {
            let using_keyword = &self.using_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::UsingKeyword
            }) {
                let child_errors = using_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected using_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // clause

        {
            let clause = &self.clause;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Clause
            }) {
                let child_errors = clause.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected clause to be present in the CST, but it was not"
                )));
            }
        }

        // for_keyword

        {
            let for_keyword = &self.for_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::ForKeyword
            }) {
                let child_errors = for_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected for_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // target

        {
            let target = &self.target;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Target
            }) {
                let child_errors = target.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected target to be present in the CST, but it was not"
                )));
            }
        }

        // global_keyword
        if let Some(global_keyword) = &self.global_keyword {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::GlobalKeyword
            }) {
                let child_errors = global_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected global_keyword to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::GlobalKeyword
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected global_keyword to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Semicolon
            }) {
                let child_errors = semicolon.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected semicolon to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

// We skip NodeChecker for VariableDeclaration

// NodeChecker for VariableDeclarationStatement is done by hand, since in V2 is represented a bit differently
impl<'arena> NodeChecker for VariableDeclarationStatement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::VariableDeclarationStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::VariableDeclarationStatement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);
        let mut errors = vec![];

        // variable_declaration

        // variable_type
        {
            let variable_type = &self.variable_declaration.variable_type;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::VariableType
            }) {
                let child_errors = variable_type.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected variable_type to be present in the CST, but it was not"
                )));
            }
        }

        // storage_location
        if let Some(storage_location) = &self.variable_declaration.storage_location {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::StorageLocation
            }) {
                let child_errors = storage_location.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected storage_location to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(_) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::StorageLocation
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected storage_location to not be present in the CST, but it was there"
                )));
            }
        }

        // name

        {
            let name = &self.variable_declaration.name;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        }

        // value
        if let Some(value) = &self.value {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Value
            }) {
                let child_errors = value.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected value to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(_) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Value
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected value to not be present in the CST, but it was there"
                )));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Semicolon
            }) {
                let child_errors = semicolon.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected semicolon to be present in the CST, but it was not"
                )));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for VariableDeclarationValue<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::VariableDeclarationValue) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::VariableDeclarationValue,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // equal

        {
            let equal = &self.equal;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Equal
            }) {
                let child_errors = equal.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected equal to be present in the CST, but it was not"
                )));
            }
        }

        // expression

        {
            let expression = &self.expression;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Expression
            }) {
                let child_errors = expression.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected expression to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for VersionPragma<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::VersionPragma) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::VersionPragma,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // solidity_keyword

        {
            let solidity_keyword = &self.solidity_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::SolidityKeyword
            }) {
                let child_errors = solidity_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected solidity_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // sets

        {
            let sets = &self.sets;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Sets)
            {
                let child_errors = sets.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected sets to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for VersionRange<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::VersionRange) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::VersionRange,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // start

        {
            let start = &self.start;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Start
            }) {
                let child_errors = start.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected start to be present in the CST, but it was not"
                )));
            }
        }

        // minus

        {
            let minus = &self.minus;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Minus
            }) {
                let child_errors = minus.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected minus to be present in the CST, but it was not"
                )));
            }
        }

        // end

        {
            let end = &self.end;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::End)
            {
                let child_errors = end.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected end to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for VersionTerm<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::VersionTerm) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::VersionTerm,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // operator
        if let Some(operator) = &self.operator {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operator
            }) {
                let child_errors = operator.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operator to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operator
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operator to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        // literal

        {
            let literal = &self.literal;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Literal
            }) {
                let child_errors = literal.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected literal to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for WhileStatement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::WhileStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::WhileStatement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // while_keyword

        {
            let while_keyword = &self.while_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::WhileKeyword
            }) {
                let child_errors = while_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected while_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // open_paren

        {
            let open_paren = &self.open_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenParen
            }) {
                let child_errors = open_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_paren to be present in the CST, but it was not"
                )));
            }
        }

        // condition

        {
            let condition = &self.condition;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Condition
            }) {
                let child_errors = condition.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected condition to be present in the CST, but it was not"
                )));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseParen
            }) {
                let child_errors = close_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_paren to be present in the CST, but it was not"
                )));
            }
        }

        // body

        {
            let body = &self.body;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Body)
            {
                let child_errors = body.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected body to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for YulBlock<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulBlock) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulBlock,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // open_brace

        {
            let open_brace = &self.open_brace;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenBrace
            }) {
                let child_errors = open_brace.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_brace to be present in the CST, but it was not"
                )));
            }
        }

        // statements

        {
            let statements = &self.statements;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Statements
            }) {
                let child_errors = statements.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected statements to be present in the CST, but it was not"
                )));
            }
        }

        // close_brace

        {
            let close_brace = &self.close_brace;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseBrace
            }) {
                let child_errors = close_brace.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_brace to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for YulBreakStatement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulBreakStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulBreakStatement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // break_keyword

        {
            let break_keyword = &self.break_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::BreakKeyword
            }) {
                let child_errors = break_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected break_keyword to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for YulColonAndEqual<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulColonAndEqual) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulColonAndEqual,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // colon

        {
            let colon = &self.colon;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Colon
            }) {
                let child_errors = colon.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected colon to be present in the CST, but it was not"
                )));
            }
        }

        // equal

        {
            let equal = &self.equal;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Equal
            }) {
                let child_errors = equal.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected equal to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for YulContinueStatement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulContinueStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulContinueStatement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // continue_keyword

        {
            let continue_keyword = &self.continue_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::ContinueKeyword
            }) {
                let child_errors = continue_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected continue_keyword to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for YulDefaultCase<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulDefaultCase) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulDefaultCase,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // default_keyword

        {
            let default_keyword = &self.default_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::DefaultKeyword
            }) {
                let child_errors = default_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected default_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // body

        {
            let body = &self.body;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Body)
            {
                let child_errors = body.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected body to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for YulEqualAndColon<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulEqualAndColon) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulEqualAndColon,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // equal

        {
            let equal = &self.equal;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Equal
            }) {
                let child_errors = equal.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected equal to be present in the CST, but it was not"
                )));
            }
        }

        // colon

        {
            let colon = &self.colon;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Colon
            }) {
                let child_errors = colon.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected colon to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for YulForStatement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulForStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulForStatement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // for_keyword

        {
            let for_keyword = &self.for_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::ForKeyword
            }) {
                let child_errors = for_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected for_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // initialization

        {
            let initialization = &self.initialization;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Initialization
            }) {
                let child_errors = initialization.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected initialization to be present in the CST, but it was not"
                )));
            }
        }

        // condition

        {
            let condition = &self.condition;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Condition
            }) {
                let child_errors = condition.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected condition to be present in the CST, but it was not"
                )));
            }
        }

        // iterator

        {
            let iterator = &self.iterator;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Iterator
            }) {
                let child_errors = iterator.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected iterator to be present in the CST, but it was not"
                )));
            }
        }

        // body

        {
            let body = &self.body;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Body)
            {
                let child_errors = body.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected body to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for YulFunctionCallExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulFunctionCallExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulFunctionCallExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // operand

        {
            let operand = &self.operand;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Operand
            }) {
                let child_errors = operand.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected operand to be present in the CST, but it was not"
                )));
            }
        }

        // open_paren

        {
            let open_paren = &self.open_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenParen
            }) {
                let child_errors = open_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_paren to be present in the CST, but it was not"
                )));
            }
        }

        // arguments

        {
            let arguments = &self.arguments;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Arguments
            }) {
                let child_errors = arguments.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected arguments to be present in the CST, but it was not"
                )));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseParen
            }) {
                let child_errors = close_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_paren to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for YulFunctionDefinition<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulFunctionDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulFunctionDefinition,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // function_keyword

        {
            let function_keyword = &self.function_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::FunctionKeyword
            }) {
                let child_errors = function_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected function_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // name

        {
            let name = &self.name;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Name)
            {
                let child_errors = name.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected name to be present in the CST, but it was not"
                )));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Parameters
            }) {
                let child_errors = parameters.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected parameters to be present in the CST, but it was not"
                )));
            }
        }

        // returns
        if let Some(returns) = &self.returns {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Returns
            }) {
                let child_errors = returns.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected returns to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Returns
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected returns to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        // body

        {
            let body = &self.body;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Body)
            {
                let child_errors = body.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected body to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for YulIfStatement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulIfStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulIfStatement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // if_keyword

        {
            let if_keyword = &self.if_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::IfKeyword
            }) {
                let child_errors = if_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected if_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // condition

        {
            let condition = &self.condition;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Condition
            }) {
                let child_errors = condition.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected condition to be present in the CST, but it was not"
                )));
            }
        }

        // body

        {
            let body = &self.body;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Body)
            {
                let child_errors = body.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected body to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for YulLabel<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulLabel) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulLabel,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // label

        {
            let label = &self.label;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Label
            }) {
                let child_errors = label.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected label to be present in the CST, but it was not"
                )));
            }
        }

        // colon

        {
            let colon = &self.colon;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Colon
            }) {
                let child_errors = colon.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected colon to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for YulLeaveStatement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulLeaveStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulLeaveStatement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // leave_keyword

        {
            let leave_keyword = &self.leave_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::LeaveKeyword
            }) {
                let child_errors = leave_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected leave_keyword to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for YulParametersDeclaration<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulParametersDeclaration) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulParametersDeclaration,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // open_paren

        {
            let open_paren = &self.open_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::OpenParen
            }) {
                let child_errors = open_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected open_paren to be present in the CST, but it was not"
                )));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Parameters
            }) {
                let child_errors = parameters.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected parameters to be present in the CST, but it was not"
                )));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CloseParen
            }) {
                let child_errors = close_paren.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected close_paren to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for YulReturnsDeclaration<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulReturnsDeclaration) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulReturnsDeclaration,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // minus_greater_than

        {
            let minus_greater_than = &self.minus_greater_than;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::MinusGreaterThan
            }) {
                let child_errors = minus_greater_than.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected minus_greater_than to be present in the CST, but it was not"
                )));
            }
        }

        // variables

        {
            let variables = &self.variables;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Variables
            }) {
                let child_errors = variables.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected variables to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for YulStackAssignmentStatement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulStackAssignmentStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulStackAssignmentStatement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // assignment

        {
            let assignment = &self.assignment;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Assignment
            }) {
                let child_errors = assignment.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected assignment to be present in the CST, but it was not"
                )));
            }
        }

        // variable

        {
            let variable = &self.variable;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Variable
            }) {
                let child_errors = variable.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected variable to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for YulSwitchStatement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulSwitchStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulSwitchStatement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // switch_keyword

        {
            let switch_keyword = &self.switch_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::SwitchKeyword
            }) {
                let child_errors = switch_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected switch_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // expression

        {
            let expression = &self.expression;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Expression
            }) {
                let child_errors = expression.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected expression to be present in the CST, but it was not"
                )));
            }
        }

        // cases

        {
            let cases = &self.cases;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Cases
            }) {
                let child_errors = cases.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected cases to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for YulValueCase<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulValueCase) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulValueCase,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // case_keyword

        {
            let case_keyword = &self.case_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::CaseKeyword
            }) {
                let child_errors = case_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected case_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // value

        {
            let value = &self.value;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Value
            }) {
                let child_errors = value.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected value to be present in the CST, but it was not"
                )));
            }
        }

        // body

        {
            let body = &self.body;
            if let Some(child) =
                extract_first(&mut children, |child: &Edge| child.label == EdgeLabel::Body)
            {
                let child_errors = body.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected body to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for YulVariableAssignmentStatement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulVariableAssignmentStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulVariableAssignmentStatement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // variables

        {
            let variables = &self.variables;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Variables
            }) {
                let child_errors = variables.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected variables to be present in the CST, but it was not"
                )));
            }
        }

        // assignment

        {
            let assignment = &self.assignment;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Assignment
            }) {
                let child_errors = assignment.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected assignment to be present in the CST, but it was not"
                )));
            }
        }

        // expression

        {
            let expression = &self.expression;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Expression
            }) {
                let child_errors = expression.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected expression to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for YulVariableDeclarationStatement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulVariableDeclarationStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulVariableDeclarationStatement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // let_keyword

        {
            let let_keyword = &self.let_keyword;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::LetKeyword
            }) {
                let child_errors = let_keyword.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected let_keyword to be present in the CST, but it was not"
                )));
            }
        }

        // variables

        {
            let variables = &self.variables;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Variables
            }) {
                let child_errors = variables.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected variables to be present in the CST, but it was not"
                )));
            }
        }

        // value
        if let Some(value) = &self.value {
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Value
            }) {
                let child_errors = value.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected value to be present in the CST, but it was not"
                )));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Value
            }) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected value to not be present in the CST, but it was there: {:#?}",
                    child
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

impl<'arena> NodeChecker for YulVariableDeclarationValue<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulVariableDeclarationValue) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulVariableDeclarationValue,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        let mut errors = vec![];

        // assignment

        {
            let assignment = &self.assignment;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Assignment
            }) {
                let child_errors = assignment.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected assignment to be present in the CST, but it was not"
                )));
            }
        }

        // expression

        {
            let expression = &self.expression;
            if let Some(child) = extract_first(&mut children, |child: &Edge| {
                child.label == EdgeLabel::Expression
            }) {
                let child_errors = expression.check_node(&child.node);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(format!(
                    "Expected expression to be present in the CST, but it was not"
                )));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left, but there's some left {:#?}",
                children
            )));
        }

        errors
    }
}

//
// Choices:
//

impl<'arena> NodeChecker for AbicoderVersion<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::AbicoderVersion) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::AbicoderVersion,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::AbicoderVersion,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::AbicoderV1Keyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::AbicoderV2Keyword(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for ArgumentsDeclaration<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ArgumentsDeclaration) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ArgumentsDeclaration,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::ArgumentsDeclaration,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::PositionalArgumentsDeclaration(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::NamedArgumentsDeclaration(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for ConstructorAttribute<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ConstructorAttribute) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ConstructorAttribute,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::ConstructorAttribute,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::ModifierInvocation(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::InternalKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::OverrideKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::PayableKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::PublicKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::VirtualKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for ContractMember<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ContractMember) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ContractMember,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::ContractMember,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::UsingDirective(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::FunctionDefinition(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ConstructorDefinition(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ReceiveFunctionDefinition(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::FallbackFunctionDefinition(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::UnnamedFunctionDefinition(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ModifierDefinition(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::StructDefinition(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::EnumDefinition(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::EventDefinition(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ErrorDefinition(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::UserDefinedValueTypeDefinition(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::StateVariableDefinition(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for ContractSpecifier<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ContractSpecifier) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ContractSpecifier,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::ContractSpecifier,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::InheritanceSpecifier(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::StorageLayoutSpecifier(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for ElementaryType<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ElementaryType) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ElementaryType,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::ElementaryType,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::BoolKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ByteKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::StringKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::AddressType(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::BytesKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::IntKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::UintKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::FixedKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::UfixedKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for ExperimentalFeature<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ExperimentalFeature) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ExperimentalFeature,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::ExperimentalFeature,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::ABIEncoderV2Keyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::SMTCheckerKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::StringLiteral(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for Expression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::Expression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::Expression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::Expression,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::AssignmentExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ConditionalExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::OrExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::AndExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::EqualityExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::InequalityExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::BitwiseOrExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::BitwiseXorExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::BitwiseAndExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ShiftExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::AdditiveExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::MultiplicativeExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ExponentiationExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::PostfixExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::PrefixExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::FunctionCallExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::CallOptionsExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::MemberAccessExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::IndexAccessExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::NewExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::TupleExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::TypeExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ArrayExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::HexNumberExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::DecimalNumberExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::StringExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ElementaryType(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::PayableKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ThisKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::SuperKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::TrueKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::FalseKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::Identifier(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for FallbackFunctionAttribute<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FallbackFunctionAttribute) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::FallbackFunctionAttribute,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::FallbackFunctionAttribute,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::ModifierInvocation(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::OverrideSpecifier(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ExternalKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::PayableKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::PureKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ViewKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::VirtualKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for ForStatementCondition<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ForStatementCondition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ForStatementCondition,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::ForStatementCondition,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::ExpressionStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::Semicolon(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for ForStatementInitialization<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ForStatementInitialization) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ForStatementInitialization,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::ForStatementInitialization,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::TupleDeconstructionStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::VariableDeclarationStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ExpressionStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::Semicolon(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for FunctionAttribute<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FunctionAttribute) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::FunctionAttribute,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::FunctionAttribute,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::ModifierInvocation(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::OverrideSpecifier(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ConstantKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ExternalKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::InternalKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::PayableKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::PrivateKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::PublicKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::PureKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ViewKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::VirtualKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for FunctionBody<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FunctionBody) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::FunctionBody,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::FunctionBody,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::Block(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::Semicolon(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for FunctionName<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FunctionName) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::FunctionName,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::FunctionName,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::Identifier(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::FallbackKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ReceiveKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for FunctionTypeAttribute<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FunctionTypeAttribute) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::FunctionTypeAttribute,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::FunctionTypeAttribute,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::InternalKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ExternalKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::PrivateKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::PublicKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ConstantKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::PureKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ViewKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::PayableKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for HexStringLiteral<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::HexStringLiteral) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::HexStringLiteral,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::HexStringLiteral,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::SingleQuotedHexStringLiteral(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::DoubleQuotedHexStringLiteral(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for ImportClause<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ImportClause) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ImportClause,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::ImportClause,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::PathImport(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::NamedImport(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ImportDeconstruction(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for MappingKeyType<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::MappingKeyType) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::MappingKeyType,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::MappingKeyType,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::ElementaryType(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::IdentifierPath(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for MemberAccessIdentifier<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        let ident_node = match self {
            Self::Identifier(element) => TerminalType {
                // TODO(v2): avoid the allocation here, it's not needed
                value: element.value.clone(),
                l: element.l,
                r: element.r,
                kind: element.kind.clone(),
                phantom: PhantomData,
            },
            Self::AddressKeyword(element) => TerminalType {
                // TODO(v2): avoid the allocation here, it's not needed
                value: element.value.clone(),
                l: element.l,
                r: element.r,
                kind: LexemeKind::Identifier,
                phantom: PhantomData,
            },
        };

        ident_node.check_node(node)
    }
}

impl<'arena> NodeChecker for ModifierAttribute<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ModifierAttribute) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ModifierAttribute,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::ModifierAttribute,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::OverrideSpecifier(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::VirtualKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for NumberUnit<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::NumberUnit) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::NumberUnit,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::NumberUnit,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::WeiKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::GweiKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::SzaboKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::FinneyKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::EtherKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::SecondsKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::MinutesKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::HoursKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::DaysKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::WeeksKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::YearsKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for Pragma<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::Pragma) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::Pragma,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::Pragma,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::VersionPragma(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::AbicoderPragma(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ExperimentalPragma(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for ReceiveFunctionAttribute<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ReceiveFunctionAttribute) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ReceiveFunctionAttribute,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::ReceiveFunctionAttribute,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::ModifierInvocation(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::OverrideSpecifier(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ExternalKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::PayableKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::VirtualKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for SourceUnitMember<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::SourceUnitMember) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::SourceUnitMember,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::SourceUnitMember,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::PragmaDirective(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ImportDirective(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ContractDefinition(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::InterfaceDefinition(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::LibraryDefinition(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::StructDefinition(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::EnumDefinition(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::FunctionDefinition(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ErrorDefinition(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::UserDefinedValueTypeDefinition(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::UsingDirective(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::EventDefinition(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ConstantDefinition(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for StateVariableAttribute<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::StateVariableAttribute) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::StateVariableAttribute,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::StateVariableAttribute,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::OverrideSpecifier(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ConstantKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::InternalKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::PrivateKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::PublicKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ImmutableKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::TransientKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for Statement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::Statement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::Statement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::Statement,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::IfStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ForStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::WhileStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::DoWhileStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ContinueStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::BreakStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ReturnStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ThrowStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::EmitStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::TryStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::RevertStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::AssemblyStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::Block(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::UncheckedBlock(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::TupleDeconstructionStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::VariableDeclarationStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ExpressionStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for StorageLocation<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::StorageLocation) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::StorageLocation,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::StorageLocation,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::MemoryKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::StorageKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::CallDataKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for StringExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::StringExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::StringExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::StringExpression,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::StringLiteral(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::StringLiterals(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::HexStringLiteral(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::HexStringLiterals(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::UnicodeStringLiterals(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for StringLiteral<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::StringLiteral) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::StringLiteral,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::StringLiteral,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::SingleQuotedStringLiteral(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::DoubleQuotedStringLiteral(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for TupleMember<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TupleMember) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::TupleMember,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::TupleMember,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::TypedTupleMember(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::UntypedTupleMember(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for TypeName<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TypeName) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::TypeName,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::TypeName,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::ArrayTypeName(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::FunctionType(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::MappingType(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ElementaryType(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::IdentifierPath(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for UnicodeStringLiteral<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UnicodeStringLiteral) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::UnicodeStringLiteral,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::UnicodeStringLiteral,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::SingleQuotedUnicodeStringLiteral(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::DoubleQuotedUnicodeStringLiteral(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for UnnamedFunctionAttribute<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UnnamedFunctionAttribute) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::UnnamedFunctionAttribute,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::UnnamedFunctionAttribute,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::ModifierInvocation(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ConstantKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ExternalKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::InternalKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::PayableKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::PrivateKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::PublicKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::PureKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::ViewKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for UsingClause<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UsingClause) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::UsingClause,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::UsingClause,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::IdentifierPath(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::UsingDeconstruction(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for UsingOperator<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UsingOperator) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::UsingOperator,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::UsingOperator,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::Ampersand(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::Asterisk(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::BangEqual(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::Bar(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::Caret(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::EqualEqual(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::GreaterThan(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::GreaterThanEqual(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::LessThan(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::LessThanEqual(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::Minus(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::Percent(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::Plus(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::Slash(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::Tilde(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for UsingTarget<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UsingTarget) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::UsingTarget,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::UsingTarget,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::TypeName(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::Asterisk(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for VariableDeclarationType<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::VariableDeclarationType) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::VariableDeclarationType,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::VariableDeclarationType,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::TypeName(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::VarKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for VersionExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::VersionExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::VersionExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::VersionExpression,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::VersionRange(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::VersionTerm(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for VersionLiteral<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::VersionLiteral) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::VersionLiteral,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::VersionLiteral,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::SimpleVersionLiteral(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::SingleQuotedVersionLiteral(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::DoubleQuotedVersionLiteral(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for VersionOperator<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::VersionOperator) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::VersionOperator,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::VersionOperator,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::Caret(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::Tilde(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::Equal(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::LessThan(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::GreaterThan(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::LessThanEqual(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::GreaterThanEqual(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for YulAssignmentOperator<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulAssignmentOperator) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulAssignmentOperator,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::YulAssignmentOperator,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::ColonEqual(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::YulColonAndEqual(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for YulExpression<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulExpression,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::YulExpression,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::YulFunctionCallExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::YulLiteral(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::YulPath(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for YulLiteral<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulLiteral) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulLiteral,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::YulLiteral,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::YulTrueKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::YulFalseKeyword(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::YulDecimalLiteral(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::YulHexLiteral(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::HexStringLiteral(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::StringLiteral(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for YulStackAssignmentOperator<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulStackAssignmentOperator) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulStackAssignmentOperator,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::YulStackAssignmentOperator,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::EqualColon(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::YulEqualAndColon(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for YulStatement<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulStatement,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::YulStatement,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::YulBlock(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::YulFunctionDefinition(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::YulStackAssignmentStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::YulIfStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::YulForStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::YulSwitchStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::YulLeaveStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::YulBreakStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::YulContinueStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::YulVariableAssignmentStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::YulLabel(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::YulVariableDeclarationStatement(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::YulExpression(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

impl<'arena> NodeChecker for YulSwitchCase<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulSwitchCase) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulSwitchCase,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(format!(
                "Expected exactly one child for {}, but got: {:#?}",
                NonterminalKind::YulSwitchCase,
                children
            ))];
        }

        let child = &children[0];

        if child.label != EdgeLabel::Variant {
            return vec![NodeCheckerError::new(format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ))];
        }

        let mut errors = vec![];

        match self {
            Self::YulDefaultCase(element) => {
                errors.extend(element.check_node(&child.node));
            }

            Self::YulValueCase(element) => {
                errors.extend(element.check_node(&child.node));
            }
        }

        errors
    }
}

//
// Repeated & Separated
//

impl<'arena> NodeChecker for ArrayValues<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ArrayValues) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ArrayValues,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for AssemblyFlags<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::AssemblyFlags) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::AssemblyFlags,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for CallOptions<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::CallOptions) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::CallOptions,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for CatchClauses<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::CatchClauses) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::CatchClauses,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for ConstructorAttributes<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ConstructorAttributes) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ConstructorAttributes,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for ContractMembers<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ContractMembers) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ContractMembers,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for ContractSpecifiers<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ContractSpecifiers) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ContractSpecifiers,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for EnumMembers<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::EnumMembers) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::EnumMembers,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for ErrorParameters<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ErrorParameters) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ErrorParameters,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for EventParameters<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::EventParameters) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::EventParameters,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for FallbackFunctionAttributes<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FallbackFunctionAttributes) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::FallbackFunctionAttributes,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for FunctionAttributes<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FunctionAttributes) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::FunctionAttributes,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for FunctionTypeAttributes<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FunctionTypeAttributes) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::FunctionTypeAttributes,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for HexStringLiterals<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::HexStringLiterals) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::HexStringLiterals,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for IdentifierPath<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::IdentifierPath) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::IdentifierPath,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for ImportDeconstructionSymbols<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ImportDeconstructionSymbols) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ImportDeconstructionSymbols,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for InheritanceTypes<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::InheritanceTypes) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::InheritanceTypes,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for InterfaceMembers<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::InterfaceMembers) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::InterfaceMembers,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for LibraryMembers<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::LibraryMembers) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::LibraryMembers,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for ModifierAttributes<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ModifierAttributes) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ModifierAttributes,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for NamedArguments<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::NamedArguments) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::NamedArguments,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for OverridePaths<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::OverridePaths) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::OverridePaths,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for Parameters<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::Parameters) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::Parameters,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for PositionalArguments<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::PositionalArguments) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::PositionalArguments,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for ReceiveFunctionAttributes<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ReceiveFunctionAttributes) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::ReceiveFunctionAttributes,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for SimpleVersionLiteral<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::SimpleVersionLiteral) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::SimpleVersionLiteral,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for SourceUnitMembers<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::SourceUnitMembers) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::SourceUnitMembers,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for StateVariableAttributes<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::StateVariableAttributes) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::StateVariableAttributes,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for Statements<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::Statements) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::Statements,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for StringLiterals<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::StringLiterals) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::StringLiterals,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for StructMembers<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::StructMembers) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::StructMembers,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for TupleDeconstructionElements<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TupleDeconstructionElements) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::TupleDeconstructionElements,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for TupleValues<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TupleValues) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::TupleValues,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for UnicodeStringLiterals<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UnicodeStringLiterals) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::UnicodeStringLiterals,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for UnnamedFunctionAttributes<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UnnamedFunctionAttributes) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::UnnamedFunctionAttributes,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for UsingDeconstructionSymbols<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UsingDeconstructionSymbols) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::UsingDeconstructionSymbols,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for VersionExpressionSet<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::VersionExpressionSet) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::VersionExpressionSet,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for VersionExpressionSets<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::VersionExpressionSets) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::VersionExpressionSets,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for YulArguments<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulArguments) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulArguments,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for YulParameters<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulParameters) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulParameters,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for YulPath<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulPath) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulPath,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for YulPaths<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulPaths) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulPaths,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for YulStatements<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulStatements) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulStatements,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for YulSwitchCases<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulSwitchCases) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulSwitchCases,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

impl<'arena> NodeChecker for YulVariableNames<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulVariableNames) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be {}, but it was {}",
                NonterminalKind::YulVariableNames,
                node.kind()
            ))];
        }

        let mut children = node.children().to_vec();
        remove_trivia(&mut children);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(format!(
                "Expected {} elements, but got: {:#?}",
                self.elements.len(),
                children
            ))];
        }

        let mut errors = vec![];

        for (i, child) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node(child));
        }
        errors
    }
}

// Terminals

impl<'arena> NodeChecker for TerminalType<'arena> {
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind: &'static str = self.kind.clone().into();

            if !Comparator::match_terminal(v1_kind, v2_kind) {
                errors.push(NodeCheckerError::new(format!(
                    "Expected node kind to be {}, but it was {}",
                    v2_kind, v1_kind
                )));
            }
        } else {
            errors.push(NodeCheckerError::new(format!(
                "Expected node kind to be a terminal, but it was {}",
                node.kind()
            )));
        }
        return errors;
    }
}
