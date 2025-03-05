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
    let mut fields_cursor = cursor.clone();
    if !fields_cursor.go_to_first_child() {
        return Err("Expected sequence node to have at least one children".into());
    }
    skip_trivia(&mut fields_cursor)?;
    expect_label(&fields_cursor, EdgeLabel::Members)?;
    let members = build_source_unit_members(fields_cursor.clone())?;

    consume_remaining_trivia(fields_cursor)?;

    Ok(Rc::new(SourceUnitStruct { cursor, members }))
}

pub fn build_tree(cursor: Cursor) -> Result<Tree> {
    expect_nonterminal_kind(&cursor, NonterminalKind::Tree)?;
    let mut fields_cursor = cursor.clone();
    if !fields_cursor.go_to_first_child() {
        return Err("Expected sequence node to have at least one children".into());
    }
    skip_trivia(&mut fields_cursor)?;
    expect_label(&fields_cursor, EdgeLabel::Keyword)?;
    let keyword = fetch_terminal_node(&fields_cursor)?;
    next_non_trivia_sibling(&mut fields_cursor)?;
    let name = if fields_cursor.label() == EdgeLabel::Name {
        Some(fetch_terminal_node(&fields_cursor)?)
    } else {
        None
    };
    next_non_trivia_sibling(&mut fields_cursor)?;
    expect_label(&fields_cursor, EdgeLabel::Node)?;
    let node = build_tree_node(fields_cursor.clone())?;
    next_non_trivia_sibling(&mut fields_cursor)?;
    expect_label(&fields_cursor, EdgeLabel::Semicolon)?;
    let semicolon = fetch_terminal_node(&fields_cursor)?;

    consume_remaining_trivia(fields_cursor)?;

    Ok(Rc::new(TreeStruct {
        cursor,
        keyword,
        name,
        node,
        semicolon,
    }))
}

pub fn build_tree_node(cursor: Cursor) -> Result<TreeNode> {
    expect_nonterminal_kind(&cursor, NonterminalKind::TreeNode)?;
    let mut fields_cursor = cursor.clone();
    if !fields_cursor.go_to_first_child() {
        return Err("Expected sequence node to have at least one children".into());
    }
    skip_trivia(&mut fields_cursor)?;
    expect_label(&fields_cursor, EdgeLabel::OpenBracket)?;
    let open_bracket = fetch_terminal_node(&fields_cursor)?;
    next_non_trivia_sibling(&mut fields_cursor)?;
    expect_label(&fields_cursor, EdgeLabel::Members)?;
    let members = build_tree_node_children(fields_cursor.clone())?;
    next_non_trivia_sibling(&mut fields_cursor)?;
    expect_label(&fields_cursor, EdgeLabel::CloseBracket)?;
    let close_bracket = fetch_terminal_node(&fields_cursor)?;

    consume_remaining_trivia(fields_cursor)?;

    Ok(Rc::new(TreeNodeStruct {
        cursor,
        open_bracket,
        members,
        close_bracket,
    }))
}

pub fn build_addition_expression(cursor: Cursor) -> Result<AdditionExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::AdditionExpression)?;
    let mut fields_cursor = cursor.clone();
    if !fields_cursor.go_to_first_child() {
        return Err("Expected sequence node to have at least one children".into());
    }
    skip_trivia(&mut fields_cursor)?;
    expect_label(&fields_cursor, EdgeLabel::LeftOperand)?;
    let left_operand = build_expression(fields_cursor.clone())?;
    next_non_trivia_sibling(&mut fields_cursor)?;
    expect_label(&fields_cursor, EdgeLabel::Operator)?;
    let operator = fetch_terminal_node(&fields_cursor)?;
    next_non_trivia_sibling(&mut fields_cursor)?;
    expect_label(&fields_cursor, EdgeLabel::RightOperand)?;
    let right_operand = build_expression(fields_cursor.clone())?;

    consume_remaining_trivia(fields_cursor)?;

    Ok(Rc::new(AdditionExpressionStruct {
        cursor,
        left_operand,
        operator,
        right_operand,
    }))
}

pub fn build_negation_expression(cursor: Cursor) -> Result<NegationExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::NegationExpression)?;
    let mut fields_cursor = cursor.clone();
    if !fields_cursor.go_to_first_child() {
        return Err("Expected sequence node to have at least one children".into());
    }
    skip_trivia(&mut fields_cursor)?;
    expect_label(&fields_cursor, EdgeLabel::Operator)?;
    let operator = fetch_terminal_node(&fields_cursor)?;
    next_non_trivia_sibling(&mut fields_cursor)?;
    expect_label(&fields_cursor, EdgeLabel::Operand)?;
    let operand = build_expression(fields_cursor.clone())?;

    consume_remaining_trivia(fields_cursor)?;

    Ok(Rc::new(NegationExpressionStruct {
        cursor,
        operator,
        operand,
    }))
}

pub fn build_member_access_expression(cursor: Cursor) -> Result<MemberAccessExpression> {
    expect_nonterminal_kind(&cursor, NonterminalKind::MemberAccessExpression)?;
    let mut fields_cursor = cursor.clone();
    if !fields_cursor.go_to_first_child() {
        return Err("Expected sequence node to have at least one children".into());
    }
    skip_trivia(&mut fields_cursor)?;
    expect_label(&fields_cursor, EdgeLabel::Operand)?;
    let operand = build_expression(fields_cursor.clone())?;
    next_non_trivia_sibling(&mut fields_cursor)?;
    expect_label(&fields_cursor, EdgeLabel::Period)?;
    let period = fetch_terminal_node(&fields_cursor)?;
    next_non_trivia_sibling(&mut fields_cursor)?;
    expect_label(&fields_cursor, EdgeLabel::Member)?;
    let member = fetch_terminal_node(&fields_cursor)?;

    consume_remaining_trivia(fields_cursor)?;

    Ok(Rc::new(MemberAccessExpressionStruct {
        cursor,
        operand,
        period,
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

pub fn build_literal(mut cursor: Cursor) -> Result<Rc<TerminalNode>> {
    expect_nonterminal_kind(&cursor, NonterminalKind::Literal)?;
    if !cursor.go_to_first_child() {
        return Err("Expected choice node to have at least one children".into());
    }
    skip_trivia(&mut cursor)?;
    expect_label(&cursor, EdgeLabel::Variant)?;
    let item = fetch_terminal_node(&cursor)?;
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
            "Expected label {:?}, but got {:?} instead",
            label,
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
            "Expected {:?} node, but got {:?} instead",
            kind,
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

#[allow(dead_code)]
#[inline]
fn next_non_trivia_sibling(cursor: &mut Cursor) -> Result<()> {
    while cursor.go_to_next_sibling() {
        if !cursor.node().is_trivia() {
            return Ok(());
        }
    }
    Err("Expected more non-trivia siblings".into())
}
