// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::rc::Rc;

#[allow(clippy::wildcard_imports)]
use super::nodes::*;
use crate::cst::{Cursor, EdgeLabel, NodeKind, NonterminalKind, TerminalNode};

//
// Sequences:
//

pub fn build_source_unit(cursor: Cursor) -> Result<SourceUnit> {
    expect_nonterminal_kind(&cursor, NonterminalKind::SourceUnit)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let members = build_source_unit_members(helper.accept_label(EdgeLabel::Members)?)?;
    helper.finalize()?;

    Ok(Rc::new(SourceUnitStruct { cursor, members }))
}

pub fn build_tree(cursor: Cursor) -> Result<Tree> {
    expect_nonterminal_kind(&cursor, NonterminalKind::Tree)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Keyword)?)?;
    let name = if helper.at_label(EdgeLabel::Name) {
        Some(fetch_terminal_node(&helper.accept_label(EdgeLabel::Name)?)?)
    } else {
        None
    };
    let node = build_tree_node(helper.accept_label(EdgeLabel::Node)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Semicolon)?)?;
    helper.finalize()?;

    Ok(Rc::new(TreeStruct { cursor, name, node }))
}

pub fn build_tree_node(cursor: Cursor) -> Result<TreeNode> {
    expect_nonterminal_kind(&cursor, NonterminalKind::TreeNode)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::OpenBracket)?)?;
    let members = build_tree_node_children(helper.accept_label(EdgeLabel::Members)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::CloseBracket)?)?;
    helper.finalize()?;

    Ok(Rc::new(TreeNodeStruct { cursor, members }))
}

pub fn build_addition_expression(cursor: Cursor) -> Result<AdditionExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::AdditionExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let left_operand = build_expression(helper.accept_label(EdgeLabel::LeftOperand)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let right_operand = build_expression(helper.accept_label(EdgeLabel::RightOperand)?)?;
    helper.finalize()?;

    Ok(Rc::new(AdditionExpressionStruct {
        cursor,
        left_operand,
        right_operand,
    }))
}

pub fn build_negation_expression(cursor: Cursor) -> Result<NegationExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::NegationExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Operator)?)?;
    let operand = build_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    helper.finalize()?;

    Ok(Rc::new(NegationExpressionStruct { cursor, operand }))
}

pub fn build_member_access_expression(cursor: Cursor) -> Result<MemberAccessExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::MemberAccessExpression)?;
    let mut helper = SequenceHelper::new(cursor.clone());
    let operand = build_expression(helper.accept_label(EdgeLabel::Operand)?)?;
    _ = fetch_terminal_node(&helper.accept_label(EdgeLabel::Period)?)?;
    let member = fetch_terminal_node(&helper.accept_label(EdgeLabel::Member)?)?;
    helper.finalize()?;

    Ok(Rc::new(MemberAccessExpressionStruct {
        cursor,
        operand,
        member,
    }))
}

//
// Choices:
//

