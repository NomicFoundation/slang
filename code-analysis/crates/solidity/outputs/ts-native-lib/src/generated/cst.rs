// This file is generated via `cargo build`. Please don't edit by hand.

use super::kinds;
use serde::Serialize;
use std::ops::Range;
use std::rc::Rc;
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Node {
    None,
    Rule {
        kind: kinds::Rule,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        children: Vec<Rc<Node>>,
    },
    Token {
        kind: kinds::Token,
        #[doc = r" Range doesn't include the trivia"]
        range: Range<usize>,
        #[doc = r" Only Trivia"]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        trivia: Vec<Rc<Node>>,
    },
    #[doc = r" For anonymous groups referenced from AST nodes i.e. `delimited_by`"]
    Group {
        #[serde(skip_serializing_if = "Vec::is_empty")]
        children: Vec<Rc<Node>>,
    },
}
#[allow(unused_variables)]
pub trait Visitor {
    fn visit_none(&mut self, node: &Rc<Node>) -> VisitorExitResponse {
        VisitorExitResponse::StepIn
    }
    fn enter_rule(
        &mut self,
        kind: kinds::Rule,
        children: &Vec<Rc<Node>>,
        node: &Rc<Node>,
    ) -> VisitorEntryResponse {
        VisitorEntryResponse::StepIn
    }
    fn exit_rule(
        &mut self,
        kind: kinds::Rule,
        children: &Vec<Rc<Node>>,
        node: &Rc<Node>,
    ) -> VisitorExitResponse {
        VisitorExitResponse::StepIn
    }
    fn enter_token(
        &mut self,
        kind: kinds::Token,
        range: &Range<usize>,
        trivia: &Vec<Rc<Node>>,
        node: &Rc<Node>,
    ) -> VisitorEntryResponse {
        VisitorEntryResponse::StepIn
    }
    fn exit_token(
        &mut self,
        kind: kinds::Token,
        range: &Range<usize>,
        trivia: &Vec<Rc<Node>>,
        node: &Rc<Node>,
    ) -> VisitorExitResponse {
        VisitorExitResponse::StepIn
    }
    fn enter_group(&mut self, children: &Vec<Rc<Node>>, node: &Rc<Node>) -> VisitorEntryResponse {
        VisitorEntryResponse::StepIn
    }
    fn exit_group(&mut self, children: &Vec<Rc<Node>>, node: &Rc<Node>) -> VisitorExitResponse {
        VisitorExitResponse::StepIn
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
pub trait Visitable {
    fn visit<T: Visitor>(&self, visitor: &mut T) -> VisitorExitResponse;
}
impl Visitable for Rc<Node> {
    fn visit<T: Visitor>(&self, visitor: &mut T) -> VisitorExitResponse {
        match self.as_ref() {
            Node::None => visitor.visit_none(self),
            Node::Rule { kind, children } => {
                match visitor.enter_rule(*kind, children, self) {
                    VisitorEntryResponse::Quit => return VisitorExitResponse::Quit,
                    VisitorEntryResponse::StepOver => {}
                    VisitorEntryResponse::StepIn => {
                        for child in children {
                            match child.visit(visitor) {
                                VisitorExitResponse::Quit => return VisitorExitResponse::Quit,
                                VisitorExitResponse::StepIn => {}
                            }
                        }
                    }
                }
                visitor.exit_rule(*kind, children, self)
            }
            Node::Token {
                kind,
                range,
                trivia,
            } => {
                match visitor.enter_token(*kind, range, trivia, self) {
                    VisitorEntryResponse::Quit => return VisitorExitResponse::Quit,
                    VisitorEntryResponse::StepOver => {}
                    VisitorEntryResponse::StepIn => {
                        for child in trivia {
                            match child.visit(visitor) {
                                VisitorExitResponse::Quit => return VisitorExitResponse::Quit,
                                VisitorExitResponse::StepIn => {}
                            }
                        }
                    }
                }
                visitor.exit_token(*kind, range, trivia, self)
            }
            Node::Group { children } => {
                match visitor.enter_group(children, self) {
                    VisitorEntryResponse::Quit => return VisitorExitResponse::Quit,
                    VisitorEntryResponse::StepOver => {}
                    VisitorEntryResponse::StepIn => {
                        for child in children {
                            match child.visit(visitor) {
                                VisitorExitResponse::Quit => return VisitorExitResponse::Quit,
                                VisitorExitResponse::StepIn => {}
                            }
                        }
                    }
                }
                visitor.exit_group(children, self)
            }
        }
    }
}
