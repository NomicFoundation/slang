// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_lines)]

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

// TODO(v2): Consider using a struct like `Helper` in
// crates/solidity/outputs/cargo/wasm/src/wrappers/ast/selectors.generated.rs

/// Extract the first edge with the given label, and remove it from the vector.
fn extract_with_label(
    v: &mut Vec<(Edge, TextIndex)>,
    label: EdgeLabel,
) -> Option<(Edge, TextIndex)> {
    match v.first() {
        Some((edge, _)) if edge.label == label => Some(v.remove(0)),
        _ => None,
    }
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
impl NodeChecker for AbicoderPragma {
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
                extract_with_label(&mut children, EdgeLabel::AbicoderKeyword)
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
                extract_with_label(&mut children, EdgeLabel::Version)
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
impl NodeChecker for AdditiveExpression {
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
                extract_with_label(&mut children, EdgeLabel::LeftOperand)
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
                extract_with_label(&mut children, EdgeLabel::Operator)
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
                extract_with_label(&mut children, EdgeLabel::RightOperand)
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
impl NodeChecker for AddressType {
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
                extract_with_label(&mut children, EdgeLabel::AddressKeyword)
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
                extract_with_label(&mut children, EdgeLabel::PayableKeyword)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::PayableKeyword) {
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
impl NodeChecker for AndExpression {
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
                extract_with_label(&mut children, EdgeLabel::LeftOperand)
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
                extract_with_label(&mut children, EdgeLabel::Operator)
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
                extract_with_label(&mut children, EdgeLabel::RightOperand)
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
impl NodeChecker for ArrayExpression {
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
                extract_with_label(&mut children, EdgeLabel::OpenBracket)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Items)
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
                extract_with_label(&mut children, EdgeLabel::CloseBracket)
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
impl NodeChecker for ArrayTypeName {
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
                extract_with_label(&mut children, EdgeLabel::Operand)
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
                extract_with_label(&mut children, EdgeLabel::OpenBracket)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Index)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Index) {
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
                extract_with_label(&mut children, EdgeLabel::CloseBracket)
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
impl NodeChecker for AssemblyFlagsDeclaration {
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
                extract_with_label(&mut children, EdgeLabel::OpenParen)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Flags)
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
                extract_with_label(&mut children, EdgeLabel::CloseParen)
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
impl NodeChecker for AssemblyStatement {
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
                extract_with_label(&mut children, EdgeLabel::AssemblyKeyword)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Label)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Label) {
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Flags)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Flags) {
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Body)
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
impl NodeChecker for AssignmentExpression {
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
                extract_with_label(&mut children, EdgeLabel::LeftOperand)
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
                extract_with_label(&mut children, EdgeLabel::Operator)
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
                extract_with_label(&mut children, EdgeLabel::RightOperand)
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
impl NodeChecker for BitwiseAndExpression {
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
                extract_with_label(&mut children, EdgeLabel::LeftOperand)
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
                extract_with_label(&mut children, EdgeLabel::Operator)
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
                extract_with_label(&mut children, EdgeLabel::RightOperand)
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
impl NodeChecker for BitwiseOrExpression {
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
                extract_with_label(&mut children, EdgeLabel::LeftOperand)
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
                extract_with_label(&mut children, EdgeLabel::Operator)
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
                extract_with_label(&mut children, EdgeLabel::RightOperand)
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
impl NodeChecker for BitwiseXorExpression {
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
                extract_with_label(&mut children, EdgeLabel::LeftOperand)
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
                extract_with_label(&mut children, EdgeLabel::Operator)
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
                extract_with_label(&mut children, EdgeLabel::RightOperand)
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
impl NodeChecker for Block {
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
                extract_with_label(&mut children, EdgeLabel::OpenBrace)
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
                extract_with_label(&mut children, EdgeLabel::Statements)
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
                extract_with_label(&mut children, EdgeLabel::CloseBrace)
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
impl NodeChecker for BreakStatement {
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
                extract_with_label(&mut children, EdgeLabel::BreakKeyword)
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
                extract_with_label(&mut children, EdgeLabel::Semicolon)
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
impl NodeChecker for CallOptionsExpression {
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
                extract_with_label(&mut children, EdgeLabel::Operand)
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
                extract_with_label(&mut children, EdgeLabel::OpenBrace)
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
                extract_with_label(&mut children, EdgeLabel::Options)
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
                extract_with_label(&mut children, EdgeLabel::CloseBrace)
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
impl NodeChecker for CatchClause {
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
                extract_with_label(&mut children, EdgeLabel::CatchKeyword)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Error)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Error) {
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Body)
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
impl NodeChecker for CatchClauseError {
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Name) {
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
                extract_with_label(&mut children, EdgeLabel::Parameters)
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
impl NodeChecker for ConditionalExpression {
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
                extract_with_label(&mut children, EdgeLabel::Operand)
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
                extract_with_label(&mut children, EdgeLabel::QuestionMark)
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
                extract_with_label(&mut children, EdgeLabel::TrueExpression)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Colon)
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
                extract_with_label(&mut children, EdgeLabel::FalseExpression)
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
impl NodeChecker for ConstantDefinition {
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
                extract_with_label(&mut children, EdgeLabel::TypeName)
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
                extract_with_label(&mut children, EdgeLabel::ConstantKeyword)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Equal)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Value)
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
                extract_with_label(&mut children, EdgeLabel::Semicolon)
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
impl NodeChecker for ConstructorDefinition {
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
                extract_with_label(&mut children, EdgeLabel::ConstructorKeyword)
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
                extract_with_label(&mut children, EdgeLabel::Parameters)
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
                extract_with_label(&mut children, EdgeLabel::Attributes)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Body)
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
impl NodeChecker for ContinueStatement {
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
                extract_with_label(&mut children, EdgeLabel::ContinueKeyword)
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
                extract_with_label(&mut children, EdgeLabel::Semicolon)
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
impl NodeChecker for ContractDefinition {
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
                extract_with_label(&mut children, EdgeLabel::AbstractKeyword)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::AbstractKeyword)
            {
                errors.push(NodeCheckerError::new(format!("Expected abstract_keyword to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // contract_keyword

        {
            let contract_keyword = &self.contract_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_with_label(&mut children, EdgeLabel::ContractKeyword)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name)
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
                extract_with_label(&mut children, EdgeLabel::Specifiers)
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
                extract_with_label(&mut children, EdgeLabel::OpenBrace)
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
                extract_with_label(&mut children, EdgeLabel::Members)
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
                extract_with_label(&mut children, EdgeLabel::CloseBrace)
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
impl NodeChecker for DecimalNumberExpression {
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
                extract_with_label(&mut children, EdgeLabel::Literal)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Unit)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Unit) {
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
impl NodeChecker for DoWhileStatement {
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
                extract_with_label(&mut children, EdgeLabel::DoKeyword)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Body)
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
                extract_with_label(&mut children, EdgeLabel::WhileKeyword)
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
                extract_with_label(&mut children, EdgeLabel::OpenParen)
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
                extract_with_label(&mut children, EdgeLabel::Condition)
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
                extract_with_label(&mut children, EdgeLabel::CloseParen)
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
                extract_with_label(&mut children, EdgeLabel::Semicolon)
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
impl NodeChecker for ElseBranch {
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
                extract_with_label(&mut children, EdgeLabel::ElseKeyword)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Body)
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
impl NodeChecker for EmitStatement {
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
                extract_with_label(&mut children, EdgeLabel::EmitKeyword)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Event)
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
                extract_with_label(&mut children, EdgeLabel::Arguments)
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
                extract_with_label(&mut children, EdgeLabel::Semicolon)
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
impl NodeChecker for EnumDefinition {
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
                extract_with_label(&mut children, EdgeLabel::EnumKeyword)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name)
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
                extract_with_label(&mut children, EdgeLabel::OpenBrace)
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
                extract_with_label(&mut children, EdgeLabel::Members)
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
                extract_with_label(&mut children, EdgeLabel::CloseBrace)
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
impl NodeChecker for EqualityExpression {
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
                extract_with_label(&mut children, EdgeLabel::LeftOperand)
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
                extract_with_label(&mut children, EdgeLabel::Operator)
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
                extract_with_label(&mut children, EdgeLabel::RightOperand)
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
impl NodeChecker for ErrorDefinition {
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
                extract_with_label(&mut children, EdgeLabel::ErrorKeyword)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name)
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
                extract_with_label(&mut children, EdgeLabel::Members)
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
                extract_with_label(&mut children, EdgeLabel::Semicolon)
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
impl NodeChecker for ErrorParameter {
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
                extract_with_label(&mut children, EdgeLabel::TypeName)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Name) {
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
impl NodeChecker for ErrorParametersDeclaration {
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
                extract_with_label(&mut children, EdgeLabel::OpenParen)
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
                extract_with_label(&mut children, EdgeLabel::Parameters)
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
                extract_with_label(&mut children, EdgeLabel::CloseParen)
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
impl NodeChecker for EventDefinition {
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
                extract_with_label(&mut children, EdgeLabel::EventKeyword)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name)
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
                extract_with_label(&mut children, EdgeLabel::Parameters)
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
                extract_with_label(&mut children, EdgeLabel::AnonymousKeyword)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::AnonymousKeyword)
            {
                errors.push(NodeCheckerError::new(format!("Expected anonymous_keyword to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_with_label(&mut children, EdgeLabel::Semicolon)
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
impl NodeChecker for EventParameter {
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
                extract_with_label(&mut children, EdgeLabel::TypeName)
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
                extract_with_label(&mut children, EdgeLabel::IndexedKeyword)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::IndexedKeyword) {
                errors.push(NodeCheckerError::new(format!("Expected indexed_keyword to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // name
        if let Some(name) = &self.name {
            // Prepare edge label

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Name) {
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
impl NodeChecker for EventParametersDeclaration {
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
                extract_with_label(&mut children, EdgeLabel::OpenParen)
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
                extract_with_label(&mut children, EdgeLabel::Parameters)
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
                extract_with_label(&mut children, EdgeLabel::CloseParen)
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
impl NodeChecker for ExperimentalPragma {
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
                extract_with_label(&mut children, EdgeLabel::ExperimentalKeyword)
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
                extract_with_label(&mut children, EdgeLabel::Feature)
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
impl NodeChecker for ExponentiationExpression {
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
                extract_with_label(&mut children, EdgeLabel::LeftOperand)
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
                extract_with_label(&mut children, EdgeLabel::Operator)
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
                extract_with_label(&mut children, EdgeLabel::RightOperand)
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
impl NodeChecker for ExpressionStatement {
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
                extract_with_label(&mut children, EdgeLabel::Expression)
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
                extract_with_label(&mut children, EdgeLabel::Semicolon)
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
impl NodeChecker for FallbackFunctionDefinition {
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
                extract_with_label(&mut children, EdgeLabel::FallbackKeyword)
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
                extract_with_label(&mut children, EdgeLabel::Parameters)
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
                extract_with_label(&mut children, EdgeLabel::Attributes)
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
                extract_with_label(&mut children, EdgeLabel::Returns)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Returns) {
                errors.push(NodeCheckerError::new(format!("Expected returns to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // body

        {
            let body = &self.body;

            // Prepare edge label

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Body)
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
impl NodeChecker for ForStatement {
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
                extract_with_label(&mut children, EdgeLabel::ForKeyword)
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
                extract_with_label(&mut children, EdgeLabel::OpenParen)
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
                extract_with_label(&mut children, EdgeLabel::Initialization)
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
                extract_with_label(&mut children, EdgeLabel::Condition)
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
                extract_with_label(&mut children, EdgeLabel::Iterator)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Iterator) {
                errors.push(NodeCheckerError::new(format!("Expected iterator to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // close_paren

        {
            let close_paren = &self.close_paren;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_with_label(&mut children, EdgeLabel::CloseParen)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Body)
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
impl NodeChecker for FunctionCallExpression {
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
                extract_with_label(&mut children, EdgeLabel::Operand)
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
                extract_with_label(&mut children, EdgeLabel::Arguments)
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
impl NodeChecker for FunctionDefinition {
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
                extract_with_label(&mut children, EdgeLabel::FunctionKeyword)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name)
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
                extract_with_label(&mut children, EdgeLabel::Parameters)
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
                extract_with_label(&mut children, EdgeLabel::Attributes)
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
                extract_with_label(&mut children, EdgeLabel::Returns)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Returns) {
                errors.push(NodeCheckerError::new(format!("Expected returns to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // body

        {
            let body = &self.body;

            // Prepare edge label

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Body)
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
impl NodeChecker for FunctionType {
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
                extract_with_label(&mut children, EdgeLabel::FunctionKeyword)
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
                extract_with_label(&mut children, EdgeLabel::Parameters)
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
                extract_with_label(&mut children, EdgeLabel::Attributes)
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
                extract_with_label(&mut children, EdgeLabel::Returns)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Returns) {
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
impl NodeChecker for HexNumberExpression {
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
                extract_with_label(&mut children, EdgeLabel::Literal)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Unit)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Unit) {
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
impl NodeChecker for IfStatement {
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
                extract_with_label(&mut children, EdgeLabel::IfKeyword)
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
                extract_with_label(&mut children, EdgeLabel::OpenParen)
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
                extract_with_label(&mut children, EdgeLabel::Condition)
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
                extract_with_label(&mut children, EdgeLabel::CloseParen)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Body)
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
                extract_with_label(&mut children, EdgeLabel::ElseBranch)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::ElseBranch) {
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
impl NodeChecker for ImportAlias {
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
                extract_with_label(&mut children, EdgeLabel::AsKeyword)
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
                extract_with_label(&mut children, EdgeLabel::Identifier)
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
impl NodeChecker for ImportDeconstruction {
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
                extract_with_label(&mut children, EdgeLabel::OpenBrace)
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
                extract_with_label(&mut children, EdgeLabel::Symbols)
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
                extract_with_label(&mut children, EdgeLabel::CloseBrace)
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
                extract_with_label(&mut children, EdgeLabel::FromKeyword)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Path)
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
impl NodeChecker for ImportDeconstructionSymbol {
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Alias)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Alias) {
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
impl NodeChecker for ImportDirective {
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
                extract_with_label(&mut children, EdgeLabel::ImportKeyword)
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
                extract_with_label(&mut children, EdgeLabel::Clause)
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
                extract_with_label(&mut children, EdgeLabel::Semicolon)
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
impl NodeChecker for IndexAccessEnd {
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Colon)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::End) {
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::End) {
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
impl NodeChecker for IndexAccessExpression {
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
                extract_with_label(&mut children, EdgeLabel::Operand)
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
                extract_with_label(&mut children, EdgeLabel::OpenBracket)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Start)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Start) {
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::End) {
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::End) {
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
                extract_with_label(&mut children, EdgeLabel::CloseBracket)
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
impl NodeChecker for InequalityExpression {
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
                extract_with_label(&mut children, EdgeLabel::LeftOperand)
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
                extract_with_label(&mut children, EdgeLabel::Operator)
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
                extract_with_label(&mut children, EdgeLabel::RightOperand)
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
impl NodeChecker for InheritanceSpecifier {
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
                extract_with_label(&mut children, EdgeLabel::IsKeyword)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Types)
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
impl NodeChecker for InheritanceType {
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
                extract_with_label(&mut children, EdgeLabel::TypeName)
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
                extract_with_label(&mut children, EdgeLabel::Arguments)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Arguments) {
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
impl NodeChecker for InterfaceDefinition {
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
                extract_with_label(&mut children, EdgeLabel::InterfaceKeyword)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name)
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
                extract_with_label(&mut children, EdgeLabel::Inheritance)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Inheritance) {
                errors.push(NodeCheckerError::new(format!("Expected inheritance to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // open_brace

        {
            let open_brace = &self.open_brace;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_with_label(&mut children, EdgeLabel::OpenBrace)
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
                extract_with_label(&mut children, EdgeLabel::Members)
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
                extract_with_label(&mut children, EdgeLabel::CloseBrace)
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
impl NodeChecker for LibraryDefinition {
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
                extract_with_label(&mut children, EdgeLabel::LibraryKeyword)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name)
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
                extract_with_label(&mut children, EdgeLabel::OpenBrace)
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
                extract_with_label(&mut children, EdgeLabel::Members)
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
                extract_with_label(&mut children, EdgeLabel::CloseBrace)
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
impl NodeChecker for MappingKey {
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
                extract_with_label(&mut children, EdgeLabel::KeyType)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Name) {
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
impl NodeChecker for MappingType {
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
                extract_with_label(&mut children, EdgeLabel::MappingKeyword)
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
                extract_with_label(&mut children, EdgeLabel::OpenParen)
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
                extract_with_label(&mut children, EdgeLabel::KeyType)
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
                extract_with_label(&mut children, EdgeLabel::EqualGreaterThan)
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
                extract_with_label(&mut children, EdgeLabel::ValueType)
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
                extract_with_label(&mut children, EdgeLabel::CloseParen)
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
impl NodeChecker for MappingValue {
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
                extract_with_label(&mut children, EdgeLabel::TypeName)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Name) {
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
impl NodeChecker for MemberAccessExpression {
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
                extract_with_label(&mut children, EdgeLabel::Operand)
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
                extract_with_label(&mut children, EdgeLabel::Period)
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
                extract_with_label(&mut children, EdgeLabel::Member)
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
impl NodeChecker for ModifierDefinition {
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
                extract_with_label(&mut children, EdgeLabel::ModifierKeyword)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name)
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
                extract_with_label(&mut children, EdgeLabel::Parameters)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Parameters) {
                errors.push(NodeCheckerError::new(format!("Expected parameters to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // attributes

        {
            let attributes = &self.attributes;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_with_label(&mut children, EdgeLabel::Attributes)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Body)
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
impl NodeChecker for ModifierInvocation {
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name)
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
                extract_with_label(&mut children, EdgeLabel::Arguments)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Arguments) {
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

/// `NodeChecker` for `MultiTypedDeclaration` - maps to V1's `TupleDeconstructionStatement` (typed variant)
impl NodeChecker for MultiTypedDeclaration {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut children = children_with_offsets(node, text_offset);
        let mut errors = vec![];

        // open_paren
        if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::OpenParen)
        {
            errors.extend(
                self.open_paren
                    .check_node_with_offset(&child.node, child_offset),
            );
        } else {
            errors.push(NodeCheckerError::new(
                "Expected open_paren to be present in the CST, but it was not".to_string(),
                node_range.clone(),
            ));
        }

        // elements
        if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Elements)
        {
            errors.extend(
                self.elements
                    .check_node_with_offset(&child.node, child_offset),
            );
        } else {
            errors.push(NodeCheckerError::new(
                "Expected elements to be present in the CST, but it was not".to_string(),
                node_range.clone(),
            ));
        }

        // close_paren
        if let Some((child, child_offset)) =
            extract_with_label(&mut children, EdgeLabel::CloseParen)
        {
            errors.extend(
                self.close_paren
                    .check_node_with_offset(&child.node, child_offset),
            );
        } else {
            errors.push(NodeCheckerError::new(
                "Expected close_paren to be present in the CST, but it was not".to_string(),
                node_range.clone(),
            ));
        }

        // value (required) - contains equal and expression
        // V1 has equal and expression as direct children
        if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Equal) {
            errors.extend(
                self.value
                    .equal
                    .check_node_with_offset(&child.node, child_offset),
            );
        } else {
            errors.push(NodeCheckerError::new(
                "Expected equal to be present in the CST, but it was not".to_string(),
                node_range.clone(),
            ));
        }

        if let Some((child, child_offset)) =
            extract_with_label(&mut children, EdgeLabel::Expression)
        {
            errors.extend(
                self.value
                    .expression
                    .check_node_with_offset(&child.node, child_offset),
            );
        } else {
            errors.push(NodeCheckerError::new(
                "Expected expression to be present in the CST, but it was not".to_string(),
                node_range.clone(),
            ));
        }

        // Remove semicolon (checked by parent VariableDeclarationStatement)
        extract_with_label(&mut children, EdgeLabel::Semicolon);

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// `NodeChecker` for `MultiTypedDeclarationElement` - maps to V1's `TupleDeconstructionElement`
impl NodeChecker for MultiTypedDeclarationElement {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TupleDeconstructionElement) {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be TupleDeconstructionElement, but it was {}",
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);
        let mut errors = vec![];

        if let Some(member) = &self.member {
            if let Some((child, child_offset)) =
                extract_with_label(&mut children, EdgeLabel::Member)
            {
                errors.extend(member.check_node_with_offset(&child.node, child_offset));
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected member to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Member) {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected member to not be present in the CST, but it was there: {child:#?}"
                ),
                node_range.clone(),
            ));
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
impl NodeChecker for MultiplicativeExpression {
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
                extract_with_label(&mut children, EdgeLabel::LeftOperand)
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
                extract_with_label(&mut children, EdgeLabel::Operator)
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
                extract_with_label(&mut children, EdgeLabel::RightOperand)
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
impl NodeChecker for NamedArgument {
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Colon)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Value)
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
impl NodeChecker for NamedArgumentGroup {
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
                extract_with_label(&mut children, EdgeLabel::OpenBrace)
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
                extract_with_label(&mut children, EdgeLabel::Arguments)
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
                extract_with_label(&mut children, EdgeLabel::CloseBrace)
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
impl NodeChecker for NamedArgumentsDeclaration {
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
                extract_with_label(&mut children, EdgeLabel::OpenParen)
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
                extract_with_label(&mut children, EdgeLabel::Arguments)
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
                extract_with_label(&mut children, EdgeLabel::CloseParen)
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
impl NodeChecker for NamedImport {
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
                extract_with_label(&mut children, EdgeLabel::Asterisk)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Alias)
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
                extract_with_label(&mut children, EdgeLabel::FromKeyword)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Path)
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
impl NodeChecker for NewExpression {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::NewExpression) {
            // Don't even check the rest
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be {}, but it was {}",
                    NonterminalKind::NewExpression,
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);

        let mut errors = vec![];

        // new_keyword

        {
            let new_keyword = &self.new_keyword;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_with_label(&mut children, EdgeLabel::NewKeyword)
            {
                let child_errors = new_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected new_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // type_name

        {
            let type_name = &self.type_name;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_with_label(&mut children, EdgeLabel::TypeName)
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
impl NodeChecker for OrExpression {
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
                extract_with_label(&mut children, EdgeLabel::LeftOperand)
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
                extract_with_label(&mut children, EdgeLabel::Operator)
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
                extract_with_label(&mut children, EdgeLabel::RightOperand)
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
impl NodeChecker for OverridePathsDeclaration {
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
                extract_with_label(&mut children, EdgeLabel::OpenParen)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Paths)
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
                extract_with_label(&mut children, EdgeLabel::CloseParen)
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
impl NodeChecker for OverrideSpecifier {
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
                extract_with_label(&mut children, EdgeLabel::OverrideKeyword)
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
                extract_with_label(&mut children, EdgeLabel::Overridden)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Overridden) {
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
impl NodeChecker for Parameter {
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
                extract_with_label(&mut children, EdgeLabel::TypeName)
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
                extract_with_label(&mut children, EdgeLabel::StorageLocation)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::StorageLocation)
            {
                errors.push(NodeCheckerError::new(format!("Expected storage_location to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // name
        if let Some(name) = &self.name {
            // Prepare edge label

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Name) {
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
impl NodeChecker for ParametersDeclaration {
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
                extract_with_label(&mut children, EdgeLabel::OpenParen)
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
                extract_with_label(&mut children, EdgeLabel::Parameters)
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
                extract_with_label(&mut children, EdgeLabel::CloseParen)
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
impl NodeChecker for PathImport {
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Path)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Alias)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Alias) {
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
impl NodeChecker for PositionalArgumentsDeclaration {
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
                extract_with_label(&mut children, EdgeLabel::OpenParen)
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
                extract_with_label(&mut children, EdgeLabel::Arguments)
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
                extract_with_label(&mut children, EdgeLabel::CloseParen)
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
impl NodeChecker for PostfixExpression {
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
                extract_with_label(&mut children, EdgeLabel::Operand)
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
                extract_with_label(&mut children, EdgeLabel::Operator)
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
impl NodeChecker for PragmaDirective {
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
                extract_with_label(&mut children, EdgeLabel::PragmaKeyword)
            {
                let child_errors = pragma_keyword.check_node_with_offset(&child.node, child_offset);
                errors.extend(child_errors);
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected pragma_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // pragma

        {
            // Prepare edge label

            if extract_with_label(&mut children, EdgeLabel::Pragma).is_some() {
                // We don't check, since V2 can't parse these yet
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected pragma to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_with_label(&mut children, EdgeLabel::Semicolon)
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
impl NodeChecker for PrefixExpression {
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
                extract_with_label(&mut children, EdgeLabel::Operator)
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
                extract_with_label(&mut children, EdgeLabel::Operand)
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
impl NodeChecker for ReceiveFunctionDefinition {
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
                extract_with_label(&mut children, EdgeLabel::ReceiveKeyword)
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
                extract_with_label(&mut children, EdgeLabel::Parameters)
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
                extract_with_label(&mut children, EdgeLabel::Attributes)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Body)
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
impl NodeChecker for ReturnStatement {
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
                extract_with_label(&mut children, EdgeLabel::ReturnKeyword)
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
                extract_with_label(&mut children, EdgeLabel::Expression)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Expression) {
                errors.push(NodeCheckerError::new(format!("Expected expression to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_with_label(&mut children, EdgeLabel::Semicolon)
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
impl NodeChecker for ReturnsDeclaration {
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
                extract_with_label(&mut children, EdgeLabel::ReturnsKeyword)
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
                extract_with_label(&mut children, EdgeLabel::Variables)
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
impl NodeChecker for RevertStatement {
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
                extract_with_label(&mut children, EdgeLabel::RevertKeyword)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Error)
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
                extract_with_label(&mut children, EdgeLabel::Arguments)
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
                extract_with_label(&mut children, EdgeLabel::Semicolon)
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
impl NodeChecker for ShiftExpression {
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
                extract_with_label(&mut children, EdgeLabel::LeftOperand)
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
                extract_with_label(&mut children, EdgeLabel::Operator)
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
                extract_with_label(&mut children, EdgeLabel::RightOperand)
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

/// `NodeChecker` for `SingleTypedDeclaration` - maps to V1's `VariableDeclarationStatement` with `TypeName`
impl NodeChecker for SingleTypedDeclaration {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut children = children_with_offsets(node, text_offset);
        let mut errors = vec![];

        // V2's declaration field contains type_name, storage_location?, name
        // These map to V1's VariableDeclarationStatement fields directly

        // type_name (through VariableType in V1)
        if let Some((child, child_offset)) =
            extract_with_label(&mut children, EdgeLabel::VariableType)
        {
            // V1's VariableType is an enum with TypeName or VarKeyword variants
            let type_children = children_with_offsets(&child.node, child_offset);
            if let Some((variant_child, variant_offset)) = type_children.first() {
                errors.extend(
                    self.declaration
                        .type_name
                        .check_node_with_offset(&variant_child.node, *variant_offset),
                );
            }
        } else {
            errors.push(NodeCheckerError::new(
                "Expected VariableType to be present in the CST, but it was not".to_string(),
                node_range.clone(),
            ));
        }

        // storage_location (optional)
        if let Some(storage_location) = &self.declaration.storage_location {
            if let Some((child, child_offset)) =
                extract_with_label(&mut children, EdgeLabel::StorageLocation)
            {
                errors.extend(storage_location.check_node_with_offset(&child.node, child_offset));
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected storage_location to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        } else if let Some((child, _)) =
            extract_with_label(&mut children, EdgeLabel::StorageLocation)
        {
            errors.push(NodeCheckerError::new(
                format!("Expected storage_location to not be present in the CST, but it was there: {child:#?}"),
                node_range.clone(),
            ));
        }

        // name
        if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name) {
            errors.extend(
                self.declaration
                    .name
                    .check_node_with_offset(&child.node, child_offset),
            );
        } else {
            errors.push(NodeCheckerError::new(
                "Expected name to be present in the CST, but it was not".to_string(),
                node_range.clone(),
            ));
        }

        // value (optional) - V1's VariableDeclarationValue
        if let Some(value) = &self.value {
            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Value)
            {
                errors.extend(value.check_node_with_offset(&child.node, child_offset));
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected value to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Value) {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected value to not be present in the CST, but it was there: {child:#?}"
                ),
                node_range.clone(),
            ));
        }

        // Remove semicolon (checked by parent VariableDeclarationStatement)
        extract_with_label(&mut children, EdgeLabel::Semicolon);

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
impl NodeChecker for SourceUnit {
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
                extract_with_label(&mut children, EdgeLabel::Members)
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
impl NodeChecker for StateVariableDefinition {
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
                extract_with_label(&mut children, EdgeLabel::TypeName)
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
                extract_with_label(&mut children, EdgeLabel::Attributes)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Value)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Value) {
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
                extract_with_label(&mut children, EdgeLabel::Semicolon)
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
impl NodeChecker for StateVariableDefinitionValue {
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Equal)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Value)
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
impl NodeChecker for StorageLayoutSpecifier {
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
                extract_with_label(&mut children, EdgeLabel::LayoutKeyword)
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
                extract_with_label(&mut children, EdgeLabel::AtKeyword)
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
                extract_with_label(&mut children, EdgeLabel::Expression)
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
impl NodeChecker for StructDefinition {
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
                extract_with_label(&mut children, EdgeLabel::StructKeyword)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name)
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
                extract_with_label(&mut children, EdgeLabel::OpenBrace)
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
                extract_with_label(&mut children, EdgeLabel::Members)
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
                extract_with_label(&mut children, EdgeLabel::CloseBrace)
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
impl NodeChecker for StructMember {
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
                extract_with_label(&mut children, EdgeLabel::TypeName)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name)
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
                extract_with_label(&mut children, EdgeLabel::Semicolon)
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
impl NodeChecker for ThrowStatement {
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
                extract_with_label(&mut children, EdgeLabel::ThrowKeyword)
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
                extract_with_label(&mut children, EdgeLabel::Semicolon)
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
impl NodeChecker for TryStatement {
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
                extract_with_label(&mut children, EdgeLabel::TryKeyword)
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
                extract_with_label(&mut children, EdgeLabel::Expression)
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
                extract_with_label(&mut children, EdgeLabel::Returns)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Returns) {
                errors.push(NodeCheckerError::new(format!("Expected returns to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // body

        {
            let body = &self.body;

            // Prepare edge label

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Body)
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
                extract_with_label(&mut children, EdgeLabel::CatchClauses)
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
impl NodeChecker for TupleExpression {
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
                extract_with_label(&mut children, EdgeLabel::OpenParen)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Items)
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
                extract_with_label(&mut children, EdgeLabel::CloseParen)
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
impl NodeChecker for TupleValue {
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
                extract_with_label(&mut children, EdgeLabel::Expression)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Expression) {
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
impl NodeChecker for TypeExpression {
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
                extract_with_label(&mut children, EdgeLabel::TypeKeyword)
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
                extract_with_label(&mut children, EdgeLabel::OpenParen)
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
                extract_with_label(&mut children, EdgeLabel::TypeName)
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
                extract_with_label(&mut children, EdgeLabel::CloseParen)
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
impl NodeChecker for UncheckedBlock {
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
                extract_with_label(&mut children, EdgeLabel::UncheckedKeyword)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Block)
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
impl NodeChecker for UnnamedFunctionDefinition {
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
                extract_with_label(&mut children, EdgeLabel::FunctionKeyword)
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
                extract_with_label(&mut children, EdgeLabel::Parameters)
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
                extract_with_label(&mut children, EdgeLabel::Attributes)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Body)
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

/// `NodeChecker` for `UntypedDeclaration` - maps to V1's `VariableDeclarationStatement` with var OR `TupleDeconstructionStatement` with var
impl NodeChecker for UntypedDeclaration {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut children = children_with_offsets(node, text_offset);
        let mut errors = vec![];

        let is_v1_var_decl =
            node.kind() == NodeKind::Nonterminal(NonterminalKind::VariableDeclarationStatement);
        let is_v1_tuple_deconstruct =
            node.kind() == NodeKind::Nonterminal(NonterminalKind::TupleDeconstructionStatement);

        if is_v1_var_decl {
            // Single var declaration: var x = ...
            // var_keyword is inside VariableType
            if let Some((child, child_offset)) =
                extract_with_label(&mut children, EdgeLabel::VariableType)
            {
                let type_children = children_with_offsets(&child.node, child_offset);
                if let Some((variant_child, variant_offset)) = type_children.first() {
                    errors.extend(
                        self.var_keyword
                            .check_node_with_offset(&variant_child.node, *variant_offset),
                    );
                }
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected VariableType to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }

            // names - should be Identifier variant
            errors.extend(self.names.check_node_with_offset(node, text_offset));

            // value - V1 has Value child containing VariableDeclarationValue
            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Value)
            {
                errors.extend(self.value.check_node_with_offset(&child.node, child_offset));
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected value to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }

            // Remove name and storage_location (handled by names checker)
            extract_with_label(&mut children, EdgeLabel::Name);
            extract_with_label(&mut children, EdgeLabel::StorageLocation);
        } else if is_v1_tuple_deconstruct {
            // Tuple var declaration: var (a, b) = ...
            // var_keyword
            if let Some((child, child_offset)) =
                extract_with_label(&mut children, EdgeLabel::VarKeyword)
            {
                errors.extend(
                    self.var_keyword
                        .check_node_with_offset(&child.node, child_offset),
                );
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected var_keyword to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }

            // names - should be UntypedTupleDeclaration variant
            errors.extend(self.names.check_node_with_offset(node, text_offset));

            // Remove open_paren, elements, close_paren (handled by names checker)
            extract_with_label(&mut children, EdgeLabel::OpenParen);
            extract_with_label(&mut children, EdgeLabel::Elements);
            extract_with_label(&mut children, EdgeLabel::CloseParen);

            // value - V1 has equal and expression directly
            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Equal)
            {
                errors.extend(
                    self.value
                        .equal
                        .check_node_with_offset(&child.node, child_offset),
                );
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected equal to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }

            if let Some((child, child_offset)) =
                extract_with_label(&mut children, EdgeLabel::Expression)
            {
                errors.extend(
                    self.value
                        .expression
                        .check_node_with_offset(&child.node, child_offset),
                );
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected expression to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        }

        // Remove semicolon (checked by parent VariableDeclarationStatement)
        extract_with_label(&mut children, EdgeLabel::Semicolon);

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// `NodeChecker` for `UntypedTupleDeclaration` - maps to V1's `TupleDeconstructionStatement` tuple part
impl NodeChecker for UntypedTupleDeclaration {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());
        let mut children = children_with_offsets(node, text_offset);
        let mut errors = vec![];

        // open_paren
        if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::OpenParen)
        {
            errors.extend(
                self.open_paren
                    .check_node_with_offset(&child.node, child_offset),
            );
        } else {
            errors.push(NodeCheckerError::new(
                "Expected open_paren to be present in the CST, but it was not".to_string(),
                node_range.clone(),
            ));
        }

        // elements
        if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Elements)
        {
            errors.extend(
                self.elements
                    .check_node_with_offset(&child.node, child_offset),
            );
        } else {
            errors.push(NodeCheckerError::new(
                "Expected elements to be present in the CST, but it was not".to_string(),
                node_range.clone(),
            ));
        }

        // close_paren
        if let Some((child, child_offset)) =
            extract_with_label(&mut children, EdgeLabel::CloseParen)
        {
            errors.extend(
                self.close_paren
                    .check_node_with_offset(&child.node, child_offset),
            );
        } else {
            errors.push(NodeCheckerError::new(
                "Expected close_paren to be present in the CST, but it was not".to_string(),
                node_range.clone(),
            ));
        }

        // Remove fields that belong to parent (checked elsewhere)
        extract_with_label(&mut children, EdgeLabel::VarKeyword);
        extract_with_label(&mut children, EdgeLabel::Equal);
        extract_with_label(&mut children, EdgeLabel::Expression);
        extract_with_label(&mut children, EdgeLabel::Semicolon);

        if !children.is_empty() {
            errors.push(NodeCheckerError::new(
                format!("Expected 0 children left, but there's some left {children:#?}"),
                node_range,
            ));
        }

        errors
    }
}

/// `NodeChecker` for `UntypedTupleDeclarationElement` - maps to V1's `TupleDeconstructionElement` (untyped)
impl NodeChecker for UntypedTupleDeclarationElement {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TupleDeconstructionElement) {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be TupleDeconstructionElement, but it was {}",
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);
        let mut errors = vec![];

        // V2's UntypedTupleDeclarationElement has `name: Identifier?`
        // V1's TupleDeconstructionElement has `member: TupleMember?` where TupleMember can be UntypedTupleMember
        if let Some(name) = &self.name {
            if let Some((child, child_offset)) =
                extract_with_label(&mut children, EdgeLabel::Member)
            {
                // The child should be a TupleMember -> UntypedTupleMember -> name
                // We need to navigate through TupleMember to get to UntypedTupleMember
                let member_children = children_with_offsets(&child.node, child_offset);
                if let Some((variant_child, variant_offset)) = member_children.first() {
                    // This should be the UntypedTupleMember
                    let untyped_children =
                        children_with_offsets(&variant_child.node, *variant_offset);
                    // Find the Name edge in UntypedTupleMember
                    for (untyped_child, untyped_child_offset) in untyped_children {
                        if untyped_child.label == EdgeLabel::Name {
                            errors.extend(
                                name.check_node_with_offset(
                                    &untyped_child.node,
                                    untyped_child_offset,
                                ),
                            );
                        }
                    }
                }
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected member to be present in the CST, but it was not".to_string(),
                    node_range.clone(),
                ));
            }
        } else if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Member) {
            errors.push(NodeCheckerError::new(
                format!(
                    "Expected member to not be present in the CST, but it was there: {child:#?}"
                ),
                node_range.clone(),
            ));
        }

        errors
    }
}

/// Generic `NodeChecker` for sequences
impl NodeChecker for UserDefinedValueTypeDefinition {
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
                extract_with_label(&mut children, EdgeLabel::TypeKeyword)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name)
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
                extract_with_label(&mut children, EdgeLabel::IsKeyword)
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
                extract_with_label(&mut children, EdgeLabel::ValueType)
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
                extract_with_label(&mut children, EdgeLabel::Semicolon)
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
impl NodeChecker for UsingAlias {
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
                extract_with_label(&mut children, EdgeLabel::AsKeyword)
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
                extract_with_label(&mut children, EdgeLabel::Operator)
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
impl NodeChecker for UsingDeconstruction {
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
                extract_with_label(&mut children, EdgeLabel::OpenBrace)
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
                extract_with_label(&mut children, EdgeLabel::Symbols)
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
                extract_with_label(&mut children, EdgeLabel::CloseBrace)
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
impl NodeChecker for UsingDeconstructionSymbol {
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Alias)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Alias) {
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
impl NodeChecker for UsingDirective {
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
                extract_with_label(&mut children, EdgeLabel::UsingKeyword)
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
                extract_with_label(&mut children, EdgeLabel::Clause)
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
                extract_with_label(&mut children, EdgeLabel::ForKeyword)
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
                extract_with_label(&mut children, EdgeLabel::Target)
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
                extract_with_label(&mut children, EdgeLabel::GlobalKeyword)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::GlobalKeyword) {
                errors.push(NodeCheckerError::new(format!("Expected global_keyword to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // semicolon

        {
            let semicolon = &self.semicolon;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_with_label(&mut children, EdgeLabel::Semicolon)
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

/// `NodeChecker` for `VariableDeclaration` - maps to V1's `TypedTupleMember`
impl NodeChecker for VariableDeclaration {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        // V1 has TupleMember -> TypedTupleMember, so we need to check if this is a TupleMember first
        // and then check the variant
        if node.kind() == NodeKind::Nonterminal(NonterminalKind::TupleMember) {
            let children = children_with_offsets(node, text_offset);
            if let Some((variant_child, variant_offset)) = children.first() {
                return self.check_node_with_offset(&variant_child.node, *variant_offset);
            }
        }

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TypedTupleMember) {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be TypedTupleMember, but it was {}",
                    node.kind()
                ),
                node_range,
            )];
        }

        let mut children = children_with_offsets(node, text_offset);
        let mut errors = vec![];

        // type_name
        if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::TypeName)
        {
            errors.extend(
                self.type_name
                    .check_node_with_offset(&child.node, child_offset),
            );
        } else {
            errors.push(NodeCheckerError::new(
                "Expected type_name to be present in the CST, but it was not".to_string(),
                node_range.clone(),
            ));
        }

        // storage_location (optional)
        if let Some(storage_location) = &self.storage_location {
            if let Some((child, child_offset)) =
                extract_with_label(&mut children, EdgeLabel::StorageLocation)
            {
                errors.extend(storage_location.check_node_with_offset(&child.node, child_offset));
            } else {
                errors.push(NodeCheckerError::new(
                    "Expected storage_location to be present in the CST, but it was not"
                        .to_string(),
                    node_range.clone(),
                ));
            }
        } else if let Some((child, _)) =
            extract_with_label(&mut children, EdgeLabel::StorageLocation)
        {
            errors.push(NodeCheckerError::new(
                format!("Expected storage_location to not be present in the CST, but it was there: {child:#?}"),
                node_range.clone(),
            ));
        }

        // name
        if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name) {
            errors.extend(self.name.check_node_with_offset(&child.node, child_offset));
        } else {
            errors.push(NodeCheckerError::new(
                "Expected name to be present in the CST, but it was not".to_string(),
                node_range.clone(),
            ));
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

/// `NodeChecker` for `VariableDeclarationStatement` - V2 consolidates V1's `VariableDeclarationStatement` and `TupleDeconstructionStatement`
impl NodeChecker for VariableDeclarationStatement {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        // V2's VariableDeclarationStatement can map to either V1's VariableDeclarationStatement
        // or V1's TupleDeconstructionStatement depending on the target variant
        let is_v1_var_decl =
            node.kind() == NodeKind::Nonterminal(NonterminalKind::VariableDeclarationStatement);
        let is_v1_tuple_deconstruct =
            node.kind() == NodeKind::Nonterminal(NonterminalKind::TupleDeconstructionStatement);

        if !is_v1_var_decl && !is_v1_tuple_deconstruct {
            return vec![NodeCheckerError::new(format!(
                "Expected node kind to be VariableDeclarationStatement or TupleDeconstructionStatement, but it was {}",
                node.kind()
            ), node_range)];
        }

        let mut children = children_with_offsets(node, text_offset);
        let mut errors = vec![];

        // Delegate to target which handles mapping to V1's structure
        errors.extend(self.target.check_node_with_offset(node, text_offset));

        // Remove fields that target already checked (depends on V1 node type)
        if is_v1_var_decl {
            extract_with_label(&mut children, EdgeLabel::VariableType);
            extract_with_label(&mut children, EdgeLabel::StorageLocation);
            extract_with_label(&mut children, EdgeLabel::Name);
            extract_with_label(&mut children, EdgeLabel::Value);
        } else {
            extract_with_label(&mut children, EdgeLabel::VarKeyword);
            extract_with_label(&mut children, EdgeLabel::OpenParen);
            extract_with_label(&mut children, EdgeLabel::Elements);
            extract_with_label(&mut children, EdgeLabel::CloseParen);
            extract_with_label(&mut children, EdgeLabel::Equal);
            extract_with_label(&mut children, EdgeLabel::Expression);
        }

        // semicolon
        if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Semicolon)
        {
            errors.extend(
                self.semicolon
                    .check_node_with_offset(&child.node, child_offset),
            );
        } else {
            errors.push(NodeCheckerError::new(
                "Expected semicolon to be present in the CST, but it was not".to_string(),
                node_range.clone(),
            ));
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
impl NodeChecker for VariableDeclarationValue {
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Equal)
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
                extract_with_label(&mut children, EdgeLabel::Expression)
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
impl NodeChecker for VersionPragma {
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
                extract_with_label(&mut children, EdgeLabel::SolidityKeyword)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Sets)
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
impl NodeChecker for VersionRange {
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Start)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Minus)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::End) {
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
impl NodeChecker for VersionTerm {
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
                extract_with_label(&mut children, EdgeLabel::Operator)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Operator) {
                errors.push(NodeCheckerError::new(format!("Expected operator to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // literal

        {
            let literal = &self.literal;

            // Prepare edge label

            if let Some((child, child_offset)) =
                extract_with_label(&mut children, EdgeLabel::Literal)
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
impl NodeChecker for WhileStatement {
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
                extract_with_label(&mut children, EdgeLabel::WhileKeyword)
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
                extract_with_label(&mut children, EdgeLabel::OpenParen)
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
                extract_with_label(&mut children, EdgeLabel::Condition)
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
                extract_with_label(&mut children, EdgeLabel::CloseParen)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Body)
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
impl NodeChecker for YulBlock {
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
                extract_with_label(&mut children, EdgeLabel::OpenBrace)
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
            // Prepare edge label

            if extract_with_label(&mut children, EdgeLabel::Statements).is_some() {
                // We don't check statements, since V2 can't parse them yet
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
                extract_with_label(&mut children, EdgeLabel::CloseBrace)
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
impl NodeChecker for YulBreakStatement {
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
                extract_with_label(&mut children, EdgeLabel::BreakKeyword)
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
impl NodeChecker for YulColonAndEqual {
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Colon)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Equal)
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
impl NodeChecker for YulContinueStatement {
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
                extract_with_label(&mut children, EdgeLabel::ContinueKeyword)
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
impl NodeChecker for YulDefaultCase {
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
                extract_with_label(&mut children, EdgeLabel::DefaultKeyword)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Body)
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
impl NodeChecker for YulEqualAndColon {
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Equal)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Colon)
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
impl NodeChecker for YulForStatement {
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
                extract_with_label(&mut children, EdgeLabel::ForKeyword)
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
                extract_with_label(&mut children, EdgeLabel::Initialization)
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
                extract_with_label(&mut children, EdgeLabel::Condition)
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
                extract_with_label(&mut children, EdgeLabel::Iterator)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Body)
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
impl NodeChecker for YulFunctionCallExpression {
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
                extract_with_label(&mut children, EdgeLabel::Operand)
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
                extract_with_label(&mut children, EdgeLabel::OpenParen)
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
                extract_with_label(&mut children, EdgeLabel::Arguments)
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
                extract_with_label(&mut children, EdgeLabel::CloseParen)
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
impl NodeChecker for YulFunctionDefinition {
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
                extract_with_label(&mut children, EdgeLabel::FunctionKeyword)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Name)
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
                extract_with_label(&mut children, EdgeLabel::Parameters)
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
                extract_with_label(&mut children, EdgeLabel::Returns)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Returns) {
                errors.push(NodeCheckerError::new(format!("Expected returns to not be present in the CST, but it was there: {child:#?}"), node_range.clone()));
            }
        }

        // body

        {
            let body = &self.body;

            // Prepare edge label

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Body)
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
impl NodeChecker for YulIfStatement {
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
                extract_with_label(&mut children, EdgeLabel::IfKeyword)
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
                extract_with_label(&mut children, EdgeLabel::Condition)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Body)
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
impl NodeChecker for YulLabel {
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Label)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Colon)
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
impl NodeChecker for YulLeaveStatement {
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
                extract_with_label(&mut children, EdgeLabel::LeaveKeyword)
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
impl NodeChecker for YulParametersDeclaration {
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
                extract_with_label(&mut children, EdgeLabel::OpenParen)
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
                extract_with_label(&mut children, EdgeLabel::Parameters)
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
                extract_with_label(&mut children, EdgeLabel::CloseParen)
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
impl NodeChecker for YulReturnsDeclaration {
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
                extract_with_label(&mut children, EdgeLabel::MinusGreaterThan)
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
                extract_with_label(&mut children, EdgeLabel::Variables)
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
impl NodeChecker for YulStackAssignmentStatement {
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
                extract_with_label(&mut children, EdgeLabel::Assignment)
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
                extract_with_label(&mut children, EdgeLabel::Variable)
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
impl NodeChecker for YulSwitchStatement {
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
                extract_with_label(&mut children, EdgeLabel::SwitchKeyword)
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
                extract_with_label(&mut children, EdgeLabel::Expression)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Cases)
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
impl NodeChecker for YulValueCase {
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
                extract_with_label(&mut children, EdgeLabel::CaseKeyword)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Value)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Body)
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
impl NodeChecker for YulVariableAssignmentStatement {
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
                extract_with_label(&mut children, EdgeLabel::Variables)
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
                extract_with_label(&mut children, EdgeLabel::Assignment)
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
                extract_with_label(&mut children, EdgeLabel::Expression)
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
impl NodeChecker for YulVariableDeclarationStatement {
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
                extract_with_label(&mut children, EdgeLabel::LetKeyword)
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
                extract_with_label(&mut children, EdgeLabel::Variables)
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

            if let Some((child, child_offset)) = extract_with_label(&mut children, EdgeLabel::Value)
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
            if let Some((child, _)) = extract_with_label(&mut children, EdgeLabel::Value) {
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
impl NodeChecker for YulVariableDeclarationValue {
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
                extract_with_label(&mut children, EdgeLabel::Assignment)
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
                extract_with_label(&mut children, EdgeLabel::Expression)
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
impl NodeChecker for AbicoderVersion {
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
impl NodeChecker for ArgumentsDeclaration {
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
impl NodeChecker for ConstructorAttribute {
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
impl NodeChecker for ContractMember {
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
impl NodeChecker for ContractSpecifier {
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
impl NodeChecker for ElementaryType {
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
impl NodeChecker for ExperimentalFeature {
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
impl NodeChecker for Expression {
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
impl NodeChecker for Expression_AdditiveExpression_Operator {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        match self {
            Self::Minus(element) => element.check_node_with_offset(node, text_offset),

            Self::Plus(element) => element.check_node_with_offset(node, text_offset),
        }
    }
}

// Special case for operator choices that are merged together
impl NodeChecker for Expression_AssignmentExpression_Operator {
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
impl NodeChecker for Expression_EqualityExpression_Operator {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        match self {
            Self::BangEqual(element) => element.check_node_with_offset(node, text_offset),

            Self::EqualEqual(element) => element.check_node_with_offset(node, text_offset),
        }
    }
}

// Special case for operator choices that are merged together
impl NodeChecker for Expression_ExponentiationExpression_Operator {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        match self {
            Self::AsteriskAsterisk(element) => element.check_node_with_offset(node, text_offset),
        }
    }
}

// Special case for operator choices that are merged together
impl NodeChecker for Expression_InequalityExpression_Operator {
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
impl NodeChecker for Expression_MultiplicativeExpression_Operator {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        match self {
            Self::Asterisk(element) => element.check_node_with_offset(node, text_offset),

            Self::Percent(element) => element.check_node_with_offset(node, text_offset),

            Self::Slash(element) => element.check_node_with_offset(node, text_offset),
        }
    }
}

// Special case for operator choices that are merged together
impl NodeChecker for Expression_PostfixExpression_Operator {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        match self {
            Self::MinusMinus(element) => element.check_node_with_offset(node, text_offset),

            Self::PlusPlus(element) => element.check_node_with_offset(node, text_offset),
        }
    }
}

// Special case for operator choices that are merged together
impl NodeChecker for Expression_PrefixExpression_Operator {
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
impl NodeChecker for Expression_ShiftExpression_Operator {
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
impl NodeChecker for FallbackFunctionAttribute {
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
impl NodeChecker for ForStatementCondition {
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
impl NodeChecker for ForStatementInitialization {
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
impl NodeChecker for FunctionAttribute {
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
impl NodeChecker for FunctionBody {
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
impl NodeChecker for FunctionName {
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
impl NodeChecker for FunctionTypeAttribute {
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
impl NodeChecker for HexStringLiteral {
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

/// `NodeChecker` for `IdentifierPathElement` is done by hand since it's not present in V1
impl NodeChecker for IdentifierPathElement {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        match self {
            Self::Identifier(element) => element.check_node_with_offset(node, text_offset),
            Self::AddressKeyword(element) => {
                let ident = Identifier {
                    range: element.range.clone(),
                };
                ident.check_node_with_offset(node, text_offset)
            }
        }
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for ImportClause {
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
impl NodeChecker for MappingKeyType {
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

/// Generic `NodeChecker` for choices
impl NodeChecker for ModifierAttribute {
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
impl NodeChecker for NumberUnit {
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
impl NodeChecker for Pragma {
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
impl NodeChecker for ReceiveFunctionAttribute {
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
impl NodeChecker for SourceUnitMember {
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
impl NodeChecker for StateVariableAttribute {
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
impl NodeChecker for Statement {
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
impl NodeChecker for StorageLocation {
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
impl NodeChecker for StringExpression {
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
impl NodeChecker for StringLiteral {
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
impl NodeChecker for TypeName {
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
impl NodeChecker for UnicodeStringLiteral {
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
impl NodeChecker for UnnamedFunctionAttribute {
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
        }

        errors
    }
}

/// `NodeChecker` for `UntypedDeclarationNames` - no direct V1 equivalent, delegate to variants
impl NodeChecker for UntypedDeclarationNames {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let mut children = children_with_offsets(node, text_offset);

        match self {
            Self::Identifier(element) => {
                // For single var declaration: var x = ...
                // The name is directly in V1's VariableDeclarationStatement
                if let Some((child, child_offset)) =
                    extract_with_label(&mut children, EdgeLabel::Name)
                {
                    element.check_node_with_offset(&child.node, child_offset)
                } else {
                    vec![NodeCheckerError::new(
                        "Expected name to be present in the CST, but it was not".to_string(),
                        text_offset..(text_offset + node.text_len()),
                    )]
                }
            }
            Self::UntypedTupleDeclaration(element) => {
                // For tuple var declaration: var (a, b) = ...
                // Delegate to the tuple names checker
                element.check_node_with_offset(node, text_offset)
            }
        }
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for UsingClause {
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
impl NodeChecker for UsingOperator {
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
impl NodeChecker for UsingTarget {
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

/// `NodeChecker` for `VariableDeclarationTarget` - no direct V1 equivalent, delegate to variants
impl NodeChecker for VariableDeclarationTarget {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        // VariableDeclarationTarget doesn't exist in V1, so we skip the kind check
        // and directly delegate to the variant's checker
        match self {
            Self::SingleTypedDeclaration(element) => {
                element.check_node_with_offset(node, text_offset)
            }
            Self::MultiTypedDeclaration(element) => {
                element.check_node_with_offset(node, text_offset)
            }
            Self::UntypedDeclaration(element) => element.check_node_with_offset(node, text_offset),
        }
    }
}

/// Generic `NodeChecker` for choices
impl NodeChecker for VersionExpression {
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
impl NodeChecker for VersionLiteral {
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
impl NodeChecker for VersionOperator {
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
impl NodeChecker for YulAssignmentOperator {
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
impl NodeChecker for YulExpression {
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
impl NodeChecker for YulLiteral {
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
impl NodeChecker for YulStackAssignmentOperator {
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
impl NodeChecker for YulStatement {
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
impl NodeChecker for YulSwitchCase {
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
impl NodeChecker for ArrayValues {
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
impl NodeChecker for AssemblyFlags {
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
impl NodeChecker for CallOptions {
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
impl NodeChecker for CatchClauses {
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
impl NodeChecker for ConstructorAttributes {
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
impl NodeChecker for ContractMembers {
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
impl NodeChecker for ContractSpecifiers {
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
impl NodeChecker for EnumMembers {
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
impl NodeChecker for ErrorParameters {
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
impl NodeChecker for EventParameters {
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
impl NodeChecker for FallbackFunctionAttributes {
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
impl NodeChecker for FunctionAttributes {
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
impl NodeChecker for FunctionTypeAttributes {
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
impl NodeChecker for HexStringLiterals {
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

/// Generic `NodeChecker` for repeated and separated
impl NodeChecker for IdentifierPath {
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
impl NodeChecker for ImportDeconstructionSymbols {
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
impl NodeChecker for InheritanceTypes {
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
impl NodeChecker for InterfaceMembers {
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
impl NodeChecker for LibraryMembers {
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
impl NodeChecker for ModifierAttributes {
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

/// `NodeChecker` for `MultiTypedDeclarationElements` maps to V1's `TupleDeconstructionElements`
impl NodeChecker for MultiTypedDeclarationElements {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TupleDeconstructionElements) {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be TupleDeconstructionElements, but it was {}",
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
impl NodeChecker for NamedArguments {
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
impl NodeChecker for OverridePaths {
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
impl NodeChecker for Parameters {
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
impl NodeChecker for PositionalArguments {
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
impl NodeChecker for ReceiveFunctionAttributes {
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
impl NodeChecker for SimpleVersionLiteral {
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
impl NodeChecker for SourceUnitMembers {
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
impl NodeChecker for StateVariableAttributes {
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
impl NodeChecker for Statements {
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
impl NodeChecker for StringLiterals {
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
impl NodeChecker for StructMembers {
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
impl NodeChecker for TupleValues {
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
impl NodeChecker for UnicodeStringLiterals {
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
impl NodeChecker for UnnamedFunctionAttributes {
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

/// `NodeChecker` for `UntypedTupleDeclarationElements` maps to V1's `TupleDeconstructionElements`
impl NodeChecker for UntypedTupleDeclarationElements {
    fn check_node_with_offset(&self, node: &Node, text_offset: TextIndex) -> Vec<NodeCheckerError> {
        let node_range = text_offset..(text_offset + node.text_len());

        if node.kind() != NodeKind::Nonterminal(NonterminalKind::TupleDeconstructionElements) {
            return vec![NodeCheckerError::new(
                format!(
                    "Expected node kind to be TupleDeconstructionElements, but it was {}",
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
impl NodeChecker for UsingDeconstructionSymbols {
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
impl NodeChecker for VersionExpressionSet {
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
impl NodeChecker for VersionExpressionSets {
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
impl NodeChecker for YulArguments {
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
impl NodeChecker for YulParameters {
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
impl NodeChecker for YulPath {
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
impl NodeChecker for YulPaths {
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
impl NodeChecker for YulStatements {
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
impl NodeChecker for YulSwitchCases {
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
impl NodeChecker for YulVariableNames {
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
impl NodeChecker for ABIEncoderV2Keyword {
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
impl NodeChecker for AbicoderKeyword {
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
impl NodeChecker for AbicoderV1Keyword {
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
impl NodeChecker for AbicoderV2Keyword {
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
impl NodeChecker for AbstractKeyword {
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
impl NodeChecker for AddressKeyword {
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
impl NodeChecker for AfterKeyword {
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
impl NodeChecker for AliasKeyword {
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
impl NodeChecker for Ampersand {
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
impl NodeChecker for AmpersandAmpersand {
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
impl NodeChecker for AmpersandEqual {
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
impl NodeChecker for AnonymousKeyword {
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
impl NodeChecker for ApplyKeyword {
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
impl NodeChecker for AsKeyword {
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
impl NodeChecker for AssemblyKeyword {
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
impl NodeChecker for Asterisk {
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
impl NodeChecker for AsteriskAsterisk {
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
impl NodeChecker for AsteriskEqual {
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
impl NodeChecker for AtKeyword {
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
impl NodeChecker for AutoKeyword {
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
impl NodeChecker for Bang {
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
impl NodeChecker for BangEqual {
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
impl NodeChecker for Bar {
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
impl NodeChecker for BarBar {
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
impl NodeChecker for BarEqual {
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
impl NodeChecker for BoolKeyword {
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
impl NodeChecker for BreakKeyword {
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
impl NodeChecker for ByteKeyword {
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
impl NodeChecker for BytesKeyword {
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
impl NodeChecker for CallDataKeyword {
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
impl NodeChecker for Caret {
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
impl NodeChecker for CaretEqual {
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
impl NodeChecker for CaseKeyword {
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
impl NodeChecker for CatchKeyword {
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
impl NodeChecker for CloseBrace {
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
impl NodeChecker for CloseBracket {
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
impl NodeChecker for CloseParen {
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
impl NodeChecker for Colon {
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
impl NodeChecker for ColonEqual {
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
impl NodeChecker for Comma {
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
impl NodeChecker for ConstantKeyword {
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
impl NodeChecker for ConstructorKeyword {
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
impl NodeChecker for ContinueKeyword {
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
impl NodeChecker for ContractKeyword {
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
impl NodeChecker for CopyOfKeyword {
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
impl NodeChecker for DaysKeyword {
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
impl NodeChecker for DecimalLiteral {
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
impl NodeChecker for DefaultKeyword {
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
impl NodeChecker for DefineKeyword {
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
impl NodeChecker for DeleteKeyword {
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
impl NodeChecker for DoKeyword {
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
impl NodeChecker for DoubleQuotedHexStringLiteral {
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
impl NodeChecker for DoubleQuotedStringLiteral {
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
impl NodeChecker for DoubleQuotedUnicodeStringLiteral {
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
impl NodeChecker for DoubleQuotedVersionLiteral {
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
impl NodeChecker for ElseKeyword {
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
impl NodeChecker for EmitKeyword {
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
impl NodeChecker for EndOfLine {
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
impl NodeChecker for EnumKeyword {
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
impl NodeChecker for Equal {
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
impl NodeChecker for EqualColon {
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
impl NodeChecker for EqualEqual {
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
impl NodeChecker for EqualGreaterThan {
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
impl NodeChecker for ErrorKeyword {
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
impl NodeChecker for EtherKeyword {
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
impl NodeChecker for EventKeyword {
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
impl NodeChecker for ExperimentalKeyword {
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
impl NodeChecker for ExternalKeyword {
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
impl NodeChecker for FallbackKeyword {
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
impl NodeChecker for FalseKeyword {
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
impl NodeChecker for FinalKeyword {
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
impl NodeChecker for FinneyKeyword {
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
impl NodeChecker for FixedKeyword {
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
impl NodeChecker for ForKeyword {
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
impl NodeChecker for FromKeyword {
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
impl NodeChecker for FunctionKeyword {
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
impl NodeChecker for GlobalKeyword {
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
impl NodeChecker for GreaterThan {
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
impl NodeChecker for GreaterThanEqual {
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
impl NodeChecker for GreaterThanGreaterThan {
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
impl NodeChecker for GreaterThanGreaterThanEqual {
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
impl NodeChecker for GreaterThanGreaterThanGreaterThan {
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
impl NodeChecker for GreaterThanGreaterThanGreaterThanEqual {
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
impl NodeChecker for GweiKeyword {
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
impl NodeChecker for HexKeyword {
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
impl NodeChecker for HexLiteral {
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
impl NodeChecker for HoursKeyword {
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
impl NodeChecker for Identifier {
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
impl NodeChecker for IfKeyword {
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
impl NodeChecker for ImmutableKeyword {
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
impl NodeChecker for ImplementsKeyword {
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
impl NodeChecker for ImportKeyword {
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
impl NodeChecker for InKeyword {
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
impl NodeChecker for IndexedKeyword {
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
impl NodeChecker for InlineKeyword {
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
impl NodeChecker for IntKeyword {
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
impl NodeChecker for InterfaceKeyword {
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
impl NodeChecker for InternalKeyword {
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
impl NodeChecker for IsKeyword {
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
impl NodeChecker for LayoutKeyword {
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
impl NodeChecker for LessThan {
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
impl NodeChecker for LessThanEqual {
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
impl NodeChecker for LessThanLessThan {
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
impl NodeChecker for LessThanLessThanEqual {
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
impl NodeChecker for LetKeyword {
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
impl NodeChecker for LibraryKeyword {
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
impl NodeChecker for MacroKeyword {
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
impl NodeChecker for MappingKeyword {
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
impl NodeChecker for MatchKeyword {
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
impl NodeChecker for MemoryKeyword {
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
impl NodeChecker for Minus {
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
impl NodeChecker for MinusEqual {
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
impl NodeChecker for MinusGreaterThan {
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
impl NodeChecker for MinusMinus {
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
impl NodeChecker for MinutesKeyword {
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
impl NodeChecker for ModifierKeyword {
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
impl NodeChecker for MultiLineComment {
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
impl NodeChecker for MultiLineNatSpecComment {
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
impl NodeChecker for MutableKeyword {
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
impl NodeChecker for NewKeyword {
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
impl NodeChecker for NullKeyword {
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
impl NodeChecker for OfKeyword {
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
impl NodeChecker for OpenBrace {
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
impl NodeChecker for OpenBracket {
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
impl NodeChecker for OpenParen {
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
impl NodeChecker for OverrideKeyword {
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
impl NodeChecker for PartialKeyword {
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
impl NodeChecker for PayableKeyword {
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
impl NodeChecker for Percent {
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
impl NodeChecker for PercentEqual {
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
impl NodeChecker for Period {
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
impl NodeChecker for Plus {
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
impl NodeChecker for PlusEqual {
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
impl NodeChecker for PlusPlus {
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
impl NodeChecker for PragmaKeyword {
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
impl NodeChecker for PrivateKeyword {
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
impl NodeChecker for PromiseKeyword {
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
impl NodeChecker for PublicKeyword {
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
impl NodeChecker for PureKeyword {
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
impl NodeChecker for QuestionMark {
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
impl NodeChecker for ReceiveKeyword {
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
impl NodeChecker for ReferenceKeyword {
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
impl NodeChecker for RelocatableKeyword {
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
impl NodeChecker for ReturnKeyword {
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
impl NodeChecker for ReturnsKeyword {
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
impl NodeChecker for RevertKeyword {
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
impl NodeChecker for SMTCheckerKeyword {
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
impl NodeChecker for SealedKeyword {
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
impl NodeChecker for SecondsKeyword {
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
impl NodeChecker for Semicolon {
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
impl NodeChecker for SingleLineComment {
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
impl NodeChecker for SingleLineNatSpecComment {
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
impl NodeChecker for SingleQuotedHexStringLiteral {
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
impl NodeChecker for SingleQuotedStringLiteral {
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
impl NodeChecker for SingleQuotedUnicodeStringLiteral {
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
impl NodeChecker for SingleQuotedVersionLiteral {
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
impl NodeChecker for SizeOfKeyword {
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
impl NodeChecker for Slash {
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
impl NodeChecker for SlashEqual {
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
impl NodeChecker for SolidityKeyword {
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
impl NodeChecker for StaticKeyword {
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
impl NodeChecker for StorageKeyword {
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
impl NodeChecker for StringKeyword {
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
impl NodeChecker for StructKeyword {
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
impl NodeChecker for SuperKeyword {
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
impl NodeChecker for SupportsKeyword {
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
impl NodeChecker for SwitchKeyword {
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
impl NodeChecker for SzaboKeyword {
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
impl NodeChecker for ThisKeyword {
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
impl NodeChecker for ThrowKeyword {
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
impl NodeChecker for Tilde {
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
impl NodeChecker for TransientKeyword {
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
impl NodeChecker for TrueKeyword {
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
impl NodeChecker for TryKeyword {
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
impl NodeChecker for TypeDefKeyword {
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
impl NodeChecker for TypeKeyword {
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
impl NodeChecker for TypeOfKeyword {
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
impl NodeChecker for UfixedKeyword {
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
impl NodeChecker for UintKeyword {
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
impl NodeChecker for UncheckedKeyword {
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
impl NodeChecker for UsingKeyword {
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
impl NodeChecker for VarKeyword {
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
impl NodeChecker for VersionSpecifier {
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
impl NodeChecker for ViewKeyword {
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
impl NodeChecker for VirtualKeyword {
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
impl NodeChecker for WeeksKeyword {
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
impl NodeChecker for WeiKeyword {
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
impl NodeChecker for WhileKeyword {
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
impl NodeChecker for Whitespace {
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
impl NodeChecker for YearsKeyword {
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
impl NodeChecker for YulAbstractKeyword {
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
impl NodeChecker for YulAfterKeyword {
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
impl NodeChecker for YulAliasKeyword {
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
impl NodeChecker for YulAnonymousKeyword {
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
impl NodeChecker for YulApplyKeyword {
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
impl NodeChecker for YulAsKeyword {
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
impl NodeChecker for YulAssemblyKeyword {
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
impl NodeChecker for YulAutoKeyword {
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
impl NodeChecker for YulBoolKeyword {
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
impl NodeChecker for YulBreakKeyword {
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
impl NodeChecker for YulBytesKeyword {
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
impl NodeChecker for YulCallDataKeyword {
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
impl NodeChecker for YulCaseKeyword {
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
impl NodeChecker for YulCatchKeyword {
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
impl NodeChecker for YulConstantKeyword {
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
impl NodeChecker for YulConstructorKeyword {
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
impl NodeChecker for YulContinueKeyword {
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
impl NodeChecker for YulContractKeyword {
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
impl NodeChecker for YulCopyOfKeyword {
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
impl NodeChecker for YulDaysKeyword {
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
impl NodeChecker for YulDecimalLiteral {
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
impl NodeChecker for YulDefaultKeyword {
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
impl NodeChecker for YulDefineKeyword {
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
impl NodeChecker for YulDeleteKeyword {
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
impl NodeChecker for YulDoKeyword {
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
impl NodeChecker for YulElseKeyword {
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
impl NodeChecker for YulEmitKeyword {
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
impl NodeChecker for YulEnumKeyword {
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
impl NodeChecker for YulEtherKeyword {
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
impl NodeChecker for YulEventKeyword {
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
impl NodeChecker for YulExternalKeyword {
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
impl NodeChecker for YulFallbackKeyword {
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
impl NodeChecker for YulFalseKeyword {
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
impl NodeChecker for YulFinalKeyword {
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
impl NodeChecker for YulFinneyKeyword {
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
impl NodeChecker for YulFixedKeyword {
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
impl NodeChecker for YulForKeyword {
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
impl NodeChecker for YulFunctionKeyword {
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
impl NodeChecker for YulGweiKeyword {
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
impl NodeChecker for YulHexKeyword {
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
impl NodeChecker for YulHexLiteral {
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
impl NodeChecker for YulHoursKeyword {
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
impl NodeChecker for YulIdentifier {
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
impl NodeChecker for YulIfKeyword {
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
impl NodeChecker for YulImmutableKeyword {
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
impl NodeChecker for YulImplementsKeyword {
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
impl NodeChecker for YulImportKeyword {
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
impl NodeChecker for YulInKeyword {
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
impl NodeChecker for YulIndexedKeyword {
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
impl NodeChecker for YulInlineKeyword {
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
impl NodeChecker for YulIntKeyword {
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
impl NodeChecker for YulInterfaceKeyword {
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
impl NodeChecker for YulInternalKeyword {
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
impl NodeChecker for YulIsKeyword {
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
impl NodeChecker for YulLeaveKeyword {
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
impl NodeChecker for YulLetKeyword {
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
impl NodeChecker for YulLibraryKeyword {
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
impl NodeChecker for YulMacroKeyword {
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
impl NodeChecker for YulMappingKeyword {
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
impl NodeChecker for YulMatchKeyword {
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
impl NodeChecker for YulMemoryKeyword {
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
impl NodeChecker for YulMinutesKeyword {
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
impl NodeChecker for YulModifierKeyword {
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
impl NodeChecker for YulMutableKeyword {
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
impl NodeChecker for YulNewKeyword {
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
impl NodeChecker for YulNullKeyword {
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
impl NodeChecker for YulOfKeyword {
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
impl NodeChecker for YulOverrideKeyword {
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
impl NodeChecker for YulPartialKeyword {
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
impl NodeChecker for YulPayableKeyword {
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
impl NodeChecker for YulPragmaKeyword {
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
impl NodeChecker for YulPrivateKeyword {
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
impl NodeChecker for YulPromiseKeyword {
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
impl NodeChecker for YulPublicKeyword {
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
impl NodeChecker for YulPureKeyword {
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
impl NodeChecker for YulReceiveKeyword {
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
impl NodeChecker for YulReferenceKeyword {
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
impl NodeChecker for YulRelocatableKeyword {
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
impl NodeChecker for YulReturnsKeyword {
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
impl NodeChecker for YulSealedKeyword {
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
impl NodeChecker for YulSecondsKeyword {
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
impl NodeChecker for YulSizeOfKeyword {
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
impl NodeChecker for YulStaticKeyword {
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
impl NodeChecker for YulStorageKeyword {
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
impl NodeChecker for YulStringKeyword {
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
impl NodeChecker for YulStructKeyword {
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
impl NodeChecker for YulSuperKeyword {
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
impl NodeChecker for YulSupportsKeyword {
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
impl NodeChecker for YulSwitchKeyword {
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
impl NodeChecker for YulSzaboKeyword {
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
impl NodeChecker for YulThisKeyword {
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
impl NodeChecker for YulThrowKeyword {
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
impl NodeChecker for YulTrueKeyword {
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
impl NodeChecker for YulTryKeyword {
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
impl NodeChecker for YulTypeDefKeyword {
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
impl NodeChecker for YulTypeKeyword {
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
impl NodeChecker for YulTypeOfKeyword {
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
impl NodeChecker for YulUfixedKeyword {
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
impl NodeChecker for YulUintKeyword {
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
impl NodeChecker for YulUncheckedKeyword {
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
impl NodeChecker for YulUsingKeyword {
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
impl NodeChecker for YulVarKeyword {
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
impl NodeChecker for YulViewKeyword {
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
impl NodeChecker for YulVirtualKeyword {
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
impl NodeChecker for YulWeeksKeyword {
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
impl NodeChecker for YulWeiKeyword {
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
impl NodeChecker for YulWhileKeyword {
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
impl NodeChecker for YulYearsKeyword {
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