pub fn build_source_unit_member(mut cursor: Cursor) -> Result<SourceUnitMember> {
    expect_nonterminal_kind(&cursor, NonterminalKind::SourceUnitMember)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::Tree) => {
            SourceUnitMember::Tree(build_tree(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::Expression) => {
            SourceUnitMember::Expression(build_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::SeparatedIdentifiers) => {
            SourceUnitMember::SeparatedIdentifiers(build_separated_identifiers(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::Literal) => {
            SourceUnitMember::Literal(build_literal(cursor.clone())?)
        }
        NodeKind::Nonterminal(_) | NodeKind::Terminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_tree_node_child(mut cursor: Cursor) -> Result<TreeNodeChild> {
    expect_nonterminal_kind(&cursor, NonterminalKind::TreeNodeChild)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::TreeNode) => {
            TreeNodeChild::TreeNode(build_tree_node(cursor.clone())?)
        }
        NodeKind::Terminal(_) => TreeNodeChild::TerminalNode(fetch_terminal_node(&cursor)?),
        NodeKind::Nonterminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_expression(mut cursor: Cursor) -> Result<Expression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::Expression)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = match cursor.node().kind() {
        NodeKind::Nonterminal(NonterminalKind::AdditionExpression) => {
            Expression::AdditionExpression(build_addition_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::NegationExpression) => {
            Expression::NegationExpression(build_negation_expression(cursor.clone())?)
        }
        NodeKind::Nonterminal(NonterminalKind::MemberAccessExpression) => {
            Expression::MemberAccessExpression(build_member_access_expression(cursor.clone())?)
        }
        NodeKind::Terminal(_) => Expression::TerminalNode(fetch_terminal_node(&cursor)?),
        NodeKind::Nonterminal(_) => {
            return Err(format!(
                "Unexpected variant node of kind {:?}",
                cursor.node().kind()
            ));
        }
    };
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

pub fn build_literal(mut cursor: Cursor) -> Result<Literal> {
    expect_nonterminal_kind(&cursor, NonterminalKind::Literal)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = Literal(fetch_terminal_node(&cursor)?);
    consume_remaining_trivia(cursor)?;
    Ok(item)
}

//
// Repeated:
//

pub fn build_source_unit_members(mut cursor: Cursor) -> Result<SourceUnitMembers> {
    expect_nonterminal_kind(&cursor, NonterminalKind::SourceUnitMembers)?;
    let mut items = SourceUnitMembers::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_source_unit_member(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

pub fn build_tree_node_children(mut cursor: Cursor) -> Result<TreeNodeChildren> {
    expect_nonterminal_kind(&cursor, NonterminalKind::TreeNodeChildren)?;
    let mut items = TreeNodeChildren::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = build_tree_node_child(cursor.clone())?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

//
// Separated:
//

pub fn build_separated_identifiers(mut cursor: Cursor) -> Result<SeparatedIdentifiers> {
    expect_nonterminal_kind(&cursor, NonterminalKind::SeparatedIdentifiers)?;
    let mut items = SeparatedIdentifiers::new();
    if !cursor.go_to_first_child() {
        return Ok(items);
    }
    loop {
        if !cursor.node().is_trivia() && cursor.label() != EdgeLabel::Separator {
            expect_label(&cursor, EdgeLabel::Item)?;
            let item = fetch_terminal_node(&cursor)?;
            items.push(item);
        }
        if !cursor.go_to_next_sibling() {
            break;
        }
    }
    Ok(items)
}

//
// Common:
//

type Result<T> = std::result::Result<T, String>;

#[allow(dead_code)]
#[inline]
fn fetch_terminal_node(cursor: &Cursor) -> Result<Rc<TerminalNode>> {
    cursor.node().into_terminal().ok_or(format!(
        "Expected terminal node, got {:?} instead",
        cursor.node().kind()
    ))
}

#[allow(dead_code)]
#[inline]
fn expect_label(cursor: &Cursor, label: EdgeLabel) -> Result<()> {
    if cursor.label() == label {
        Ok(())
    } else {
        Err(format!(
            "Expected label {label:?}, but got {:?} instead",
            cursor.label()
        ))
    }
}

#[allow(dead_code)]
#[inline]
fn expect_nonterminal_kind(cursor: &Cursor, kind: NonterminalKind) -> Result<()> {
    if cursor.node().is_nonterminal_with_kind(kind) {
        Ok(())
    } else {
        Err(format!(
            "Expected {kind:?} node, but got {:?} instead",
            cursor.node().kind()
        ))
    }
}

#[allow(dead_code)]
#[inline]
fn skip_trivia(cursor: &mut Cursor) -> Result<()> {
    while cursor.node().is_trivia() {
        if !cursor.go_to_next_sibling() {
            return Err("Expected choice node to have at least non trivia child".into());
        }
    }
    Ok(())
}

#[allow(dead_code)]
#[inline]
fn consume_remaining_trivia(mut cursor: Cursor) -> Result<()> {
    while cursor.go_to_next_sibling() {
        if !cursor.node().is_trivia() {
            return Err("Unexpected non-trivia node".into());
        }
    }
    Ok(())
}

struct SequenceHelper {
    cursor: Cursor,
    finished: bool,
}

impl SequenceHelper {
    fn new(mut cursor: Cursor) -> Self {
        let mut finished = false;
        if cursor.go_to_first_child() {
            // skip initial trivia
            while cursor.node().is_trivia() {
                if !cursor.go_to_next_sibling() {
                    finished = true;
                    break;
                }
            }
        }
        Self { cursor, finished }
    }

    fn at_label(&self, label: EdgeLabel) -> bool {
        !self.finished && self.cursor.label() == label
    }

    fn accept_label(&mut self, label: EdgeLabel) -> Result<Cursor> {
        if self.finished {
            return Err(format!(
                "Expected more sibling nodes, looking for label {label:?}"
            ));
        }
        if self.cursor.label() == label {
            let cursor = self.cursor.clone();
            loop {
                if !self.cursor.go_to_next_sibling() {
                    self.finished = true;
                    break;
                }
                if !self.cursor.node().is_trivia() {
                    break;
                }
            }
            Ok(cursor)
        } else {
            Err(format!(
                "Expected node with label {label:?}, got {:?}",
                self.cursor.label()
            ))
        }
    }

    fn finalize(self) -> Result<()> {
        consume_remaining_trivia(self.cursor)
    }
}
