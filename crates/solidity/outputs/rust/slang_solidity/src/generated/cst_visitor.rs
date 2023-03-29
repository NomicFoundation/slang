// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::{ops::Range, rc::Rc};

use super::cst::*;
use super::kinds::*;

#[allow(unused_variables)]
pub trait Visitor<E> {
    fn enter_rule(
        &mut self,
        kind: RuleKind,
        range: &Range<usize>,
        children: &Vec<Rc<Node>>,
        node: &Rc<Node>,
        path: &Vec<Rc<Node>>,
    ) -> Result<VisitorEntryResponse, E> {
        Ok(VisitorEntryResponse::StepIn)
    }

    fn exit_rule(
        &mut self,
        kind: RuleKind,
        range: &Range<usize>,
        children: &Vec<Rc<Node>>,
        node: &Rc<Node>,
        path: &Vec<Rc<Node>>,
    ) -> Result<VisitorExitResponse, E> {
        Ok(VisitorExitResponse::StepIn)
    }

    fn enter_token(
        &mut self,
        kind: TokenKind,
        range: &Range<usize>,
        trivia: &Vec<Rc<Node>>,
        node: &Rc<Node>,
        path: &Vec<Rc<Node>>,
    ) -> Result<VisitorEntryResponse, E> {
        Ok(VisitorEntryResponse::StepIn)
    }

    fn exit_token(
        &mut self,
        kind: TokenKind,
        range: &Range<usize>,
        trivia: &Vec<Rc<Node>>,
        node: &Rc<Node>,
        path: &Vec<Rc<Node>>,
    ) -> Result<VisitorExitResponse, E> {
        Ok(VisitorExitResponse::StepIn)
    }
}

pub enum VisitorEntryResponse {
    Quit,
    StepIn,
    StepOver,
}

pub enum VisitorExitResponse {
    Quit,
    StepIn,
}

pub trait Visitable<T: Visitor<E>, E> {
    fn visit(&self, visitor: &mut T) -> Result<VisitorExitResponse, E>;
    fn visit_with_path(
        &self,
        visitor: &mut T,
        path: &mut Vec<Rc<Node>>,
    ) -> Result<VisitorExitResponse, E>;
}

impl<T: Visitor<E>, E> Visitable<T, E> for Rc<Node> {
    fn visit(&self, visitor: &mut T) -> Result<VisitorExitResponse, E> {
        self.visit_with_path(visitor, &mut Vec::new())
    }

    fn visit_with_path(
        &self,
        visitor: &mut T,
        path: &mut Vec<Rc<Node>>,
    ) -> Result<VisitorExitResponse, E> {
        match self.as_ref() {
            Node::Rule {
                kind,
                range,
                children,
            } => {
                match visitor.enter_rule(*kind, range, children, self, path)? {
                    VisitorEntryResponse::Quit => return Ok(VisitorExitResponse::Quit),
                    VisitorEntryResponse::StepOver => {}
                    VisitorEntryResponse::StepIn => {
                        path.push(self.clone());
                        for child in children {
                            match child.visit_with_path(visitor, path)? {
                                VisitorExitResponse::Quit => {
                                    path.pop();
                                    return Ok(VisitorExitResponse::Quit);
                                }
                                VisitorExitResponse::StepIn => {}
                            }
                        }
                        path.pop();
                    }
                }
                visitor.exit_rule(*kind, range, children, self, path)
            }
            Node::Token {
                kind,
                range,
                trivia,
            } => {
                match visitor.enter_token(*kind, range, trivia, self, path)? {
                    VisitorEntryResponse::Quit => return Ok(VisitorExitResponse::Quit),
                    VisitorEntryResponse::StepOver => {}
                    VisitorEntryResponse::StepIn => {
                        path.push(self.clone());
                        for child in trivia {
                            match child.visit_with_path(visitor, path)? {
                                VisitorExitResponse::Quit => {
                                    path.pop();
                                    return Ok(VisitorExitResponse::Quit);
                                }
                                VisitorExitResponse::StepIn => {}
                            }
                        }
                        path.pop();
                    }
                }
                visitor.exit_token(*kind, range, trivia, self, path)
            }
        }
    }
}
