use std::{ops::Range, rc::Rc};

use super::{
    cst::*,
    text_index::{TextIndex, TextRange},
};

#[allow(unused_variables)]
pub trait Visitor<E> {
    fn enter_rule(
        &mut self,
        node: &Rc<RuleNode>,
        path: &Vec<Rc<RuleNode>>,
        range: &TextRange,
    ) -> Result<VisitorEntryResponse, E> {
        Ok(VisitorEntryResponse::StepIn)
    }

    fn exit_rule(
        &mut self,
        node: &Rc<RuleNode>,
        path: &Vec<Rc<RuleNode>>,
        range: &TextRange,
    ) -> Result<VisitorExitResponse, E> {
        Ok(VisitorExitResponse::Continue)
    }

    fn enter_token(
        &mut self,
        node: &Rc<TokenNode>,
        path: &Vec<Rc<RuleNode>>,
        range: &TextRange,
    ) -> Result<VisitorEntryResponse, E> {
        Ok(VisitorEntryResponse::StepIn)
    }

    fn exit_token(
        &mut self,
        node: &Rc<TokenNode>,
        path: &Vec<Rc<RuleNode>>,
        range: &TextRange,
    ) -> Result<VisitorExitResponse, E> {
        Ok(VisitorExitResponse::Continue)
    }
}

pub enum VisitorEntryResponse {
    Quit,
    StepIn,
    StepOver,
}

pub enum VisitorExitResponse {
    Quit,
    Continue,
}

pub trait Visitable<T: Visitor<E>, E> {
    fn accept_visitor(&self, visitor: &mut T) -> Result<VisitorExitResponse, E>;
}

impl<T: Visitor<E>, E> Visitable<T, E> for Node {
    fn accept_visitor(&self, visitor: &mut T) -> Result<VisitorExitResponse, E> {
        accept_visitor_with_path(self, visitor, &mut Vec::new(), Default::default())
            .map(|(response, _)| response)
    }
}

fn accept_visitor_with_path<T: Visitor<E>, E>(
    node: &Node,
    visitor: &mut T,
    path: &mut Vec<Rc<RuleNode>>,
    source_offset: TextIndex,
) -> Result<(VisitorExitResponse, TextIndex), E> {
    match node {
        Node::Rule(rule_node) => {
            let range = Range {
                start: source_offset,
                end: source_offset + rule_node.text_len,
            };
            match visitor.enter_rule(rule_node, path, &range)? {
                VisitorEntryResponse::Quit => {
                    return Ok((VisitorExitResponse::Quit, source_offset))
                }
                VisitorEntryResponse::StepOver => {}
                VisitorEntryResponse::StepIn => {
                    path.push(rule_node.clone());
                    let mut child_source_offset = source_offset;
                    for child in &rule_node.children {
                        match accept_visitor_with_path(&child, visitor, path, child_source_offset)?
                        {
                            (VisitorExitResponse::Quit, new_source_offset) => {
                                path.pop();
                                return Ok((VisitorExitResponse::Quit, new_source_offset));
                            }
                            (VisitorExitResponse::Continue, new_source_offset) => {
                                child_source_offset = new_source_offset;
                                continue;
                            }
                        };
                    }
                    path.pop();
                }
            }
            visitor
                .exit_rule(rule_node, path, &range)
                .map(|response| (response, range.end))
        }
        Node::Token(token_node) => {
            let range = Range {
                start: source_offset,
                end: source_offset + (&token_node.text).into(),
            };
            match visitor.enter_token(token_node, path, &range)? {
                VisitorEntryResponse::Quit => {
                    return Ok((VisitorExitResponse::Quit, source_offset))
                }
                VisitorEntryResponse::StepOver | VisitorEntryResponse::StepIn => {}
            }
            visitor
                .exit_token(token_node, path, &range)
                .map(|response| (response, range.end))
        }
    }
}
