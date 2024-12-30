// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_lines)] // large match statements for all non-terminals
#![allow(clippy::unnecessary_wraps)] // using `Result` for all functions for error handling

use std::rc::Rc;

use crate::rust_crate::cst::{Edge, EdgeLabel, Node, NonterminalKind, NonterminalNode};

//
// Sequences:
//

pub fn select_sequence(node: &Rc<NonterminalNode>) -> Result<Vec<Option<Node>>> {
    let mut helper = Helper::new(node);

    let result = match node.kind {
        NonterminalKind::SourceUnit => helper.source_unit_sequence()?,
        NonterminalKind::Tree => helper.tree_sequence()?,
        NonterminalKind::TreeNode => helper.tree_node_sequence()?,
        NonterminalKind::AdditionExpression => helper.addition_expression_sequence()?,
        NonterminalKind::NegationExpression => helper.negation_expression_sequence()?,
        NonterminalKind::MemberAccessExpression => helper.member_access_expression_sequence()?,
        _ => {
            // Should not theoretically happen, since we're only called from our own generated AST types.
            return Err(format!(
                "Unexpected parent node with NonterminalKind '{0}'.",
                node.kind
            ));
        }
    };

    helper.finalize()?;
    Ok(result)
}
impl Helper {
    fn source_unit_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![Some(self.select(EdgeLabel::Members)?)])
    }
}

impl Helper {
    fn tree_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Keyword)?),
            self.try_select(EdgeLabel::Name),
            Some(self.select(EdgeLabel::Node)?),
            Some(self.select(EdgeLabel::Semicolon)?),
        ])
    }
}

impl Helper {
    fn tree_node_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::OpenBracket)?),
            Some(self.select(EdgeLabel::Members)?),
            Some(self.select(EdgeLabel::CloseBracket)?),
        ])
    }
}

impl Helper {
    fn addition_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::LeftOperand)?),
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::RightOperand)?),
        ])
    }
}

impl Helper {
    fn negation_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Operator)?),
            Some(self.select(EdgeLabel::Operand)?),
        ])
    }
}

impl Helper {
    fn member_access_expression_sequence(&mut self) -> Result<Vec<Option<Node>>> {
        Ok(vec![
            Some(self.select(EdgeLabel::Operand)?),
            Some(self.select(EdgeLabel::Period)?),
            Some(self.select(EdgeLabel::Member)?),
        ])
    }
}
//
// Choices:
//

pub fn select_choice(node: &Rc<NonterminalNode>) -> Result<Node> {
    let mut helper = Helper::new(node);

    let variant = helper.select(EdgeLabel::Variant)?;

    helper.finalize()?;
    Ok(variant)
}

//
// Repeated:
//

pub fn select_repeated(node: &Rc<NonterminalNode>) -> Result<Vec<Node>> {
    let mut helper = Helper::new(node);

    let mut items = vec![];

    while let Some(item) = helper.try_select(EdgeLabel::Item) {
        items.push(item);
    }

    helper.finalize()?;
    Ok(items)
}

//
// Separated:
//

pub fn select_separated(node: &Rc<NonterminalNode>) -> Result<Vec<Vec<Node>>> {
    let mut helper = Helper::new(node);

    let mut items = vec![];
    let mut separators = vec![];

    if let Some(first) = helper.try_select(EdgeLabel::Item) {
        items.push(first);

        while let Some(separator) = helper.try_select(EdgeLabel::Separator) {
            separators.push(separator);

            items.push(helper.select(EdgeLabel::Item)?);
        }
    }

    helper.finalize()?;
    Ok(vec![items, separators])
}

//
// Common:
//

type Result<T> = std::result::Result<T, String>;

struct Helper {
    node: Rc<NonterminalNode>,
    index: usize,
}

impl Helper {
    fn new(node: &Rc<NonterminalNode>) -> Self {
        Self {
            node: Rc::clone(node),
            index: 0,
        }
    }

    fn select(&mut self, target_label: EdgeLabel) -> Result<Node> {
        match self.try_select(target_label) {
            Some(node) => {
                Ok(node)
            },
            None => {
                Err(format!("Missing child with label '{target_label}'. Creating AST types from incorrect/incomplete CST nodes is not supported yet."))
            }
        }
    }

    fn try_select(&mut self, target_label: EdgeLabel) -> Option<Node> {
        let (label, node) = self.current()?;

        if label == target_label {
            self.index += 1;
            Some(node.clone())
        } else {
            None
        }
    }

    fn current(&mut self) -> Option<(EdgeLabel, Node)> {
        loop {
            let Edge { label, node } = self.node.children.get(self.index)?;

            match label {
                // Skip root nodes:
                | EdgeLabel::Root
                // Skip trivia:
                | EdgeLabel::LeadingTrivia | EdgeLabel::TrailingTrivia => {
                    self.index += 1;
                    continue;
                }
                // Otherwise, return the edge:
                other_label => {
                    return Some((*other_label, node.clone()));
                }
            }
        }
    }

    fn finalize(mut self) -> Result<()> {
        match self.current() {
            Some((label, _)) => {
                Err(format!("Unrecognized child with label '{label}'. Creating AST types from incorrect/incomplete CST nodes is not supported yet."))
            }
            None => {
                Ok(())
            },
        }
    }
}
