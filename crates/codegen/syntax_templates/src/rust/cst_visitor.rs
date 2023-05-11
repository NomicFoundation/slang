use std::rc::Rc;

use super::cst::*;
use super::kinds::*;
use super::language::TextRange;

#[allow(unused_variables)]
pub trait Visitor<E> {
    fn enter_rule(
        &mut self,
        kind: RuleKind,
        range: &TextRange,
        children: &Vec<Rc<Node>>,
        node: &Rc<Node>,
        path: &Vec<Rc<Node>>,
    ) -> Result<VisitorEntryResponse, E> {
        Ok(VisitorEntryResponse::StepIn)
    }

    fn exit_rule(
        &mut self,
        kind: RuleKind,
        range: &TextRange,
        children: &Vec<Rc<Node>>,
        node: &Rc<Node>,
        path: &Vec<Rc<Node>>,
    ) -> Result<VisitorExitResponse, E> {
        Ok(VisitorExitResponse::Continue)
    }

    fn enter_token(
        &mut self,
        kind: TokenKind,
        range: &TextRange,
        trivia: &Vec<Rc<Node>>,
        node: &Rc<Node>,
        path: &Vec<Rc<Node>>,
    ) -> Result<VisitorEntryResponse, E> {
        Ok(VisitorEntryResponse::StepIn)
    }

    fn exit_token(
        &mut self,
        kind: TokenKind,
        range: &TextRange,
        trivia: &Vec<Rc<Node>>,
        node: &Rc<Node>,
        path: &Vec<Rc<Node>>,
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

impl<T: Visitor<E>, E> Visitable<T, E> for Rc<Node> {
    fn accept_visitor(&self, visitor: &mut T) -> Result<VisitorExitResponse, E> {
        accept_visitor_with_path(self, visitor, &mut Vec::new())
    }
}

fn accept_visitor_with_path<T: Visitor<E>, E>(
    node: &Rc<Node>,
    visitor: &mut T,
    path: &mut Vec<Rc<Node>>,
) -> Result<VisitorExitResponse, E> {
    match node.as_ref() {
        Node::Rule {
            kind,
            range,
            children,
        } => {
            match visitor.enter_rule(*kind, range, children, node, path)? {
                VisitorEntryResponse::Quit => return Ok(VisitorExitResponse::Quit),
                VisitorEntryResponse::StepOver => {}
                VisitorEntryResponse::StepIn => {
                    path.push(node.clone());
                    for child in children {
                        match accept_visitor_with_path(child, visitor, path)? {
                            VisitorExitResponse::Quit => {
                                path.pop();
                                return Ok(VisitorExitResponse::Quit);
                            }
                            VisitorExitResponse::Continue => {
                                continue;
                            }
                        };
                    }
                    path.pop();
                }
            }
            visitor.exit_rule(*kind, range, children, node, path)
        }
        Node::Token {
            kind,
            range,
            trivia,
        } => {
            match visitor.enter_token(*kind, range, trivia, node, path)? {
                VisitorEntryResponse::Quit => return Ok(VisitorExitResponse::Quit),
                VisitorEntryResponse::StepOver => {}
                VisitorEntryResponse::StepIn => {
                    path.push(node.clone());
                    for child in trivia {
                        match accept_visitor_with_path(child, visitor, path)? {
                            VisitorExitResponse::Quit => {
                                path.pop();
                                return Ok(VisitorExitResponse::Quit);
                            }
                            VisitorExitResponse::Continue => {
                                continue;
                            }
                        };
                    }
                    path.pop();
                }
            }
            visitor.exit_token(*kind, range, trivia, node, path)
        }
    }
}
