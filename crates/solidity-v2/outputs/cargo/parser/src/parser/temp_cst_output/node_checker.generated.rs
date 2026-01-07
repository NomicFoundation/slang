// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_lines)]

use std::marker::PhantomData;

use slang_solidity::cst::{Edge, EdgeLabel, Node, NodeKind, NonterminalKind, TextIndex, TextRange};
use slang_solidity::diagnostic::{Diagnostic, Severity};
#[allow(clippy::wildcard_imports)]
use slang_solidity_v2_cst::structured_cst::nodes::*;

/// An error found when checking a node
#[derive(Clone, Debug)]
pub struct NodeCheckerError {
    pub err: String,
    pub text_range: TextRange,
}

impl Diagnostic for NodeCheckerError {
    fn text_range(&self) -> TextRange {
        self.text_range.clone()
    }

    fn severity(&self) -> Severity {
        Severity::Error
    }

    fn message(&self) -> String {
        self.err.clone()
    }
}

impl NodeCheckerError {
    pub(crate) fn new(err: String, text_range: TextRange) -> NodeCheckerError {
        NodeCheckerError { err, text_range }
    }
}

/// A trait that checks whether a CST node (V1) matches the structure of the AST node (V2).
pub trait NodeChecker {
    /// Check whether self matches the given CST node, tracking text offset.
    ///
    /// Returns a vector of errors found, empty if no errors.
    ///
    /// Note: The text offset calculating and tracking seems to reinvent a lot of the logic behind the Cursor,
    /// but it's simple enough that we can do it here. Using a cursor makes it more difficult to compare both
    /// trees, since we'd need to keep track of which children we've seen.
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError>;

    /// Check whether self matches the given CST node.
    ///
    /// Returns a vector of errors found, empty if no errors.
    fn check_node(&self, node: &Node) -> Vec<NodeCheckerError> {
        self.check_node_with_offset(node, TextIndex::ZERO)
    }
}

/// Extract the first element that satisfies the predicate, and remove it from the vector.
fn extract_first<T>(v: &mut Vec<T>, finder: impl Fn(&T) -> bool) -> Option<T> {
    if let Some(idx) = v.iter().position(finder) {
        return Some(v.remove(idx));
    }
    None
}

/// Extract the first edge with the given label, and remove it from the vector.
fn extract_first_with_label(
    v: &mut Vec<(Edge, TextIndex)>,
    label: EdgeLabel,
) -> Option<(Edge, TextIndex)> {
    extract_first(v, |(child, _): &(Edge, TextIndex)| child.label == label)
}

/// Compute children with their text offsets, filtering out trivia and separators.
///
/// This computes offsets BEFORE filtering so that offsets remain accurate.
///
/// TODO: At some point we may need to check the trivia as well
fn children_with_offsets(node: &Node, text_offset: TextIndex) -> Vec<(Edge, TextIndex)> {
    let mut result = vec![];
    let mut current_offset = text_offset;

    for child in node.children() {
        let child_offset = current_offset;
        current_offset += child.node.text_len();

        // Skip trivia and separators (V2 doesn't parse them)
        if child.node.is_trivia() || child.label == EdgeLabel::Separator {
            continue;
        }

        result.push((child.clone(), child_offset));
    }

    result
}

//
// Sequences:
//

/// Generic `NodeChecker` for sequences
impl NodeChecker for AbicoderPragma<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::AbicoderPragma) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::AbicoderPragma,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // abicoder_keyword

        {
            let abicoder_keyword = &self.abicoder_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::AbicoderKeyword)
            {
                let child_errors =
                    abicoder_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected abicoder_keyword to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        }

        // version

        {
            let version = &self.version;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Version)
            {
                let child_errors = version.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected version to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for AdditiveExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::AdditiveExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::AdditiveExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // left_operand

        {
            let left_operand = &self.left_operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::LeftOperand)
            {
                let child_errors = left_operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected left_operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // Expression_AdditiveExpression_Operator

        {
            let expression_additive_expression_operator =
                &self.expression_additive_expression_operator;

            // Prepare edge label

            // Special case for operator fields that are merged together

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Operator)
            {
                let child_errors = expression_additive_expression_operator
                    .check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new("Expected Expression_AdditiveExpression_Operator to be present in the CST, but it was not".to_string(), node_range.clone()));
            }
        }

        // right_operand

        {
            let right_operand = &self.right_operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::RightOperand)
            {
                let child_errors = right_operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected right_operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for AddressType<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::AddressType) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::AddressType,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // address_keyword

        {
            let address_keyword = &self.address_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::AddressKeyword)
            {
                let child_errors =
                    address_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected address_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // payable_keyword
        if let Some(payable_keyword) = &self.payable_keyword {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::PayableKeyword)
            {
                let child_errors =
                    payable_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected payable_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) =
                extract_first_with_label(&mut children, EdgeLabel::PayableKeyword)
            {
                errors.push(NodeCheckerError::new(format!("Expected payable_keyword to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for AndExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::AndExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::AndExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // left_operand

        {
            let left_operand = &self.left_operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::LeftOperand)
            {
                let child_errors = left_operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected left_operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // operator

        {
            let operator = &self.operator;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Operator)
            {
                let child_errors = operator.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected operator to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // right_operand

        {
            let right_operand = &self.right_operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::RightOperand)
            {
                let child_errors = right_operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected right_operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ArrayExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ArrayExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ArrayExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // open_bracket

        {
            let open_bracket = &self.open_bracket;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenBracket)
            {
                let child_errors = open_bracket.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_bracket to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // items

        {
            let items = &self.items;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Items)
            {
                let child_errors = items.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected items to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_bracket

        {
            let close_bracket = &self.close_bracket;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseBracket)
            {
                let child_errors = close_bracket.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_bracket to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ArrayTypeName<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ArrayTypeName) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ArrayTypeName,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // operand

        {
            let operand = &self.operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Operand)
            {
                let child_errors = operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // open_bracket

        {
            let open_bracket = &self.open_bracket;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenBracket)
            {
                let child_errors = open_bracket.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_bracket to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // index
        if let Some(index) = &self.index {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Index)
            {
                let child_errors = index.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected index to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Index) {
                errors.push(NodeCheckerError::new(
                    format!(
                        "Expected index to not be present in the CST, but it was there: {child:#?}"
                    ),
                    node_range.clone(),
                ));
            }
        }

        // close_bracket

        {
            let close_bracket = &self.close_bracket;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseBracket)
            {
                let child_errors = close_bracket.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_bracket to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for AssemblyFlagsDeclaration<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::AssemblyFlagsDeclaration) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::AssemblyFlagsDeclaration,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // open_paren

        {
            let open_paren = &self.open_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenParen)
            {
                let child_errors = open_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // flags

        {
            let flags = &self.flags;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Flags)
            {
                let child_errors = flags.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected flags to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseParen)
            {
                let child_errors = close_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for AssemblyStatement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::AssemblyStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::AssemblyStatement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // assembly_keyword

        {
            let assembly_keyword = &self.assembly_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::AssemblyKeyword)
            {
                let child_errors =
                    assembly_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected assembly_keyword to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        }

        // label
        if let Some(label) = &self.label {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Label)
            {
                let child_errors = label.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected label to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Label) {
                errors.push(NodeCheckerError::new(
                    format!(
                        "Expected label to not be present in the CST, but it was there: {child:#?}"
                    ),
                    node_range.clone(),
                ));
            }
        }

        // flags
        if let Some(flags) = &self.flags {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Flags)
            {
                let child_errors = flags.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected flags to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Flags) {
                errors.push(NodeCheckerError::new(
                    format!(
                        "Expected flags to not be present in the CST, but it was there: {child:#?}"
                    ),
                    node_range.clone(),
                ));
            }
        }

        // body

        {
            let body = &self.body;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Body)
            {
                let child_errors = body.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected body to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for AssignmentExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::AssignmentExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::AssignmentExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // left_operand

        {
            let left_operand = &self.left_operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::LeftOperand)
            {
                let child_errors = left_operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected left_operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // Expression_AssignmentExpression_Operator

        {
            let expression_assignment_expression_operator =
                &self.expression_assignment_expression_operator;

            // Prepare edge label

            // Special case for operator fields that are merged together

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Operator)
            {
                let child_errors = expression_assignment_expression_operator
                    .check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new("Expected Expression_AssignmentExpression_Operator to be present in the CST, but it was not".to_string(), node_range.clone()));
            }
        }

        // right_operand

        {
            let right_operand = &self.right_operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::RightOperand)
            {
                let child_errors = right_operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected right_operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for BitwiseAndExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::BitwiseAndExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::BitwiseAndExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // left_operand

        {
            let left_operand = &self.left_operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::LeftOperand)
            {
                let child_errors = left_operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected left_operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // operator

        {
            let operator = &self.operator;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Operator)
            {
                let child_errors = operator.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected operator to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // right_operand

        {
            let right_operand = &self.right_operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::RightOperand)
            {
                let child_errors = right_operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected right_operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for BitwiseOrExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::BitwiseOrExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::BitwiseOrExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // left_operand

        {
            let left_operand = &self.left_operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::LeftOperand)
            {
                let child_errors = left_operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected left_operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // operator

        {
            let operator = &self.operator;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Operator)
            {
                let child_errors = operator.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected operator to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // right_operand

        {
            let right_operand = &self.right_operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::RightOperand)
            {
                let child_errors = right_operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected right_operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for BitwiseXorExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::BitwiseXorExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::BitwiseXorExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // left_operand

        {
            let left_operand = &self.left_operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::LeftOperand)
            {
                let child_errors = left_operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected left_operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // operator

        {
            let operator = &self.operator;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Operator)
            {
                let child_errors = operator.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected operator to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // right_operand

        {
            let right_operand = &self.right_operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::RightOperand)
            {
                let child_errors = right_operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected right_operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for Block<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::Block) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::Block,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // open_brace

        {
            let open_brace = &self.open_brace;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenBrace)
            {
                let child_errors = open_brace.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_brace to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // statements

        {
            let statements = &self.statements;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Statements)
            {
                let child_errors = statements.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected statements to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_brace

        {
            let close_brace = &self.close_brace;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseBrace)
            {
                let child_errors = close_brace.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_brace to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for BreakStatement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::BreakStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::BreakStatement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // break_keyword

        {
            let break_keyword = &self.break_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::BreakKeyword)
            {
                let child_errors = break_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected break_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Semicolon)
            {
                let child_errors = semicolon.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected semicolon to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for CallOptionsExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::CallOptionsExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::CallOptionsExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // operand

        {
            let operand = &self.operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Operand)
            {
                let child_errors = operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // open_brace

        {
            let open_brace = &self.open_brace;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenBrace)
            {
                let child_errors = open_brace.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_brace to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // options

        {
            let options = &self.options;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Options)
            {
                let child_errors = options.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected options to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_brace

        {
            let close_brace = &self.close_brace;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseBrace)
            {
                let child_errors = close_brace.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_brace to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

// We skip NodeChecker for CallOptionsNew, it's handled by NewExpression

/// Generic `NodeChecker` for sequences
impl NodeChecker for CatchClause<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::CatchClause) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::CatchClause,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // catch_keyword

        {
            let catch_keyword = &self.catch_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CatchKeyword)
            {
                let child_errors = catch_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected catch_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // error
        if let Some(error) = &self.error {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Error)
            {
                let child_errors = error.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected error to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Error) {
                errors.push(NodeCheckerError::new(
                    format!(
                        "Expected error to not be present in the CST, but it was there: {child:#?}"
                    ),
                    node_range.clone(),
                ));
            }
        }

        // body

        {
            let body = &self.body;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Body)
            {
                let child_errors = body.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected body to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for CatchClauseError<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::CatchClauseError) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::CatchClauseError,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // name
        if let Some(name) = &self.name {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Name) {
                errors.push(NodeCheckerError::new(
                    format!(
                        "Expected name to not be present in the CST, but it was there: {child:#?}"
                    ),
                    node_range.clone(),
                ));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Parameters)
            {
                let child_errors = parameters.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected parameters to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ConditionalExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ConditionalExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ConditionalExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // operand

        {
            let operand = &self.operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Operand)
            {
                let child_errors = operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // question_mark

        {
            let question_mark = &self.question_mark;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::QuestionMark)
            {
                let child_errors = question_mark.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected question_mark to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // true_expression

        {
            let true_expression = &self.true_expression;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::TrueExpression)
            {
                let child_errors =
                    true_expression.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected true_expression to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // colon

        {
            let colon = &self.colon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Colon)
            {
                let child_errors = colon.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected colon to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // false_expression

        {
            let false_expression = &self.false_expression;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::FalseExpression)
            {
                let child_errors =
                    false_expression.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected false_expression to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ConstantDefinition<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ConstantDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ConstantDefinition,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // type_name

        {
            let type_name = &self.type_name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::TypeName)
            {
                let child_errors = type_name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected type_name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // constant_keyword

        {
            let constant_keyword = &self.constant_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::ConstantKeyword)
            {
                let child_errors =
                    constant_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected constant_keyword to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        }

        // name

        {
            let name = &self.name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // equal

        {
            let equal = &self.equal;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Equal)
            {
                let child_errors = equal.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected equal to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // value

        {
            let value = &self.value;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Value)
            {
                let child_errors = value.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected value to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Semicolon)
            {
                let child_errors = semicolon.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected semicolon to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ConstructorDefinition<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ConstructorDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ConstructorDefinition,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // constructor_keyword

        {
            let constructor_keyword = &self.constructor_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::ConstructorKeyword)
            {
                let child_errors =
                    constructor_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected constructor_keyword to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Parameters)
            {
                let child_errors = parameters.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected parameters to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // attributes

        {
            let attributes = &self.attributes;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Attributes)
            {
                let child_errors = attributes.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected attributes to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // body

        {
            let body = &self.body;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Body)
            {
                let child_errors = body.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected body to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ContinueStatement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ContinueStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ContinueStatement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // continue_keyword

        {
            let continue_keyword = &self.continue_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::ContinueKeyword)
            {
                let child_errors =
                    continue_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected continue_keyword to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Semicolon)
            {
                let child_errors = semicolon.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected semicolon to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ContractDefinition<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ContractDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ContractDefinition,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // abstract_keyword
        if let Some(abstract_keyword) = &self.abstract_keyword {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::AbstractKeyword)
            {
                let child_errors =
                    abstract_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected abstract_keyword to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) =
                extract_first_with_label(&mut children, EdgeLabel::AbstractKeyword)
            {
                errors.push(NodeCheckerError::new(format!("Expected abstract_keyword to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // contract_keyword

        {
            let contract_keyword = &self.contract_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::ContractKeyword)
            {
                let child_errors =
                    contract_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected contract_keyword to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        }

        // name

        {
            let name = &self.name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // specifiers

        {
            let specifiers = &self.specifiers;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Specifiers)
            {
                let child_errors = specifiers.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected specifiers to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // open_brace

        {
            let open_brace = &self.open_brace;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenBrace)
            {
                let child_errors = open_brace.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_brace to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // members

        {
            let members = &self.members;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Members)
            {
                let child_errors = members.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected members to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_brace

        {
            let close_brace = &self.close_brace;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseBrace)
            {
                let child_errors = close_brace.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_brace to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for DecimalNumberExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::DecimalNumberExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::DecimalNumberExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // literal

        {
            let literal = &self.literal;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Literal)
            {
                let child_errors = literal.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected literal to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // unit
        if let Some(unit) = &self.unit {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Unit)
            {
                let child_errors = unit.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected unit to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Unit) {
                errors.push(NodeCheckerError::new(
                    format!(
                        "Expected unit to not be present in the CST, but it was there: {child:#?}"
                    ),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for DoWhileStatement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::DoWhileStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::DoWhileStatement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // do_keyword

        {
            let do_keyword = &self.do_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::DoKeyword)
            {
                let child_errors = do_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected do_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // body

        {
            let body = &self.body;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Body)
            {
                let child_errors = body.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected body to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // while_keyword

        {
            let while_keyword = &self.while_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::WhileKeyword)
            {
                let child_errors = while_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected while_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // open_paren

        {
            let open_paren = &self.open_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenParen)
            {
                let child_errors = open_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // condition

        {
            let condition = &self.condition;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Condition)
            {
                let child_errors = condition.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected condition to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseParen)
            {
                let child_errors = close_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Semicolon)
            {
                let child_errors = semicolon.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected semicolon to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ElseBranch<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ElseBranch) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ElseBranch,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // else_keyword

        {
            let else_keyword = &self.else_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::ElseKeyword)
            {
                let child_errors = else_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected else_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // body

        {
            let body = &self.body;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Body)
            {
                let child_errors = body.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected body to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for EmitStatement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::EmitStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::EmitStatement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // emit_keyword

        {
            let emit_keyword = &self.emit_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::EmitKeyword)
            {
                let child_errors = emit_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected emit_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // event

        {
            let event = &self.event;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Event)
            {
                let child_errors = event.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected event to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // arguments

        {
            let arguments = &self.arguments;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Arguments)
            {
                let child_errors = arguments.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected arguments to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Semicolon)
            {
                let child_errors = semicolon.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected semicolon to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for EnumDefinition<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::EnumDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::EnumDefinition,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // enum_keyword

        {
            let enum_keyword = &self.enum_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::EnumKeyword)
            {
                let child_errors = enum_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected enum_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // name

        {
            let name = &self.name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // open_brace

        {
            let open_brace = &self.open_brace;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenBrace)
            {
                let child_errors = open_brace.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_brace to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // members

        {
            let members = &self.members;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Members)
            {
                let child_errors = members.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected members to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_brace

        {
            let close_brace = &self.close_brace;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseBrace)
            {
                let child_errors = close_brace.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_brace to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for EqualityExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::EqualityExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::EqualityExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // left_operand

        {
            let left_operand = &self.left_operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::LeftOperand)
            {
                let child_errors = left_operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected left_operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // Expression_EqualityExpression_Operator

        {
            let expression_equality_expression_operator =
                &self.expression_equality_expression_operator;

            // Prepare edge label

            // Special case for operator fields that are merged together

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Operator)
            {
                let child_errors = expression_equality_expression_operator
                    .check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new("Expected Expression_EqualityExpression_Operator to be present in the CST, but it was not".to_string(), node_range.clone()));
            }
        }

        // right_operand

        {
            let right_operand = &self.right_operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::RightOperand)
            {
                let child_errors = right_operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected right_operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ErrorDefinition<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ErrorDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ErrorDefinition,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // error_keyword

        {
            let error_keyword = &self.error_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::ErrorKeyword)
            {
                let child_errors = error_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected error_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // name

        {
            let name = &self.name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // members

        {
            let members = &self.members;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Members)
            {
                let child_errors = members.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected members to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Semicolon)
            {
                let child_errors = semicolon.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected semicolon to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ErrorParameter<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ErrorParameter) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ErrorParameter,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // type_name

        {
            let type_name = &self.type_name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::TypeName)
            {
                let child_errors = type_name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected type_name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // name
        if let Some(name) = &self.name {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Name) {
                errors.push(NodeCheckerError::new(
                    format!(
                        "Expected name to not be present in the CST, but it was there: {child:#?}"
                    ),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ErrorParametersDeclaration<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ErrorParametersDeclaration) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ErrorParametersDeclaration,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // open_paren

        {
            let open_paren = &self.open_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenParen)
            {
                let child_errors = open_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Parameters)
            {
                let child_errors = parameters.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected parameters to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseParen)
            {
                let child_errors = close_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for EventDefinition<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::EventDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::EventDefinition,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // event_keyword

        {
            let event_keyword = &self.event_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::EventKeyword)
            {
                let child_errors = event_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected event_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // name

        {
            let name = &self.name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Parameters)
            {
                let child_errors = parameters.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected parameters to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // anonymous_keyword
        if let Some(anonymous_keyword) = &self.anonymous_keyword {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::AnonymousKeyword)
            {
                let child_errors =
                    anonymous_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected anonymous_keyword to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) =
                extract_first_with_label(&mut children, EdgeLabel::AnonymousKeyword)
            {
                errors.push(NodeCheckerError::new(format!("Expected anonymous_keyword to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Semicolon)
            {
                let child_errors = semicolon.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected semicolon to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for EventParameter<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::EventParameter) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::EventParameter,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // type_name

        {
            let type_name = &self.type_name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::TypeName)
            {
                let child_errors = type_name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected type_name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // indexed_keyword
        if let Some(indexed_keyword) = &self.indexed_keyword {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::IndexedKeyword)
            {
                let child_errors =
                    indexed_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected indexed_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) =
                extract_first_with_label(&mut children, EdgeLabel::IndexedKeyword)
            {
                errors.push(NodeCheckerError::new(format!("Expected indexed_keyword to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // name
        if let Some(name) = &self.name {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Name) {
                errors.push(NodeCheckerError::new(
                    format!(
                        "Expected name to not be present in the CST, but it was there: {child:#?}"
                    ),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for EventParametersDeclaration<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::EventParametersDeclaration) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::EventParametersDeclaration,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // open_paren

        {
            let open_paren = &self.open_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenParen)
            {
                let child_errors = open_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Parameters)
            {
                let child_errors = parameters.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected parameters to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseParen)
            {
                let child_errors = close_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ExperimentalPragma<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ExperimentalPragma) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ExperimentalPragma,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // experimental_keyword

        {
            let experimental_keyword = &self.experimental_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::ExperimentalKeyword)
            {
                let child_errors =
                    experimental_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected experimental_keyword to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        }

        // feature

        {
            let feature = &self.feature;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Feature)
            {
                let child_errors = feature.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected feature to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ExponentiationExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ExponentiationExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ExponentiationExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // left_operand

        {
            let left_operand = &self.left_operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::LeftOperand)
            {
                let child_errors = left_operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected left_operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // Expression_ExponentiationExpression_Operator

        {
            let expression_exponentiation_expression_operator =
                &self.expression_exponentiation_expression_operator;

            // Prepare edge label

            // Special case for operator fields that are merged together

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Operator)
            {
                let child_errors = expression_exponentiation_expression_operator
                    .check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new("Expected Expression_ExponentiationExpression_Operator to be present in the CST, but it was not".to_string(), node_range.clone()));
            }
        }

        // right_operand

        {
            let right_operand = &self.right_operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::RightOperand)
            {
                let child_errors = right_operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected right_operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ExpressionStatement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ExpressionStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ExpressionStatement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // expression

        {
            let expression = &self.expression;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Expression)
            {
                let child_errors = expression.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected expression to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Semicolon)
            {
                let child_errors = semicolon.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected semicolon to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for FallbackFunctionDefinition<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FallbackFunctionDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::FallbackFunctionDefinition,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // fallback_keyword

        {
            let fallback_keyword = &self.fallback_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::FallbackKeyword)
            {
                let child_errors =
                    fallback_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected fallback_keyword to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Parameters)
            {
                let child_errors = parameters.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected parameters to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // attributes

        {
            let attributes = &self.attributes;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Attributes)
            {
                let child_errors = attributes.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected attributes to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // returns
        if let Some(returns) = &self.returns {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Returns)
            {
                let child_errors = returns.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected returns to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Returns) {
                errors.push(NodeCheckerError::new(format!("Expected returns to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // body

        {
            let body = &self.body;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Body)
            {
                let child_errors = body.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected body to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ForStatement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ForStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ForStatement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // for_keyword

        {
            let for_keyword = &self.for_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::ForKeyword)
            {
                let child_errors = for_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected for_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // open_paren

        {
            let open_paren = &self.open_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenParen)
            {
                let child_errors = open_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // initialization

        {
            let initialization = &self.initialization;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Initialization)
            {
                let child_errors = initialization.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected initialization to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // condition

        {
            let condition = &self.condition;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Condition)
            {
                let child_errors = condition.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected condition to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // iterator
        if let Some(iterator) = &self.iterator {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Iterator)
            {
                let child_errors = iterator.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected iterator to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Iterator) {
                errors.push(NodeCheckerError::new(format!("Expected iterator to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseParen)
            {
                let child_errors = close_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // body

        {
            let body = &self.body;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Body)
            {
                let child_errors = body.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected body to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for FunctionCallExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FunctionCallExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::FunctionCallExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // operand

        {
            let operand = &self.operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Operand)
            {
                let child_errors = operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // arguments

        {
            let arguments = &self.arguments;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Arguments)
            {
                let child_errors = arguments.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected arguments to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for FunctionDefinition<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FunctionDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::FunctionDefinition,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // function_keyword

        {
            let function_keyword = &self.function_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::FunctionKeyword)
            {
                let child_errors =
                    function_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected function_keyword to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        }

        // name

        {
            let name = &self.name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Parameters)
            {
                let child_errors = parameters.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected parameters to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // attributes

        {
            let attributes = &self.attributes;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Attributes)
            {
                let child_errors = attributes.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected attributes to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // returns
        if let Some(returns) = &self.returns {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Returns)
            {
                let child_errors = returns.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected returns to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Returns) {
                errors.push(NodeCheckerError::new(format!("Expected returns to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // body

        {
            let body = &self.body;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Body)
            {
                let child_errors = body.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected body to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for FunctionType<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FunctionType) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::FunctionType,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // function_keyword

        {
            let function_keyword = &self.function_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::FunctionKeyword)
            {
                let child_errors =
                    function_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected function_keyword to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Parameters)
            {
                let child_errors = parameters.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected parameters to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // attributes

        {
            let attributes = &self.attributes;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Attributes)
            {
                let child_errors = attributes.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected attributes to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // returns
        if let Some(returns) = &self.returns {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Returns)
            {
                let child_errors = returns.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected returns to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Returns) {
                errors.push(NodeCheckerError::new(format!("Expected returns to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for HexNumberExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::HexNumberExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::HexNumberExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // literal

        {
            let literal = &self.literal;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Literal)
            {
                let child_errors = literal.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected literal to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // unit
        if let Some(unit) = &self.unit {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Unit)
            {
                let child_errors = unit.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected unit to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Unit) {
                errors.push(NodeCheckerError::new(
                    format!(
                        "Expected unit to not be present in the CST, but it was there: {child:#?}"
                    ),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// NodeChecker for IdentifierPath is done by hand, since the V2 representation is a bit different
impl<'arena> NodeChecker for IdentifierPath<'arena> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::IdentifierPath) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::IdentifierPath,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        let tail_len = self
            .tail
            .as_ref()
            .map_or(0, |tail| tail.elements.elements.len());

        if children.len() != 1 + tail_len {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    1 + tail_len,
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            if i == 0 {
                // head
                let head = &self.head;
                let child_errors = head.check_node_with_offset(&child.node, *child_offset);
                errors.extend(child_errors);
                continue;
            }
            // tail elements

            let element = &self.tail.as_ref().unwrap().elements.elements[i - 1];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

// We skip NodeChecker for IdentifierPathTail, it's handled by IdentifierPath

/// Generic `NodeChecker` for sequences
impl NodeChecker for IfStatement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::IfStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::IfStatement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // if_keyword

        {
            let if_keyword = &self.if_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::IfKeyword)
            {
                let child_errors = if_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected if_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // open_paren

        {
            let open_paren = &self.open_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenParen)
            {
                let child_errors = open_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // condition

        {
            let condition = &self.condition;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Condition)
            {
                let child_errors = condition.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected condition to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseParen)
            {
                let child_errors = close_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // body

        {
            let body = &self.body;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Body)
            {
                let child_errors = body.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected body to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // else_branch
        if let Some(else_branch) = &self.else_branch {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::ElseBranch)
            {
                let child_errors = else_branch.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected else_branch to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::ElseBranch)
            {
                errors.push(NodeCheckerError::new(format!("Expected else_branch to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ImportAlias<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ImportAlias) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ImportAlias,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // as_keyword

        {
            let as_keyword = &self.as_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::AsKeyword)
            {
                let child_errors = as_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected as_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // identifier

        {
            let identifier = &self.identifier;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Identifier)
            {
                let child_errors = identifier.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected identifier to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ImportDeconstruction<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ImportDeconstruction) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ImportDeconstruction,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // open_brace

        {
            let open_brace = &self.open_brace;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenBrace)
            {
                let child_errors = open_brace.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_brace to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // symbols

        {
            let symbols = &self.symbols;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Symbols)
            {
                let child_errors = symbols.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected symbols to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_brace

        {
            let close_brace = &self.close_brace;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseBrace)
            {
                let child_errors = close_brace.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_brace to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // from_keyword

        {
            let from_keyword = &self.from_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::FromKeyword)
            {
                let child_errors = from_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected from_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // path

        {
            let path = &self.path;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Path)
            {
                let child_errors = path.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected path to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ImportDeconstructionSymbol<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ImportDeconstructionSymbol) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ImportDeconstructionSymbol,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // name

        {
            let name = &self.name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // alias
        if let Some(alias) = &self.alias {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Alias)
            {
                let child_errors = alias.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected alias to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Alias) {
                errors.push(NodeCheckerError::new(
                    format!(
                        "Expected alias to not be present in the CST, but it was there: {child:#?}"
                    ),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ImportDirective<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ImportDirective) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ImportDirective,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // import_keyword

        {
            let import_keyword = &self.import_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::ImportKeyword)
            {
                let child_errors = import_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected import_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // clause

        {
            let clause = &self.clause;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Clause)
            {
                let child_errors = clause.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected clause to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Semicolon)
            {
                let child_errors = semicolon.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected semicolon to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for IndexAccessEnd<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::IndexAccessEnd) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::IndexAccessEnd,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // colon

        {
            let colon = &self.colon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Colon)
            {
                let child_errors = colon.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected colon to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // end
        if let Some(end) = &self.end {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::End)
            {
                let child_errors = end.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected end to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::End) {
                errors.push(NodeCheckerError::new(
                    format!(
                        "Expected end to not be present in the CST, but it was there: {child:#?}"
                    ),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for IndexAccessExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::IndexAccessExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::IndexAccessExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // operand

        {
            let operand = &self.operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Operand)
            {
                let child_errors = operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // open_bracket

        {
            let open_bracket = &self.open_bracket;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenBracket)
            {
                let child_errors = open_bracket.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_bracket to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // start
        if let Some(start) = &self.start {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Start)
            {
                let child_errors = start.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected start to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Start) {
                errors.push(NodeCheckerError::new(
                    format!(
                        "Expected start to not be present in the CST, but it was there: {child:#?}"
                    ),
                    node_range.clone(),
                ));
            }
        }

        // end
        if let Some(end) = &self.end {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::End)
            {
                let child_errors = end.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected end to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::End) {
                errors.push(NodeCheckerError::new(
                    format!(
                        "Expected end to not be present in the CST, but it was there: {child:#?}"
                    ),
                    node_range.clone(),
                ));
            }
        }

        // close_bracket

        {
            let close_bracket = &self.close_bracket;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseBracket)
            {
                let child_errors = close_bracket.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_bracket to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for InequalityExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::InequalityExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::InequalityExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // left_operand

        {
            let left_operand = &self.left_operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::LeftOperand)
            {
                let child_errors = left_operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected left_operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // Expression_InequalityExpression_Operator

        {
            let expression_inequality_expression_operator =
                &self.expression_inequality_expression_operator;

            // Prepare edge label

            // Special case for operator fields that are merged together

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Operator)
            {
                let child_errors = expression_inequality_expression_operator
                    .check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new("Expected Expression_InequalityExpression_Operator to be present in the CST, but it was not".to_string(), node_range.clone()));
            }
        }

        // right_operand

        {
            let right_operand = &self.right_operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::RightOperand)
            {
                let child_errors = right_operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected right_operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for InheritanceSpecifier<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::InheritanceSpecifier) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::InheritanceSpecifier,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // is_keyword

        {
            let is_keyword = &self.is_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::IsKeyword)
            {
                let child_errors = is_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected is_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // types

        {
            let types = &self.types;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Types)
            {
                let child_errors = types.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected types to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for InheritanceType<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::InheritanceType) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::InheritanceType,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // type_name

        {
            let type_name = &self.type_name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::TypeName)
            {
                let child_errors = type_name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected type_name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // arguments
        if let Some(arguments) = &self.arguments {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Arguments)
            {
                let child_errors = arguments.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected arguments to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Arguments)
            {
                errors.push(NodeCheckerError::new(format!("Expected arguments to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for InterfaceDefinition<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::InterfaceDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::InterfaceDefinition,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // interface_keyword

        {
            let interface_keyword = &self.interface_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::InterfaceKeyword)
            {
                let child_errors =
                    interface_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected interface_keyword to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        }

        // name

        {
            let name = &self.name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // inheritance
        if let Some(inheritance) = &self.inheritance {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Inheritance)
            {
                let child_errors = inheritance.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected inheritance to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) =
                extract_first_with_label(&mut children, EdgeLabel::Inheritance)
            {
                errors.push(NodeCheckerError::new(format!("Expected inheritance to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // open_brace

        {
            let open_brace = &self.open_brace;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenBrace)
            {
                let child_errors = open_brace.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_brace to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // members

        {
            let members = &self.members;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Members)
            {
                let child_errors = members.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected members to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_brace

        {
            let close_brace = &self.close_brace;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseBrace)
            {
                let child_errors = close_brace.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_brace to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for LibraryDefinition<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::LibraryDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::LibraryDefinition,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // library_keyword

        {
            let library_keyword = &self.library_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::LibraryKeyword)
            {
                let child_errors =
                    library_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected library_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // name

        {
            let name = &self.name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // open_brace

        {
            let open_brace = &self.open_brace;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenBrace)
            {
                let child_errors = open_brace.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_brace to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // members

        {
            let members = &self.members;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Members)
            {
                let child_errors = members.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected members to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_brace

        {
            let close_brace = &self.close_brace;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseBrace)
            {
                let child_errors = close_brace.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_brace to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for MappingKey<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::MappingKey) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::MappingKey,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // key_type

        {
            let key_type = &self.key_type;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::KeyType)
            {
                let child_errors = key_type.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected key_type to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // name
        if let Some(name) = &self.name {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Name) {
                errors.push(NodeCheckerError::new(
                    format!(
                        "Expected name to not be present in the CST, but it was there: {child:#?}"
                    ),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for MappingType<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::MappingType) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::MappingType,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // mapping_keyword

        {
            let mapping_keyword = &self.mapping_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::MappingKeyword)
            {
                let child_errors =
                    mapping_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected mapping_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // open_paren

        {
            let open_paren = &self.open_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenParen)
            {
                let child_errors = open_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // key_type

        {
            let key_type = &self.key_type;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::KeyType)
            {
                let child_errors = key_type.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected key_type to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // equal_greater_than

        {
            let equal_greater_than = &self.equal_greater_than;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::EqualGreaterThan)
            {
                let child_errors =
                    equal_greater_than.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected equal_greater_than to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        }

        // value_type

        {
            let value_type = &self.value_type;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::ValueType)
            {
                let child_errors = value_type.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected value_type to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseParen)
            {
                let child_errors = close_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for MappingValue<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::MappingValue) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::MappingValue,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // type_name

        {
            let type_name = &self.type_name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::TypeName)
            {
                let child_errors = type_name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected type_name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // name
        if let Some(name) = &self.name {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Name) {
                errors.push(NodeCheckerError::new(
                    format!(
                        "Expected name to not be present in the CST, but it was there: {child:#?}"
                    ),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for MemberAccessExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::MemberAccessExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::MemberAccessExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // operand

        {
            let operand = &self.operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Operand)
            {
                let child_errors = operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // period

        {
            let period = &self.period;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Period)
            {
                let child_errors = period.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected period to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // member

        {
            let member = &self.member;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Member)
            {
                let child_errors = member.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected member to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ModifierDefinition<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ModifierDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ModifierDefinition,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // modifier_keyword

        {
            let modifier_keyword = &self.modifier_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::ModifierKeyword)
            {
                let child_errors =
                    modifier_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected modifier_keyword to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        }

        // name

        {
            let name = &self.name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // parameters
        if let Some(parameters) = &self.parameters {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Parameters)
            {
                let child_errors = parameters.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected parameters to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Parameters)
            {
                errors.push(NodeCheckerError::new(format!("Expected parameters to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // attributes

        {
            let attributes = &self.attributes;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Attributes)
            {
                let child_errors = attributes.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected attributes to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // body

        {
            let body = &self.body;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Body)
            {
                let child_errors = body.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected body to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ModifierInvocation<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ModifierInvocation) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ModifierInvocation,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // name

        {
            let name = &self.name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // arguments
        if let Some(arguments) = &self.arguments {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Arguments)
            {
                let child_errors = arguments.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected arguments to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Arguments)
            {
                errors.push(NodeCheckerError::new(format!("Expected arguments to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for MultiplicativeExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::MultiplicativeExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::MultiplicativeExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // left_operand

        {
            let left_operand = &self.left_operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::LeftOperand)
            {
                let child_errors = left_operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected left_operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // Expression_MultiplicativeExpression_Operator

        {
            let expression_multiplicative_expression_operator =
                &self.expression_multiplicative_expression_operator;

            // Prepare edge label

            // Special case for operator fields that are merged together

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Operator)
            {
                let child_errors = expression_multiplicative_expression_operator
                    .check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new("Expected Expression_MultiplicativeExpression_Operator to be present in the CST, but it was not".to_string(), node_range.clone()));
            }
        }

        // right_operand

        {
            let right_operand = &self.right_operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::RightOperand)
            {
                let child_errors = right_operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected right_operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for NamedArgument<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::NamedArgument) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::NamedArgument,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // name

        {
            let name = &self.name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // colon

        {
            let colon = &self.colon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Colon)
            {
                let child_errors = colon.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected colon to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // value

        {
            let value = &self.value;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Value)
            {
                let child_errors = value.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected value to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for NamedArgumentGroup<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::NamedArgumentGroup) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::NamedArgumentGroup,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // open_brace

        {
            let open_brace = &self.open_brace;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenBrace)
            {
                let child_errors = open_brace.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_brace to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // arguments

        {
            let arguments = &self.arguments;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Arguments)
            {
                let child_errors = arguments.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected arguments to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_brace

        {
            let close_brace = &self.close_brace;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseBrace)
            {
                let child_errors = close_brace.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_brace to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for NamedArgumentsDeclaration<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::NamedArgumentsDeclaration) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::NamedArgumentsDeclaration,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // open_paren

        {
            let open_paren = &self.open_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenParen)
            {
                let child_errors = open_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // arguments

        {
            let arguments = &self.arguments;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Arguments)
            {
                let child_errors = arguments.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected arguments to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseParen)
            {
                let child_errors = close_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for NamedImport<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::NamedImport) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::NamedImport,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // asterisk

        {
            let asterisk = &self.asterisk;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Asterisk)
            {
                let child_errors = asterisk.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected asterisk to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // alias

        {
            let alias = &self.alias;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Alias)
            {
                let child_errors = alias.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected alias to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // from_keyword

        {
            let from_keyword = &self.from_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::FromKeyword)
            {
                let child_errors = from_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected from_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // path

        {
            let path = &self.path;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Path)
            {
                let child_errors = path.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected path to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Helper to check a V2 CallOptionsNew against a V1 CallOptionsExpression
///
/// It extracts the braces and the options from v1_children, leaving the operand to the outer scope
fn check_call_options_new(
    v2_options: &CallOptionsNew<'_>,
    v1_node: &Node,
    v1_children: &mut Vec<(Edge, TextIndex)>,
    v1_offset: TextIndex,
) -> Vec<NodeCheckerError> {
    let node_range = v1_offset..(v1_offset + v1_node.text_len());
    let mut errors = vec![];

    // V1 CallOptionsExpression has: open_brace, options (CallOptions), close_brace
    // V2 CallOptionsNew has the same structure

    // open_brace
    {
        let open_brace = &v2_options.open_brace;
        if let Some((child, child_offset)) =
            extract_first_with_label(v1_children, EdgeLabel::OpenBrace)
        {
            errors.extend(open_brace.check_node_with_offset(&child.node, child_offset));
        } else {
            errors.push(NodeCheckerError::new(
                "Expected open_brace to be present in CallOptionsExpression".to_string(),
                node_range.clone(),
            ));
        }
    }

    // options (CallOptions)
    {
        let options = &v2_options.options;
        if let Some((child, child_offset)) =
            extract_first_with_label(v1_children, EdgeLabel::Options)
        {
            errors.extend(options.check_node_with_offset(&child.node, child_offset));
        } else {
            errors.push(NodeCheckerError::new(
                "Expected options to be present in CallOptionsExpression".to_string(),
                node_range.clone(),
            ));
        }
    }

    // close_brace
    {
        let close_brace = &v2_options.close_brace;
        if let Some((child, child_offset)) =
            extract_first_with_label(v1_children, EdgeLabel::CloseBrace)
        {
            errors.extend(close_brace.check_node_with_offset(&child.node, child_offset));
        } else {
            errors.push(NodeCheckerError::new(
                "Expected close_brace to be present in CallOptionsExpression".to_string(),
                node_range.clone(),
            ));
        }
    }

    errors
}

/// Helper to unwrap an Expression node and get its inner variant node
fn unwrap_expression(
    node: &Node,
    offset: TextIndex,
) -> Result<(Node, TextIndex), NodeCheckerError> {
    let node_range = offset..(offset + node.text_len());

    if node.kind() != NodeKind::Nonterminal(NonterminalKind::Expression) {
        return Err(NodeCheckerError::new(
            format!("Expected Expression, but got {}", node.kind()),
            node_range,
        ));
    }

    let children = children_with_offsets(node, offset);
    if children.len() != 1 {
        return Err(NodeCheckerError::new(
            format!("Expected exactly one child for Expression, but got: {children:#?}"),
            node_range,
        ));
    }

    let (child, child_offset) = &children[0];
    if child.label != EdgeLabel::Variant {
        let child_range = *child_offset..(*child_offset + child.node.text_len());
        return Err(NodeCheckerError::new(
            format!(
                "Expected child to be of variant type, but it was {}",
                child.label
            ),
            child_range,
        ));
    }

    Ok((child.node.clone(), *child_offset))
}

/// NodeChecker for NewExpression is done by hand, since in V2 it includes the call options and arguments
///
/// In V1: FunctionCallExpression -> (CallOptionsExpression)* -> NewExpression
/// In V2: NewExpression contains new_keyword, type_name, options (MultipleCallOptionsNew), arguments
impl<'arena> NodeChecker for NewExpression<'arena> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        // What we parse as a NewExpression in V2 is parsed as a FunctionCallExpression in V1,
        // with optional CallOptionsExpression nodes in between, and a V1 NewExpression at the core
        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FunctionCallExpression) {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::FunctionCallExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);
        let mut errors = vec![];

        // Extract operand from FunctionCallExpression
        let (operand_node, operand_offset) = if let Some((child, child_offset)) =
            extract_first_with_label(&mut children, EdgeLabel::Operand)
        {
            (child.node, child_offset)
        } else {
            errors.push(NodeCheckerError::new(
                "Expected operand to be present in FunctionCallExpression".to_string(),
                node_range,
            ));
            return errors;
        };

        // Unwrap the Expression to get the variant
        let (mut current_node, mut current_offset) =
            match unwrap_expression(&operand_node, operand_offset) {
                Ok(result) => result,
                Err(e) => {
                    errors.push(e);
                    return errors;
                }
            };

        // Traverse through CallOptionsExpression nodes to find the NewExpression,
        // checking each one against V2's options as we go.
        //
        // V1 traversal order: outermost first (closest to the function call)
        // V2 storage order: source order (innermost first, closest to `new`)
        // So we iterate V2's options in reverse to match V1's traversal order.
        let mut v2_options_iter = self
            .options
            .as_ref()
            .map(|opts| opts.elements.iter().rev().peekable());
        let mut v1_call_options_count = 0usize;

        while current_node.kind() == NodeKind::Nonterminal(NonterminalKind::CallOptionsExpression) {
            v1_call_options_count += 1;
            let call_opts_range = current_offset..(current_offset + current_node.text_len());

            // Get the operand of this CallOptionsExpression
            let mut call_opts_children = children_with_offsets(&current_node, current_offset);

            // Check this V1 CallOptionsExpression against the corresponding V2 CallOptionsNew
            if let Some(ref mut iter) = v2_options_iter {
                if let Some(v2_opt) = iter.next() {
                    errors.extend(check_call_options_new(
                        v2_opt,
                        &current_node,
                        &mut call_opts_children,
                        current_offset,
                    ));
                } else {
                    // V1 has more options than V2
                    errors.push(NodeCheckerError::new(
                        "Found CallOptionsExpression in V1, but no corresponding option in V2"
                            .to_string(),
                        call_opts_range.clone(),
                    ));
                }
            } else {
                // V2 has no options at all, but V1 has some
                errors.push(NodeCheckerError::new(
                    "Found CallOptionsExpression in V1, but V2 has no options".to_string(),
                    call_opts_range.clone(),
                ));
            }

            if let Some((operand, op_offset)) =
                extract_first_with_label(&mut call_opts_children, EdgeLabel::Operand)
            {
                // Unwrap the operand Expression
                match unwrap_expression(&operand.node, op_offset) {
                    Ok((inner_node, inner_offset)) => {
                        current_node = inner_node;
                        current_offset = inner_offset;
                    }
                    Err(e) => {
                        errors.push(e);
                        // Continue to try to check the rest
                        break;
                    }
                }
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected operand in CallOptionsExpression".to_string(),
                    call_opts_range,
                ));
                // Continue to try to check the rest
                break;
            }

            if !call_opts_children.is_empty() {
                errors.push(NodeCheckerError::new(
                    format!("Expected 0 children left in CallOptionsExpression, but there's some left {call_opts_children:#?}"),
                    node_range.clone(),
                ));
            }
        }

        // Check if V2 has more options than V1
        if let Some(ref mut iter) = v2_options_iter {
            let remaining: Vec<_> = iter.collect();
            if !remaining.is_empty() {
                errors.push(NodeCheckerError::new(
                    format!(
                        "V2 has {} more call options than V1 (V1 had {}, V2 has {})",
                        remaining.len(),
                        v1_call_options_count,
                        v1_call_options_count + remaining.len()
                    ),
                    node_range.clone(),
                ));
            }
        }

        // Now current_node should be the V1 NewExpression
        let new_expression_node = current_node;
        let new_expression_offset = current_offset;
        let new_expression_range =
            new_expression_offset..(new_expression_offset + new_expression_node.text_len());

        if new_expression_node.kind() != NodeKind::Nonterminal(NonterminalKind::NewExpression) {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::NewExpression,
                    new_expression_node.kind()
                ),
                new_expression_range.clone(),
            ));
            // Don't return early, continue checking what we can
        }

        let mut new_expression_children =
            children_with_offsets(&new_expression_node, new_expression_offset);

        // new_keyword
        {
            let new_keyword = &self.new_keyword;
            if let Some((child, child_offset)) =
                extract_first_with_label(&mut new_expression_children, EdgeLabel::NewKeyword)
            {
                errors.extend(new_keyword.check_node_with_offset(&child.node, child_offset));
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected new_keyword to be present in NewExpression".to_string(),
                    new_expression_range.clone(),
                ));
            }
        }

        // type_name
        {
            let type_name = &self.type_name;
            if let Some((child, child_offset)) =
                extract_first_with_label(&mut new_expression_children, EdgeLabel::TypeName)
            {
                errors.extend(type_name.check_node_with_offset(&child.node, child_offset));
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected type_name to be present in NewExpression".to_string(),
                    new_expression_range.clone(),
                ));
            }
        }

        // arguments
        {
            let arguments = &self.arguments;
            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Arguments)
            {
                errors.extend(arguments.check_node_with_offset(&child.node, child_offset));
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected arguments to be present in FunctionCallExpression".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left in FunctionCallExpression, but there's some left {children:#?}",
            ), node_range.clone()));
        }

        if !new_expression_children.is_empty() {
            errors.push(NodeCheckerError::new(format!(
                "Expected 0 children left in NewExpression, but there's some left {new_expression_children:#?}",
            ), new_expression_range));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for OrExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::OrExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::OrExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // left_operand

        {
            let left_operand = &self.left_operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::LeftOperand)
            {
                let child_errors = left_operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected left_operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // operator

        {
            let operator = &self.operator;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Operator)
            {
                let child_errors = operator.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected operator to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // right_operand

        {
            let right_operand = &self.right_operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::RightOperand)
            {
                let child_errors = right_operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected right_operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for OverridePathsDeclaration<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::OverridePathsDeclaration) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::OverridePathsDeclaration,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // open_paren

        {
            let open_paren = &self.open_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenParen)
            {
                let child_errors = open_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // paths

        {
            let paths = &self.paths;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Paths)
            {
                let child_errors = paths.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected paths to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseParen)
            {
                let child_errors = close_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for OverrideSpecifier<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::OverrideSpecifier) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::OverrideSpecifier,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // override_keyword

        {
            let override_keyword = &self.override_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OverrideKeyword)
            {
                let child_errors =
                    override_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected override_keyword to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        }

        // overridden
        if let Some(overridden) = &self.overridden {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Overridden)
            {
                let child_errors = overridden.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected overridden to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Overridden)
            {
                errors.push(NodeCheckerError::new(format!("Expected overridden to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for Parameter<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::Parameter) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::Parameter,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // type_name

        {
            let type_name = &self.type_name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::TypeName)
            {
                let child_errors = type_name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected type_name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // storage_location
        if let Some(storage_location) = &self.storage_location {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::StorageLocation)
            {
                let child_errors =
                    storage_location.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected storage_location to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) =
                extract_first_with_label(&mut children, EdgeLabel::StorageLocation)
            {
                errors.push(NodeCheckerError::new(format!("Expected storage_location to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // name
        if let Some(name) = &self.name {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Name) {
                errors.push(NodeCheckerError::new(
                    format!(
                        "Expected name to not be present in the CST, but it was there: {child:#?}"
                    ),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ParametersDeclaration<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ParametersDeclaration) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ParametersDeclaration,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // open_paren

        {
            let open_paren = &self.open_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenParen)
            {
                let child_errors = open_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Parameters)
            {
                let child_errors = parameters.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected parameters to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseParen)
            {
                let child_errors = close_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for PathImport<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::PathImport) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::PathImport,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // path

        {
            let path = &self.path;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Path)
            {
                let child_errors = path.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected path to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // alias
        if let Some(alias) = &self.alias {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Alias)
            {
                let child_errors = alias.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected alias to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Alias) {
                errors.push(NodeCheckerError::new(
                    format!(
                        "Expected alias to not be present in the CST, but it was there: {child:#?}"
                    ),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for PositionalArgumentsDeclaration<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::PositionalArgumentsDeclaration) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::PositionalArgumentsDeclaration,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // open_paren

        {
            let open_paren = &self.open_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenParen)
            {
                let child_errors = open_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // arguments

        {
            let arguments = &self.arguments;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Arguments)
            {
                let child_errors = arguments.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected arguments to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseParen)
            {
                let child_errors = close_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for PostfixExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::PostfixExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::PostfixExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // operand

        {
            let operand = &self.operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Operand)
            {
                let child_errors = operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // Expression_PostfixExpression_Operator

        {
            let expression_postfix_expression_operator =
                &self.expression_postfix_expression_operator;

            // Prepare edge label

            // Special case for operator fields that are merged together

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Operator)
            {
                let child_errors = expression_postfix_expression_operator
                    .check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new("Expected Expression_PostfixExpression_Operator to be present in the CST, but it was not".to_string(), node_range.clone()));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

// TODO(v2): PragmaDirective ignores the pragma value for now until the Lexer can
// perform a context switch, therefore the checker ignores that edge
impl<'arena> NodeChecker for PragmaDirective<'arena> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::PragmaDirective) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::PragmaDirective,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // pragma_keyword

        {
            let pragma_keyword = &self.pragma_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::PragmaKeyword)
            {
                let child_errors = pragma_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    format!("Expected pragma_keyword to be present in the CST, but it was not"),
                    node_range.clone(),
                ));
            }
        }

        // pragma

        {
            // Prepare edge label

            if let Some(_) = extract_first_with_label(&mut children, EdgeLabel::Pragma) {
                // We don't check, since V2 can't parse these yet
            } else {
                errors.push(NodeCheckerError::new(
                    format!("Expected pragma to be present in the CST, but it was not"),
                    node_range.clone(),
                ));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Semicolon)
            {
                let child_errors = semicolon.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    format!("Expected semicolon to be present in the CST, but it was not"),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected 0 children left, but there's some left {:#?}",
                    children
                ),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for PrefixExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::PrefixExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::PrefixExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // Expression_PrefixExpression_Operator

        {
            let expression_prefix_expression_operator = &self.expression_prefix_expression_operator;

            // Prepare edge label

            // Special case for operator fields that are merged together

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Operator)
            {
                let child_errors = expression_prefix_expression_operator
                    .check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new("Expected Expression_PrefixExpression_Operator to be present in the CST, but it was not".to_string(), node_range.clone()));
            }
        }

        // operand

        {
            let operand = &self.operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Operand)
            {
                let child_errors = operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ReceiveFunctionDefinition<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ReceiveFunctionDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ReceiveFunctionDefinition,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // receive_keyword

        {
            let receive_keyword = &self.receive_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::ReceiveKeyword)
            {
                let child_errors =
                    receive_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected receive_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Parameters)
            {
                let child_errors = parameters.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected parameters to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // attributes

        {
            let attributes = &self.attributes;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Attributes)
            {
                let child_errors = attributes.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected attributes to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // body

        {
            let body = &self.body;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Body)
            {
                let child_errors = body.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected body to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ReturnStatement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ReturnStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ReturnStatement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // return_keyword

        {
            let return_keyword = &self.return_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::ReturnKeyword)
            {
                let child_errors = return_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected return_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // expression
        if let Some(expression) = &self.expression {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Expression)
            {
                let child_errors = expression.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected expression to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Expression)
            {
                errors.push(NodeCheckerError::new(format!("Expected expression to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Semicolon)
            {
                let child_errors = semicolon.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected semicolon to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ReturnsDeclaration<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ReturnsDeclaration) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ReturnsDeclaration,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // returns_keyword

        {
            let returns_keyword = &self.returns_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::ReturnsKeyword)
            {
                let child_errors =
                    returns_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected returns_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // variables

        {
            let variables = &self.variables;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Variables)
            {
                let child_errors = variables.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected variables to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for RevertStatement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::RevertStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::RevertStatement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // revert_keyword

        {
            let revert_keyword = &self.revert_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::RevertKeyword)
            {
                let child_errors = revert_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected revert_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // error

        {
            let error = &self.error;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Error)
            {
                let child_errors = error.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected error to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // arguments

        {
            let arguments = &self.arguments;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Arguments)
            {
                let child_errors = arguments.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected arguments to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Semicolon)
            {
                let child_errors = semicolon.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected semicolon to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ShiftExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ShiftExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ShiftExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // left_operand

        {
            let left_operand = &self.left_operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::LeftOperand)
            {
                let child_errors = left_operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected left_operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // Expression_ShiftExpression_Operator

        {
            let expression_shift_expression_operator = &self.expression_shift_expression_operator;

            // Prepare edge label

            // Special case for operator fields that are merged together

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Operator)
            {
                let child_errors = expression_shift_expression_operator
                    .check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new("Expected Expression_ShiftExpression_Operator to be present in the CST, but it was not".to_string(), node_range.clone()));
            }
        }

        // right_operand

        {
            let right_operand = &self.right_operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::RightOperand)
            {
                let child_errors = right_operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected right_operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for SourceUnit<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::SourceUnit) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::SourceUnit,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // members

        {
            let members = &self.members;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Members)
            {
                let child_errors = members.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected members to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for StateVariableDefinition<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::StateVariableDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::StateVariableDefinition,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // type_name

        {
            let type_name = &self.type_name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::TypeName)
            {
                let child_errors = type_name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected type_name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // attributes

        {
            let attributes = &self.attributes;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Attributes)
            {
                let child_errors = attributes.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected attributes to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // name

        {
            let name = &self.name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // value
        if let Some(value) = &self.value {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Value)
            {
                let child_errors = value.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected value to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Value) {
                errors.push(NodeCheckerError::new(
                    format!(
                        "Expected value to not be present in the CST, but it was there: {child:#?}"
                    ),
                    node_range.clone(),
                ));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Semicolon)
            {
                let child_errors = semicolon.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected semicolon to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for StateVariableDefinitionValue<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::StateVariableDefinitionValue) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::StateVariableDefinitionValue,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // equal

        {
            let equal = &self.equal;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Equal)
            {
                let child_errors = equal.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected equal to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // value

        {
            let value = &self.value;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Value)
            {
                let child_errors = value.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected value to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for StorageLayoutSpecifier<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::StorageLayoutSpecifier) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::StorageLayoutSpecifier,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // layout_keyword

        {
            let layout_keyword = &self.layout_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::LayoutKeyword)
            {
                let child_errors = layout_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected layout_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // at_keyword

        {
            let at_keyword = &self.at_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::AtKeyword)
            {
                let child_errors = at_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected at_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // expression

        {
            let expression = &self.expression;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Expression)
            {
                let child_errors = expression.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected expression to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for StructDefinition<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::StructDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::StructDefinition,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // struct_keyword

        {
            let struct_keyword = &self.struct_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::StructKeyword)
            {
                let child_errors = struct_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected struct_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // name

        {
            let name = &self.name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // open_brace

        {
            let open_brace = &self.open_brace;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenBrace)
            {
                let child_errors = open_brace.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_brace to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // members

        {
            let members = &self.members;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Members)
            {
                let child_errors = members.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected members to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_brace

        {
            let close_brace = &self.close_brace;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseBrace)
            {
                let child_errors = close_brace.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_brace to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for StructMember<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::StructMember) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::StructMember,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // type_name

        {
            let type_name = &self.type_name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::TypeName)
            {
                let child_errors = type_name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected type_name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // name

        {
            let name = &self.name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Semicolon)
            {
                let child_errors = semicolon.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected semicolon to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for ThrowStatement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ThrowStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ThrowStatement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // throw_keyword

        {
            let throw_keyword = &self.throw_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::ThrowKeyword)
            {
                let child_errors = throw_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected throw_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Semicolon)
            {
                let child_errors = semicolon.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected semicolon to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for TryStatement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TryStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::TryStatement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // try_keyword

        {
            let try_keyword = &self.try_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::TryKeyword)
            {
                let child_errors = try_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected try_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // expression

        {
            let expression = &self.expression;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Expression)
            {
                let child_errors = expression.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected expression to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // returns
        if let Some(returns) = &self.returns {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Returns)
            {
                let child_errors = returns.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected returns to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Returns) {
                errors.push(NodeCheckerError::new(format!("Expected returns to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // body

        {
            let body = &self.body;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Body)
            {
                let child_errors = body.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected body to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // catch_clauses

        {
            let catch_clauses = &self.catch_clauses;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CatchClauses)
            {
                let child_errors = catch_clauses.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected catch_clauses to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for TupleDeconstructionElement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TupleDeconstructionElement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::TupleDeconstructionElement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // member
        if let Some(member) = &self.member {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Member)
            {
                let child_errors = member.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected member to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Member) {
                errors.push(NodeCheckerError::new(format!("Expected member to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for TupleDeconstructionStatement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TupleDeconstructionStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::TupleDeconstructionStatement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // var_keyword
        if let Some(var_keyword) = &self.var_keyword {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::VarKeyword)
            {
                let child_errors = var_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected var_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::VarKeyword)
            {
                errors.push(NodeCheckerError::new(format!("Expected var_keyword to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // open_paren

        {
            let open_paren = &self.open_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenParen)
            {
                let child_errors = open_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // elements

        {
            let elements = &self.elements;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Elements)
            {
                let child_errors = elements.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected elements to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseParen)
            {
                let child_errors = close_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // equal

        {
            let equal = &self.equal;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Equal)
            {
                let child_errors = equal.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected equal to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // expression

        {
            let expression = &self.expression;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Expression)
            {
                let child_errors = expression.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected expression to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Semicolon)
            {
                let child_errors = semicolon.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected semicolon to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for TupleExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TupleExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::TupleExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // open_paren

        {
            let open_paren = &self.open_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenParen)
            {
                let child_errors = open_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // items

        {
            let items = &self.items;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Items)
            {
                let child_errors = items.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected items to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseParen)
            {
                let child_errors = close_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for TupleValue<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TupleValue) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::TupleValue,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // expression
        if let Some(expression) = &self.expression {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Expression)
            {
                let child_errors = expression.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected expression to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Expression)
            {
                errors.push(NodeCheckerError::new(format!("Expected expression to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for TypeExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TypeExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::TypeExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // type_keyword

        {
            let type_keyword = &self.type_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::TypeKeyword)
            {
                let child_errors = type_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected type_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // open_paren

        {
            let open_paren = &self.open_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenParen)
            {
                let child_errors = open_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // type_name

        {
            let type_name = &self.type_name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::TypeName)
            {
                let child_errors = type_name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected type_name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseParen)
            {
                let child_errors = close_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for TypedTupleMember<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TypedTupleMember) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::TypedTupleMember,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // type_name

        {
            let type_name = &self.type_name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::TypeName)
            {
                let child_errors = type_name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected type_name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // storage_location
        if let Some(storage_location) = &self.storage_location {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::StorageLocation)
            {
                let child_errors =
                    storage_location.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected storage_location to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) =
                extract_first_with_label(&mut children, EdgeLabel::StorageLocation)
            {
                errors.push(NodeCheckerError::new(format!("Expected storage_location to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // name

        {
            let name = &self.name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for UncheckedBlock<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UncheckedBlock) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::UncheckedBlock,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // unchecked_keyword

        {
            let unchecked_keyword = &self.unchecked_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::UncheckedKeyword)
            {
                let child_errors =
                    unchecked_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected unchecked_keyword to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        }

        // block

        {
            let block = &self.block;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Block)
            {
                let child_errors = block.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected block to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for UnnamedFunctionDefinition<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UnnamedFunctionDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::UnnamedFunctionDefinition,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // function_keyword

        {
            let function_keyword = &self.function_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::FunctionKeyword)
            {
                let child_errors =
                    function_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected function_keyword to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Parameters)
            {
                let child_errors = parameters.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected parameters to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // attributes

        {
            let attributes = &self.attributes;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Attributes)
            {
                let child_errors = attributes.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected attributes to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // body

        {
            let body = &self.body;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Body)
            {
                let child_errors = body.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected body to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for UntypedTupleMember<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UntypedTupleMember) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::UntypedTupleMember,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // storage_location
        if let Some(storage_location) = &self.storage_location {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::StorageLocation)
            {
                let child_errors =
                    storage_location.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected storage_location to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) =
                extract_first_with_label(&mut children, EdgeLabel::StorageLocation)
            {
                errors.push(NodeCheckerError::new(format!("Expected storage_location to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // name

        {
            let name = &self.name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for UserDefinedValueTypeDefinition<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UserDefinedValueTypeDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::UserDefinedValueTypeDefinition,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // type_keyword

        {
            let type_keyword = &self.type_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::TypeKeyword)
            {
                let child_errors = type_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected type_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // name

        {
            let name = &self.name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // is_keyword

        {
            let is_keyword = &self.is_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::IsKeyword)
            {
                let child_errors = is_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected is_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // value_type

        {
            let value_type = &self.value_type;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::ValueType)
            {
                let child_errors = value_type.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected value_type to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Semicolon)
            {
                let child_errors = semicolon.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected semicolon to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for UsingAlias<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UsingAlias) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::UsingAlias,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // as_keyword

        {
            let as_keyword = &self.as_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::AsKeyword)
            {
                let child_errors = as_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected as_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // operator

        {
            let operator = &self.operator;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Operator)
            {
                let child_errors = operator.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected operator to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for UsingDeconstruction<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UsingDeconstruction) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::UsingDeconstruction,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // open_brace

        {
            let open_brace = &self.open_brace;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenBrace)
            {
                let child_errors = open_brace.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_brace to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // symbols

        {
            let symbols = &self.symbols;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Symbols)
            {
                let child_errors = symbols.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected symbols to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_brace

        {
            let close_brace = &self.close_brace;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseBrace)
            {
                let child_errors = close_brace.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_brace to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for UsingDeconstructionSymbol<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UsingDeconstructionSymbol) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::UsingDeconstructionSymbol,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // name

        {
            let name = &self.name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // alias
        if let Some(alias) = &self.alias {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Alias)
            {
                let child_errors = alias.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected alias to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Alias) {
                errors.push(NodeCheckerError::new(
                    format!(
                        "Expected alias to not be present in the CST, but it was there: {child:#?}"
                    ),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for UsingDirective<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UsingDirective) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::UsingDirective,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // using_keyword

        {
            let using_keyword = &self.using_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::UsingKeyword)
            {
                let child_errors = using_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected using_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // clause

        {
            let clause = &self.clause;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Clause)
            {
                let child_errors = clause.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected clause to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // for_keyword

        {
            let for_keyword = &self.for_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::ForKeyword)
            {
                let child_errors = for_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected for_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // target

        {
            let target = &self.target;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Target)
            {
                let child_errors = target.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected target to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // global_keyword
        if let Some(global_keyword) = &self.global_keyword {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::GlobalKeyword)
            {
                let child_errors = global_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected global_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) =
                extract_first_with_label(&mut children, EdgeLabel::GlobalKeyword)
            {
                errors.push(NodeCheckerError::new(format!("Expected global_keyword to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Semicolon)
            {
                let child_errors = semicolon.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected semicolon to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

// We skip NodeChecker for VariableDeclaration, it's handled by VariableDeclarationStatement

/// NodeChecker for VariableDeclarationStatement is done by hand, since in V2 is represented differently
impl NodeChecker for VariableDeclarationStatement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::VariableDeclarationStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::VariableDeclarationStatement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);
        let mut errors = vec![];

        // variable_declaration

        // variable_type
        {
            let variable_type = &self.variable_declaration.variable_type;
            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::VariableType)
            {
                let child_errors = variable_type.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    format!("Expected variable_type to be present in the CST, but it was not"),
                    node_range.clone(),
                ));
            }
        }

        // storage_location
        if let Some(storage_location) = &self.variable_declaration.storage_location {
            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::StorageLocation)
            {
                let child_errors =
                    storage_location.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    format!("Expected storage_location to be present in the CST, but it was not"),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(_) = extract_first_with_label(&mut children, EdgeLabel::StorageLocation) {
                errors.push(NodeCheckerError::new(
                    format!(
                        "Expected storage_location to not be present in the CST, but it was there"
                    ),
                    node_range.clone(),
                ));
            }
        }

        // name

        {
            let name = &self.variable_declaration.name;
            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    format!("Expected name to be present in the CST, but it was not"),
                    node_range.clone(),
                ));
            }
        }

        // value
        if let Some(value) = &self.value {
            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Value)
            {
                let child_errors = value.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    format!("Expected value to be present in the CST, but it was not"),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some(_) = extract_first_with_label(&mut children, EdgeLabel::Value) {
                errors.push(NodeCheckerError::new(
                    format!("Expected value to not be present in the CST, but it was there"),
                    node_range.clone(),
                ));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;
            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Semicolon)
            {
                let child_errors = semicolon.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    format!("Expected semicolon to be present in the CST, but it was not"),
                    node_range.clone(),
                ));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for VariableDeclarationValue<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::VariableDeclarationValue) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::VariableDeclarationValue,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // equal

        {
            let equal = &self.equal;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Equal)
            {
                let child_errors = equal.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected equal to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // expression

        {
            let expression = &self.expression;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Expression)
            {
                let child_errors = expression.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected expression to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for VersionPragma<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::VersionPragma) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::VersionPragma,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // solidity_keyword

        {
            let solidity_keyword = &self.solidity_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::SolidityKeyword)
            {
                let child_errors =
                    solidity_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected solidity_keyword to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        }

        // sets

        {
            let sets = &self.sets;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Sets)
            {
                let child_errors = sets.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected sets to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for VersionRange<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::VersionRange) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::VersionRange,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // start

        {
            let start = &self.start;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Start)
            {
                let child_errors = start.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected start to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // minus

        {
            let minus = &self.minus;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Minus)
            {
                let child_errors = minus.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected minus to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // end

        {
            let end = &self.end;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::End)
            {
                let child_errors = end.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected end to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for VersionTerm<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::VersionTerm) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::VersionTerm,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // operator
        if let Some(operator) = &self.operator {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Operator)
            {
                let child_errors = operator.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected operator to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Operator) {
                errors.push(NodeCheckerError::new(format!("Expected operator to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // literal

        {
            let literal = &self.literal;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Literal)
            {
                let child_errors = literal.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected literal to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for WhileStatement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::WhileStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::WhileStatement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // while_keyword

        {
            let while_keyword = &self.while_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::WhileKeyword)
            {
                let child_errors = while_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected while_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // open_paren

        {
            let open_paren = &self.open_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenParen)
            {
                let child_errors = open_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // condition

        {
            let condition = &self.condition;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Condition)
            {
                let child_errors = condition.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected condition to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseParen)
            {
                let child_errors = close_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // body

        {
            let body = &self.body;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Body)
            {
                let child_errors = body.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected body to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

// TODO(v2): YulBlock ignores the statements for now until the Lexer can
// perform a context switch, therefore the checker ignores that edge
impl<'arena> NodeChecker for YulBlock<'arena> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulBlock) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulBlock,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // open_brace

        {
            let open_brace = &self.open_brace;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenBrace)
            {
                let child_errors = open_brace.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    format!("Expected open_brace to be present in the CST, but it was not"),
                    node_range.clone(),
                ));
            }
        }

        // statements

        {
            // Prepare edge label

            if let Some(_) = extract_first_with_label(&mut children, EdgeLabel::Statements) {
                // We don't check statements, since V2 can't parse them yet
            } else {
                errors.push(NodeCheckerError::new(
                    format!("Expected statements to be present in the CST, but it was not"),
                    node_range.clone(),
                ));
            }
        }

        // close_brace

        {
            let close_brace = &self.close_brace;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseBrace)
            {
                let child_errors = close_brace.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    format!("Expected close_brace to be present in the CST, but it was not"),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected 0 children left, but there's some left {:#?}",
                    children
                ),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for YulBreakStatement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulBreakStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulBreakStatement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // break_keyword

        {
            let break_keyword = &self.break_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::BreakKeyword)
            {
                let child_errors = break_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected break_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for YulColonAndEqual<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulColonAndEqual) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulColonAndEqual,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // colon

        {
            let colon = &self.colon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Colon)
            {
                let child_errors = colon.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected colon to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // equal

        {
            let equal = &self.equal;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Equal)
            {
                let child_errors = equal.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected equal to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for YulContinueStatement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulContinueStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulContinueStatement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // continue_keyword

        {
            let continue_keyword = &self.continue_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::ContinueKeyword)
            {
                let child_errors =
                    continue_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected continue_keyword to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for YulDefaultCase<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulDefaultCase) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulDefaultCase,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // default_keyword

        {
            let default_keyword = &self.default_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::DefaultKeyword)
            {
                let child_errors =
                    default_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected default_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // body

        {
            let body = &self.body;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Body)
            {
                let child_errors = body.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected body to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for YulEqualAndColon<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulEqualAndColon) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulEqualAndColon,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // equal

        {
            let equal = &self.equal;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Equal)
            {
                let child_errors = equal.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected equal to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // colon

        {
            let colon = &self.colon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Colon)
            {
                let child_errors = colon.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected colon to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for YulForStatement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulForStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulForStatement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // for_keyword

        {
            let for_keyword = &self.for_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::ForKeyword)
            {
                let child_errors = for_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected for_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // initialization

        {
            let initialization = &self.initialization;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Initialization)
            {
                let child_errors = initialization.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected initialization to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // condition

        {
            let condition = &self.condition;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Condition)
            {
                let child_errors = condition.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected condition to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // iterator

        {
            let iterator = &self.iterator;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Iterator)
            {
                let child_errors = iterator.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected iterator to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // body

        {
            let body = &self.body;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Body)
            {
                let child_errors = body.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected body to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for YulFunctionCallExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulFunctionCallExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulFunctionCallExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // operand

        {
            let operand = &self.operand;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Operand)
            {
                let child_errors = operand.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected operand to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // open_paren

        {
            let open_paren = &self.open_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenParen)
            {
                let child_errors = open_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // arguments

        {
            let arguments = &self.arguments;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Arguments)
            {
                let child_errors = arguments.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected arguments to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseParen)
            {
                let child_errors = close_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for YulFunctionDefinition<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulFunctionDefinition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulFunctionDefinition,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // function_keyword

        {
            let function_keyword = &self.function_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::FunctionKeyword)
            {
                let child_errors =
                    function_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected function_keyword to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        }

        // name

        {
            let name = &self.name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Name)
            {
                let child_errors = name.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected name to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Parameters)
            {
                let child_errors = parameters.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected parameters to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // returns
        if let Some(returns) = &self.returns {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Returns)
            {
                let child_errors = returns.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected returns to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Returns) {
                errors.push(NodeCheckerError::new(format!("Expected returns to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // body

        {
            let body = &self.body;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Body)
            {
                let child_errors = body.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected body to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for YulIfStatement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulIfStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulIfStatement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // if_keyword

        {
            let if_keyword = &self.if_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::IfKeyword)
            {
                let child_errors = if_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected if_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // condition

        {
            let condition = &self.condition;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Condition)
            {
                let child_errors = condition.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected condition to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // body

        {
            let body = &self.body;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Body)
            {
                let child_errors = body.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected body to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for YulLabel<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulLabel) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulLabel,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // label

        {
            let label = &self.label;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Label)
            {
                let child_errors = label.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected label to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // colon

        {
            let colon = &self.colon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Colon)
            {
                let child_errors = colon.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected colon to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for YulLeaveStatement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulLeaveStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulLeaveStatement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // leave_keyword

        {
            let leave_keyword = &self.leave_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::LeaveKeyword)
            {
                let child_errors = leave_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected leave_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for YulParametersDeclaration<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulParametersDeclaration) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulParametersDeclaration,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // open_paren

        {
            let open_paren = &self.open_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::OpenParen)
            {
                let child_errors = open_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected open_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // parameters

        {
            let parameters = &self.parameters;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Parameters)
            {
                let child_errors = parameters.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected parameters to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CloseParen)
            {
                let child_errors = close_paren.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected close_paren to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for YulReturnsDeclaration<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulReturnsDeclaration) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulReturnsDeclaration,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // minus_greater_than

        {
            let minus_greater_than = &self.minus_greater_than;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::MinusGreaterThan)
            {
                let child_errors =
                    minus_greater_than.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected minus_greater_than to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        }

        // variables

        {
            let variables = &self.variables;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Variables)
            {
                let child_errors = variables.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected variables to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for YulStackAssignmentStatement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulStackAssignmentStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulStackAssignmentStatement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // assignment

        {
            let assignment = &self.assignment;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Assignment)
            {
                let child_errors = assignment.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected assignment to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // variable

        {
            let variable = &self.variable;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Variable)
            {
                let child_errors = variable.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected variable to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for YulSwitchStatement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulSwitchStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulSwitchStatement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // switch_keyword

        {
            let switch_keyword = &self.switch_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::SwitchKeyword)
            {
                let child_errors = switch_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected switch_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // expression

        {
            let expression = &self.expression;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Expression)
            {
                let child_errors = expression.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected expression to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // cases

        {
            let cases = &self.cases;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Cases)
            {
                let child_errors = cases.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected cases to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for YulValueCase<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulValueCase) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulValueCase,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // case_keyword

        {
            let case_keyword = &self.case_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::CaseKeyword)
            {
                let child_errors = case_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected case_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // value

        {
            let value = &self.value;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Value)
            {
                let child_errors = value.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected value to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // body

        {
            let body = &self.body;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Body)
            {
                let child_errors = body.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected body to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for YulVariableAssignmentStatement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulVariableAssignmentStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulVariableAssignmentStatement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // variables

        {
            let variables = &self.variables;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Variables)
            {
                let child_errors = variables.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected variables to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // assignment

        {
            let assignment = &self.assignment;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Assignment)
            {
                let child_errors = assignment.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected assignment to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // expression

        {
            let expression = &self.expression;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Expression)
            {
                let child_errors = expression.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected expression to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for YulVariableDeclarationStatement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulVariableDeclarationStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulVariableDeclarationStatement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // let_keyword

        {
            let let_keyword = &self.let_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::LetKeyword)
            {
                let child_errors = let_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected let_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // variables

        {
            let variables = &self.variables;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Variables)
            {
                let child_errors = variables.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected variables to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // value
        if let Some(value) = &self.value {
            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Value)
            {
                let child_errors = value.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected value to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else {
            // If it's not there on the AST, it shouldn't be in the CST
            if let Some((child, _)) = extract_first_with_label(&mut children, EdgeLabel::Value) {
                errors.push(NodeCheckerError::new(
                    format!(
                        "Expected value to not be present in the CST, but it was there: {child:#?}"
                    ),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for YulVariableDeclarationValue<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulVariableDeclarationValue) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulVariableDeclarationValue,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // assignment

        {
            let assignment = &self.assignment;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Assignment)
            {
                let child_errors = assignment.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected assignment to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // expression

        {
            let expression = &self.expression;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_first_with_label(&mut children, EdgeLabel::Expression)
            {
                let child_errors = expression.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected expression to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

//
// Choices:
//

/// Generic `NodeChecker` for choices
impl NodeChecker for AbicoderVersion<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::AbicoderVersion) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::AbicoderVersion,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::AbicoderVersion
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::AbicoderV1Keyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::AbicoderV2Keyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for ArgumentsDeclaration<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ArgumentsDeclaration) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ArgumentsDeclaration,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::ArgumentsDeclaration
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::PositionalArgumentsDeclaration(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::NamedArgumentsDeclaration(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for ConstructorAttribute<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ConstructorAttribute) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ConstructorAttribute,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::ConstructorAttribute
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::ModifierInvocation(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::InternalKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::OverrideKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::PayableKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::PublicKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::VirtualKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for ContractMember<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ContractMember) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ContractMember,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::ContractMember
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::UsingDirective(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::FunctionDefinition(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ConstructorDefinition(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ReceiveFunctionDefinition(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::FallbackFunctionDefinition(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::UnnamedFunctionDefinition(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ModifierDefinition(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::StructDefinition(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::EnumDefinition(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::EventDefinition(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ErrorDefinition(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::UserDefinedValueTypeDefinition(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::StateVariableDefinition(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for ContractSpecifier<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ContractSpecifier) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ContractSpecifier,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::ContractSpecifier
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::InheritanceSpecifier(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::StorageLayoutSpecifier(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for ElementaryType<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ElementaryType) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ElementaryType,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::ElementaryType
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::BoolKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ByteKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::StringKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::AddressType(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::BytesKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::IntKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::UintKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::FixedKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::UfixedKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for ExperimentalFeature<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ExperimentalFeature) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ExperimentalFeature,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::ExperimentalFeature
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::ABIEncoderV2Keyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::SMTCheckerKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::StringLiteral(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for Expression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::Expression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::Expression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::Expression
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::AssignmentExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ConditionalExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::OrExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::AndExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::EqualityExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::InequalityExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::BitwiseOrExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::BitwiseXorExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::BitwiseAndExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ShiftExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::AdditiveExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::MultiplicativeExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ExponentiationExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::PostfixExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::PrefixExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::FunctionCallExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::CallOptionsExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::MemberAccessExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::IndexAccessExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::NewExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::TupleExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::TypeExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ArrayExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::HexNumberExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::DecimalNumberExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::StringExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ElementaryType(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::PayableKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ThisKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::SuperKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::TrueKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::FalseKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::Identifier(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

// Special case for operator choices that are merged together
impl NodeChecker for Expression_AdditiveExpression_Operator<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        match self {
            Self::Minus(element) => element.check_node_with_offset(node, text_offset),

            Self::Plus(element) => element.check_node_with_offset(node, text_offset),
        }
    }
}

// Special case for operator choices that are merged together
impl NodeChecker for Expression_AssignmentExpression_Operator<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        match self {
            Self::AmpersandEqual(element) => element.check_node_with_offset(node, text_offset),

            Self::AsteriskEqual(element) => element.check_node_with_offset(node, text_offset),

            Self::BarEqual(element) => element.check_node_with_offset(node, text_offset),

            Self::CaretEqual(element) => element.check_node_with_offset(node, text_offset),

            Self::Equal(element) => element.check_node_with_offset(node, text_offset),

            Self::GreaterThanGreaterThanEqual(element) => {
                element.check_node_with_offset(node, text_offset)
            }

            Self::GreaterThanGreaterThanGreaterThanEqual(element) => {
                element.check_node_with_offset(node, text_offset)
            }

            Self::LessThanLessThanEqual(element) => {
                element.check_node_with_offset(node, text_offset)
            }

            Self::MinusEqual(element) => element.check_node_with_offset(node, text_offset),

            Self::PercentEqual(element) => element.check_node_with_offset(node, text_offset),

            Self::PlusEqual(element) => element.check_node_with_offset(node, text_offset),

            Self::SlashEqual(element) => element.check_node_with_offset(node, text_offset),
        }
    }
}

// Special case for operator choices that are merged together
impl NodeChecker for Expression_EqualityExpression_Operator<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        match self {
            Self::BangEqual(element) => element.check_node_with_offset(node, text_offset),

            Self::EqualEqual(element) => element.check_node_with_offset(node, text_offset),
        }
    }
}

// Special case for operator choices that are merged together
impl NodeChecker for Expression_ExponentiationExpression_Operator<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        match self {
            Self::AsteriskAsterisk(element) => element.check_node_with_offset(node, text_offset),
        }
    }
}

// Special case for operator choices that are merged together
impl NodeChecker for Expression_InequalityExpression_Operator<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        match self {
            Self::GreaterThan(element) => element.check_node_with_offset(node, text_offset),

            Self::GreaterThanEqual(element) => element.check_node_with_offset(node, text_offset),

            Self::LessThan(element) => element.check_node_with_offset(node, text_offset),

            Self::LessThanEqual(element) => element.check_node_with_offset(node, text_offset),
        }
    }
}

// Special case for operator choices that are merged together
impl NodeChecker for Expression_MultiplicativeExpression_Operator<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        match self {
            Self::Asterisk(element) => element.check_node_with_offset(node, text_offset),

            Self::Percent(element) => element.check_node_with_offset(node, text_offset),

            Self::Slash(element) => element.check_node_with_offset(node, text_offset),
        }
    }
}

// Special case for operator choices that are merged together
impl NodeChecker for Expression_PostfixExpression_Operator<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        match self {
            Self::MinusMinus(element) => element.check_node_with_offset(node, text_offset),

            Self::PlusPlus(element) => element.check_node_with_offset(node, text_offset),
        }
    }
}

// Special case for operator choices that are merged together
impl NodeChecker for Expression_PrefixExpression_Operator<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        match self {
            Self::Bang(element) => element.check_node_with_offset(node, text_offset),

            Self::DeleteKeyword(element) => element.check_node_with_offset(node, text_offset),

            Self::Minus(element) => element.check_node_with_offset(node, text_offset),

            Self::MinusMinus(element) => element.check_node_with_offset(node, text_offset),

            Self::Plus(element) => element.check_node_with_offset(node, text_offset),

            Self::PlusPlus(element) => element.check_node_with_offset(node, text_offset),

            Self::Tilde(element) => element.check_node_with_offset(node, text_offset),
        }
    }
}

// Special case for operator choices that are merged together
impl NodeChecker for Expression_ShiftExpression_Operator<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        match self {
            Self::GreaterThanGreaterThan(element) => {
                element.check_node_with_offset(node, text_offset)
            }

            Self::GreaterThanGreaterThanGreaterThan(element) => {
                element.check_node_with_offset(node, text_offset)
            }

            Self::LessThanLessThan(element) => element.check_node_with_offset(node, text_offset),
        }
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for FallbackFunctionAttribute<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FallbackFunctionAttribute) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::FallbackFunctionAttribute,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::FallbackFunctionAttribute
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::ModifierInvocation(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::OverrideSpecifier(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ExternalKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::PayableKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::PureKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ViewKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::VirtualKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for ForStatementCondition<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ForStatementCondition) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ForStatementCondition,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::ForStatementCondition
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::ExpressionStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::Semicolon(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for ForStatementInitialization<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ForStatementInitialization) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ForStatementInitialization,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::ForStatementInitialization
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::TupleDeconstructionStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::VariableDeclarationStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ExpressionStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::Semicolon(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for FunctionAttribute<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FunctionAttribute) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::FunctionAttribute,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::FunctionAttribute
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::ModifierInvocation(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::OverrideSpecifier(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ConstantKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ExternalKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::InternalKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::PayableKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::PrivateKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::PublicKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::PureKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ViewKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::VirtualKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for FunctionBody<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FunctionBody) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::FunctionBody,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::FunctionBody
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::Block(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::Semicolon(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for FunctionName<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FunctionName) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::FunctionName,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::FunctionName
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::Identifier(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::FallbackKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ReceiveKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for FunctionTypeAttribute<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FunctionTypeAttribute) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::FunctionTypeAttribute,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::FunctionTypeAttribute
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::InternalKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ExternalKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::PrivateKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::PublicKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ConstantKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::PureKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ViewKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::PayableKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for HexStringLiteral<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::HexStringLiteral) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::HexStringLiteral,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::HexStringLiteral
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::SingleQuotedHexStringLiteral(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::DoubleQuotedHexStringLiteral(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for ImportClause<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ImportClause) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ImportClause,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::ImportClause
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::PathImport(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::NamedImport(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ImportDeconstruction(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for MappingKeyType<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::MappingKeyType) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::MappingKeyType,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::MappingKeyType
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::ElementaryType(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::IdentifierPath(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// `NodeChecker` for MemberAccessIdentifier is done by hand since it's not present in V1
impl NodeChecker for MemberAccessIdentifier<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        match self {
            Self::Identifier(element) => element.check_node_with_offset(node, text_offset),
            Self::AddressKeyword(element) => {
                let ident = Identifier {
                    l: element.l,
                    r: element.r,
                    phantom: PhantomData,
                };
                ident.check_node_with_offset(node, text_offset)
            }
        }
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for ModifierAttribute<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ModifierAttribute) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ModifierAttribute,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::ModifierAttribute
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::OverrideSpecifier(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::VirtualKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for NumberUnit<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::NumberUnit) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::NumberUnit,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::NumberUnit
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::WeiKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::GweiKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::SzaboKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::FinneyKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::EtherKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::SecondsKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::MinutesKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::HoursKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::DaysKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::WeeksKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::YearsKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for Pragma<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::Pragma) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::Pragma,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::Pragma
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::VersionPragma(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::AbicoderPragma(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ExperimentalPragma(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for ReceiveFunctionAttribute<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ReceiveFunctionAttribute) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ReceiveFunctionAttribute,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::ReceiveFunctionAttribute
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::ModifierInvocation(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::OverrideSpecifier(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ExternalKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::PayableKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::VirtualKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for SourceUnitMember<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::SourceUnitMember) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::SourceUnitMember,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::SourceUnitMember
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::PragmaDirective(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ImportDirective(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ContractDefinition(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::InterfaceDefinition(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::LibraryDefinition(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::StructDefinition(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::EnumDefinition(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::FunctionDefinition(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ErrorDefinition(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::UserDefinedValueTypeDefinition(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::UsingDirective(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::EventDefinition(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ConstantDefinition(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for StateVariableAttribute<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::StateVariableAttribute) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::StateVariableAttribute,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::StateVariableAttribute
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::OverrideSpecifier(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ConstantKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::InternalKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::PrivateKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::PublicKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ImmutableKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::TransientKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for Statement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::Statement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::Statement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::Statement
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::IfStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ForStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::WhileStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::DoWhileStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ContinueStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::BreakStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ReturnStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ThrowStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::EmitStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::TryStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::RevertStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::AssemblyStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::Block(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::UncheckedBlock(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::TupleDeconstructionStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::VariableDeclarationStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ExpressionStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for StorageLocation<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::StorageLocation) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::StorageLocation,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::StorageLocation
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::MemoryKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::StorageKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::CallDataKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for StringExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::StringExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::StringExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::StringExpression
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::StringLiteral(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::StringLiterals(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::HexStringLiteral(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::HexStringLiterals(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::UnicodeStringLiterals(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for StringLiteral<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::StringLiteral) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::StringLiteral,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::StringLiteral
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::SingleQuotedStringLiteral(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::DoubleQuotedStringLiteral(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for TupleMember<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TupleMember) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::TupleMember,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::TupleMember
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::TypedTupleMember(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::UntypedTupleMember(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for TypeName<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TypeName) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::TypeName,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::TypeName
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::ArrayTypeName(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::FunctionType(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::MappingType(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ElementaryType(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::IdentifierPath(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for UnicodeStringLiteral<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UnicodeStringLiteral) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::UnicodeStringLiteral,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::UnicodeStringLiteral
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::SingleQuotedUnicodeStringLiteral(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::DoubleQuotedUnicodeStringLiteral(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for UnnamedFunctionAttribute<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UnnamedFunctionAttribute) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::UnnamedFunctionAttribute,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::UnnamedFunctionAttribute
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::ModifierInvocation(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ConstantKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ExternalKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::InternalKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::PayableKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::PrivateKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::PublicKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::PureKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::ViewKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for UsingClause<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UsingClause) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::UsingClause,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::UsingClause
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::IdentifierPath(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::UsingDeconstruction(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for UsingOperator<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UsingOperator) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::UsingOperator,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::UsingOperator
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::Ampersand(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::Asterisk(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::BangEqual(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::Bar(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::Caret(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::EqualEqual(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::GreaterThan(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::GreaterThanEqual(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::LessThan(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::LessThanEqual(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::Minus(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::Percent(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::Plus(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::Slash(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::Tilde(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for UsingTarget<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UsingTarget) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::UsingTarget,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::UsingTarget
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::TypeName(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::Asterisk(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for VariableDeclarationType<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::VariableDeclarationType) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::VariableDeclarationType,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::VariableDeclarationType
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::TypeName(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::VarKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for VersionExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::VersionExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::VersionExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::VersionExpression
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::VersionRange(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::VersionTerm(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for VersionLiteral<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::VersionLiteral) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::VersionLiteral,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::VersionLiteral
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::SimpleVersionLiteral(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::SingleQuotedVersionLiteral(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::DoubleQuotedVersionLiteral(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for VersionOperator<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::VersionOperator) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::VersionOperator,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::VersionOperator
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::Caret(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::Tilde(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::Equal(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::LessThan(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::GreaterThan(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::LessThanEqual(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::GreaterThanEqual(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for YulAssignmentOperator<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulAssignmentOperator) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulAssignmentOperator,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::YulAssignmentOperator
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::ColonEqual(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::YulColonAndEqual(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for YulExpression<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::YulExpression
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::YulFunctionCallExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::YulLiteral(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::YulPath(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for YulLiteral<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulLiteral) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulLiteral,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::YulLiteral
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::YulTrueKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::YulFalseKeyword(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::YulDecimalLiteral(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::YulHexLiteral(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::HexStringLiteral(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::StringLiteral(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for YulStackAssignmentOperator<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulStackAssignmentOperator) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulStackAssignmentOperator,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::YulStackAssignmentOperator
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::EqualColon(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::YulEqualAndColon(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for YulStatement<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulStatement) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulStatement,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::YulStatement
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::YulBlock(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::YulFunctionDefinition(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::YulStackAssignmentStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::YulIfStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::YulForStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::YulSwitchStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::YulLeaveStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::YulBreakStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::YulContinueStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::YulVariableAssignmentStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::YulLabel(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::YulVariableDeclarationStatement(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::YulExpression(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for YulSwitchCase<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulSwitchCase) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulSwitchCase,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != 1 {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected exactly one child for {}, but got: {children:#?}",
                    NonterminalKind::YulSwitchCase
                ),
                node_range,
            )];
        }

        let (child, child_offset) = &children[0];

        if child.label != EdgeLabel::Variant {
            let child_range = *child_offset..(*child_offset + child.node.text_len());
            return vec![NodeCheckerError::new(
                format!(
                    "Expected child to be of variant type, but it was {}",
                    child.label
                ),
                child_range,
            )];
        }

        let mut errors = vec![];

        match self {
            Self::YulDefaultCase(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }

            Self::YulValueCase(element) => {
                errors.extend(element.check_node_with_offset(&child.node, *child_offset));
            }
        }

        errors
    }
}

//
// Repeated & Separated
//

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for ArrayValues<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ArrayValues) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ArrayValues,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for AssemblyFlags<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::AssemblyFlags) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::AssemblyFlags,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for CallOptions<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::CallOptions) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::CallOptions,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for CatchClauses<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::CatchClauses) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::CatchClauses,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for ConstructorAttributes<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ConstructorAttributes) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ConstructorAttributes,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for ContractMembers<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ContractMembers) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ContractMembers,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for ContractSpecifiers<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ContractSpecifiers) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ContractSpecifiers,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for EnumMembers<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::EnumMembers) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::EnumMembers,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for ErrorParameters<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ErrorParameters) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ErrorParameters,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for EventParameters<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::EventParameters) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::EventParameters,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for FallbackFunctionAttributes<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FallbackFunctionAttributes) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::FallbackFunctionAttributes,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for FunctionAttributes<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FunctionAttributes) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::FunctionAttributes,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for FunctionTypeAttributes<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::FunctionTypeAttributes) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::FunctionTypeAttributes,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for HexStringLiterals<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::HexStringLiterals) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::HexStringLiterals,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

// Skip NodeChecker for IdentifierPathTailElements

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for ImportDeconstructionSymbols<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ImportDeconstructionSymbols) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ImportDeconstructionSymbols,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for InheritanceTypes<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::InheritanceTypes) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::InheritanceTypes,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for InterfaceMembers<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::InterfaceMembers) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::InterfaceMembers,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for LibraryMembers<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::LibraryMembers) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::LibraryMembers,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for ModifierAttributes<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ModifierAttributes) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ModifierAttributes,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

// We skip NodeChecker for MultipleCallOptionsNew, it's handled by NewExpression

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for NamedArguments<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::NamedArguments) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::NamedArguments,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for OverridePaths<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::OverridePaths) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::OverridePaths,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for Parameters<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::Parameters) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::Parameters,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for PositionalArguments<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::PositionalArguments) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::PositionalArguments,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for ReceiveFunctionAttributes<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::ReceiveFunctionAttributes) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::ReceiveFunctionAttributes,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for SimpleVersionLiteral<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::SimpleVersionLiteral) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::SimpleVersionLiteral,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for SourceUnitMembers<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::SourceUnitMembers) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::SourceUnitMembers,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for StateVariableAttributes<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::StateVariableAttributes) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::StateVariableAttributes,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for Statements<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::Statements) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::Statements,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for StringLiterals<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::StringLiterals) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::StringLiterals,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for StructMembers<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::StructMembers) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::StructMembers,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for TupleDeconstructionElements<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TupleDeconstructionElements) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::TupleDeconstructionElements,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for TupleValues<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TupleValues) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::TupleValues,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for UnicodeStringLiterals<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UnicodeStringLiterals) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::UnicodeStringLiterals,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for UnnamedFunctionAttributes<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UnnamedFunctionAttributes) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::UnnamedFunctionAttributes,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for UsingDeconstructionSymbols<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::UsingDeconstructionSymbols) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::UsingDeconstructionSymbols,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for VersionExpressionSet<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::VersionExpressionSet) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::VersionExpressionSet,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for VersionExpressionSets<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::VersionExpressionSets) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::VersionExpressionSets,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for YulArguments<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulArguments) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulArguments,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for YulParameters<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulParameters) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulParameters,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for YulPath<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulPath) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulPath,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for YulPaths<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulPaths) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulPaths,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for YulStatements<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulStatements) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulStatements,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for YulSwitchCases<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulSwitchCases) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulSwitchCases,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for YulVariableNames<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::YulVariableNames) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::YulVariableNames,
                    node.kind()
                ),
                node_range,
            )];
        }

        let children = children_with_offsets(node, text_offset);

        if children.len() != self.elements.len() {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected {} elements, but got: {:#?}",
                    self.elements.len(),
                    children
                ),
                node_range,
            )];
        }

        let mut errors = vec![];

        for (i, (child, child_offset)) in children.iter().enumerate() {
            let element = &self.elements[i];
            errors.extend(element.check_node_with_offset(&child.node, *child_offset));
        }
        errors
    }
}

// Terminals

/// Generic `NodeChecker` for terminals
impl NodeChecker for ABIEncoderV2Keyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "ABIEncoderV2Keyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for AbicoderKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "AbicoderKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for AbicoderV1Keyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "AbicoderV1Keyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for AbicoderV2Keyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "AbicoderV2Keyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for AbstractKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "AbstractKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for AddressKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "AddressKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for AfterKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "AfterKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for AliasKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "AliasKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for Ampersand<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "Ampersand";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for AmpersandAmpersand<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "AmpersandAmpersand";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for AmpersandEqual<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "AmpersandEqual";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for AnonymousKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "AnonymousKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for ApplyKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "ApplyKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for AsKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "AsKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for AssemblyKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "AssemblyKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for Asterisk<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "Asterisk";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for AsteriskAsterisk<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "AsteriskAsterisk";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for AsteriskEqual<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "AsteriskEqual";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for AtKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "AtKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for AutoKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "AutoKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for Bang<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "Bang";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for BangEqual<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "BangEqual";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for Bar<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "Bar";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for BarBar<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "BarBar";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for BarEqual<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "BarEqual";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for BoolKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "BoolKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for BreakKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "BreakKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for ByteKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "ByteKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for BytesKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "BytesKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for CallDataKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "CallDataKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for Caret<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "Caret";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for CaretEqual<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "CaretEqual";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for CaseKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "CaseKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for CatchKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "CatchKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for CloseBrace<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "CloseBrace";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for CloseBracket<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "CloseBracket";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for CloseParen<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "CloseParen";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for Colon<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "Colon";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for ColonEqual<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "ColonEqual";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for Comma<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "Comma";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for ConstantKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "ConstantKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for ConstructorKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "ConstructorKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for ContinueKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "ContinueKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for ContractKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "ContractKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for CopyOfKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "CopyOfKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for DaysKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "DaysKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for DecimalLiteral<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "DecimalLiteral";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for DefaultKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "DefaultKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for DefineKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "DefineKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for DeleteKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "DeleteKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for DoKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "DoKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for DoubleQuotedHexStringLiteral<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "DoubleQuotedHexStringLiteral";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for DoubleQuotedStringLiteral<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "DoubleQuotedStringLiteral";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for DoubleQuotedUnicodeStringLiteral<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "DoubleQuotedUnicodeStringLiteral";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for DoubleQuotedVersionLiteral<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "DoubleQuotedVersionLiteral";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for ElseKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "ElseKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for EmitKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "EmitKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for EndOfLine<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "EndOfLine";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for EnumKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "EnumKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for Equal<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "Equal";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for EqualColon<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "EqualColon";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for EqualEqual<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "EqualEqual";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for EqualGreaterThan<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "EqualGreaterThan";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for ErrorKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "ErrorKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for EtherKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "EtherKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for EventKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "EventKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for ExperimentalKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "ExperimentalKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for ExternalKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "ExternalKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for FallbackKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "FallbackKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for FalseKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "FalseKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for FinalKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "FinalKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for FinneyKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "FinneyKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for FixedKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "FixedKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for ForKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "ForKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for FromKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "FromKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for FunctionKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "FunctionKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for GlobalKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "GlobalKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for GreaterThan<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "GreaterThan";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for GreaterThanEqual<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "GreaterThanEqual";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for GreaterThanGreaterThan<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "GreaterThanGreaterThan";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for GreaterThanGreaterThanEqual<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "GreaterThanGreaterThanEqual";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for GreaterThanGreaterThanGreaterThan<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "GreaterThanGreaterThanGreaterThan";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for GreaterThanGreaterThanGreaterThanEqual<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "GreaterThanGreaterThanGreaterThanEqual";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for GweiKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "GweiKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for HexKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "HexKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for HexLiteral<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "HexLiteral";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for HoursKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "HoursKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for Identifier<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "Identifier";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for IfKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "IfKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for ImmutableKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "ImmutableKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for ImplementsKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "ImplementsKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for ImportKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "ImportKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for InKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "InKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for IndexedKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "IndexedKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for InlineKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "InlineKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for IntKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "IntKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for InterfaceKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "InterfaceKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for InternalKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "InternalKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for IsKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "IsKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for LayoutKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "LayoutKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for LessThan<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "LessThan";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for LessThanEqual<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "LessThanEqual";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for LessThanLessThan<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "LessThanLessThan";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for LessThanLessThanEqual<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "LessThanLessThanEqual";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for LetKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "LetKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for LibraryKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "LibraryKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for MacroKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "MacroKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for MappingKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "MappingKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for MatchKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "MatchKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for MemoryKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "MemoryKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for Minus<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "Minus";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for MinusEqual<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "MinusEqual";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for MinusGreaterThan<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "MinusGreaterThan";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for MinusMinus<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "MinusMinus";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for MinutesKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "MinutesKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for ModifierKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "ModifierKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for MultiLineComment<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "MultiLineComment";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for MultiLineNatSpecComment<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "MultiLineNatSpecComment";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for MutableKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "MutableKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for NewKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "NewKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for NullKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "NullKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for OfKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "OfKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for OpenBrace<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "OpenBrace";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for OpenBracket<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "OpenBracket";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for OpenParen<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "OpenParen";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for OverrideKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "OverrideKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for PartialKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "PartialKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for PayableKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "PayableKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for Percent<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "Percent";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for PercentEqual<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "PercentEqual";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for Period<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "Period";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for Plus<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "Plus";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for PlusEqual<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "PlusEqual";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for PlusPlus<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "PlusPlus";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for PragmaKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "PragmaKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for PrivateKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "PrivateKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for PromiseKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "PromiseKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for PublicKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "PublicKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for PureKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "PureKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for QuestionMark<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "QuestionMark";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for ReceiveKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "ReceiveKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for ReferenceKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "ReferenceKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for RelocatableKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "RelocatableKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for ReturnKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "ReturnKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for ReturnsKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "ReturnsKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for RevertKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "RevertKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for SMTCheckerKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "SMTCheckerKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for SealedKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "SealedKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for SecondsKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "SecondsKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for Semicolon<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "Semicolon";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for SingleLineComment<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "SingleLineComment";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for SingleLineNatSpecComment<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "SingleLineNatSpecComment";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for SingleQuotedHexStringLiteral<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "SingleQuotedHexStringLiteral";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for SingleQuotedStringLiteral<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "SingleQuotedStringLiteral";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for SingleQuotedUnicodeStringLiteral<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "SingleQuotedUnicodeStringLiteral";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for SingleQuotedVersionLiteral<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "SingleQuotedVersionLiteral";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for SizeOfKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "SizeOfKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for Slash<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "Slash";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for SlashEqual<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "SlashEqual";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for SolidityKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "SolidityKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for StaticKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "StaticKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for StorageKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "StorageKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for StringKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "StringKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for StructKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "StructKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for SuperKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "SuperKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for SupportsKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "SupportsKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for SwitchKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "SwitchKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for SzaboKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "SzaboKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for ThisKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "ThisKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for ThrowKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "ThrowKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for Tilde<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "Tilde";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for TransientKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "TransientKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for TrueKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "TrueKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for TryKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "TryKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for TypeDefKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "TypeDefKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for TypeKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "TypeKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for TypeOfKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "TypeOfKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for UfixedKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "UfixedKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for UintKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "UintKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for UncheckedKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "UncheckedKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for UsingKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "UsingKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for VarKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "VarKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for VersionSpecifier<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "VersionSpecifier";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for ViewKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "ViewKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for VirtualKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "VirtualKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for WeeksKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "WeeksKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for WeiKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "WeiKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for WhileKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "WhileKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for Whitespace<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "Whitespace";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YearsKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YearsKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulAbstractKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulAbstractKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulAfterKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulAfterKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulAliasKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulAliasKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulAnonymousKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulAnonymousKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulApplyKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulApplyKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulAsKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulAsKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulAssemblyKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulAssemblyKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulAutoKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulAutoKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulBoolKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulBoolKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulBreakKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulBreakKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulBytesKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulBytesKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulCallDataKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulCallDataKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulCaseKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulCaseKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulCatchKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulCatchKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulConstantKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulConstantKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulConstructorKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulConstructorKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulContinueKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulContinueKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulContractKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulContractKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulCopyOfKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulCopyOfKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulDaysKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulDaysKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulDecimalLiteral<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulDecimalLiteral";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulDefaultKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulDefaultKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulDefineKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulDefineKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulDeleteKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulDeleteKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulDoKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulDoKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulElseKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulElseKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulEmitKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulEmitKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulEnumKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulEnumKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulEtherKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulEtherKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulEventKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulEventKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulExternalKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulExternalKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulFallbackKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulFallbackKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulFalseKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulFalseKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulFinalKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulFinalKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulFinneyKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulFinneyKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulFixedKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulFixedKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulForKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulForKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulFunctionKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulFunctionKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulGweiKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulGweiKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulHexKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulHexKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulHexLiteral<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulHexLiteral";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulHoursKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulHoursKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulIdentifier<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulIdentifier";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulIfKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulIfKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulImmutableKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulImmutableKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulImplementsKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulImplementsKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulImportKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulImportKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulInKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulInKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulIndexedKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulIndexedKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulInlineKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulInlineKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulIntKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulIntKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulInterfaceKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulInterfaceKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulInternalKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulInternalKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulIsKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulIsKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulLeaveKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulLeaveKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulLetKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulLetKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulLibraryKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulLibraryKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulMacroKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulMacroKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulMappingKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulMappingKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulMatchKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulMatchKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulMemoryKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulMemoryKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulMinutesKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulMinutesKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulModifierKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulModifierKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulMutableKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulMutableKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulNewKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulNewKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulNullKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulNullKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulOfKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulOfKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulOverrideKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulOverrideKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulPartialKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulPartialKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulPayableKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulPayableKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulPragmaKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulPragmaKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulPrivateKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulPrivateKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulPromiseKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulPromiseKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulPublicKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulPublicKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulPureKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulPureKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulReceiveKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulReceiveKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulReferenceKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulReferenceKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulRelocatableKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulRelocatableKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulReturnsKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulReturnsKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulSealedKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulSealedKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulSecondsKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulSecondsKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulSizeOfKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulSizeOfKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulStaticKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulStaticKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulStorageKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulStorageKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulStringKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulStringKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulStructKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulStructKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulSuperKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulSuperKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulSupportsKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulSupportsKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulSwitchKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulSwitchKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulSzaboKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulSzaboKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulThisKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulThisKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulThrowKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulThrowKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulTrueKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulTrueKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulTryKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulTryKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulTypeDefKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulTypeDefKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulTypeKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulTypeKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulTypeOfKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulTypeOfKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulUfixedKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulUfixedKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulUintKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulUintKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulUncheckedKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulUncheckedKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulUsingKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulUsingKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulVarKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulVarKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulViewKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulViewKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulVirtualKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulVirtualKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulWeeksKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulWeeksKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulWeiKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulWeiKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulWhileKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulWhileKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}

/// Generic `NodeChecker` for terminals
impl NodeChecker for YulYearsKeyword<'_> {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut errors = vec![];
        if let NodeKind::Terminal(terminal_kind) = node.kind() {
            let v1_kind = terminal_kind.as_ref();
            let v2_kind = "YulYearsKeyword";

            if v1_kind != v2_kind {
                errors.push(NodeCheckerError::new(
                    format!("Expected node kind to be {v2_kind}, but it was {v1_kind}"),
                    node_range,
                ));
            }
        } else {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected node kind to be a terminal, but it was {}",
                    node.kind()
                ),
                node_range,
            ));
        }
        errors
    }
}
