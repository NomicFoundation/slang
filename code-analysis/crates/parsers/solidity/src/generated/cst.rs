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
        children: Vec<NodeRef>,
    },
    Token {
        kind: kinds::Token,
        #[doc = r" Range doesn't include the trivia"]
        range: Range<usize>,
        #[doc = r" Only Trivia"]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        trivia: Vec<NodeRef>,
    },
    #[doc = r" For anonymous groups referenced from AST nodes i.e. `delimited_by`"]
    Group {
        #[serde(skip_serializing_if = "Vec::is_empty")]
        children: Vec<NodeRef>,
    },
}
pub type NodeRef = Rc<Node>;
#[allow(unused_variables)]
pub trait Visitor {
    fn enter_rule(
        &mut self,
        kind: &kinds::Rule,
        children: &Vec<NodeRef>,
        node: &NodeRef,
    ) -> VisitorEntryResponse {
        VisitorEntryResponse::Continue
    }
    fn exit_rule(
        &mut self,
        kind: &kinds::Rule,
        children: &Vec<NodeRef>,
        node: &NodeRef,
    ) -> VisitorExitResponse {
        VisitorExitResponse::Continue
    }
    fn enter_token(
        &mut self,
        kind: &kinds::Token,
        range: &Range<usize>,
        trivia: &Vec<NodeRef>,
        node: &NodeRef,
    ) -> VisitorEntryResponse {
        VisitorEntryResponse::Continue
    }
    fn exit_token(
        &mut self,
        kind: &kinds::Token,
        range: &Range<usize>,
        trivia: &Vec<NodeRef>,
        node: &NodeRef,
    ) -> VisitorExitResponse {
        VisitorExitResponse::Continue
    }
    fn enter_group(&mut self, children: &Vec<NodeRef>, node: &NodeRef) -> VisitorEntryResponse {
        VisitorEntryResponse::Continue
    }
    fn exit_group(&mut self, children: &Vec<NodeRef>, node: &NodeRef) -> VisitorExitResponse {
        VisitorExitResponse::Continue
    }
    fn visit_none(&mut self, node: &NodeRef) -> VisitorExitResponse {
        VisitorExitResponse::Continue
    }
}
pub enum VisitorEntryResponse {
    Quit,
    Continue,
    ContinueButIgnoreChildren,
}
pub enum VisitorExitResponse {
    Quit,
    Continue,
}
pub trait Visitable {
    fn visit<T: Visitor>(&self, visitor: &mut T) -> VisitorExitResponse;
}
impl Node {
    #[inline]
    pub fn none() -> NodeRef {
        Rc::new(Self::None)
    }
    #[inline]
    pub fn rule(kind: kinds::Rule, children: Vec<NodeRef>) -> NodeRef {
        Rc::new(Self::Rule { kind, children })
    }
    #[inline]
    pub fn trivia_token(range: Range<usize>, kind: kinds::Token) -> NodeRef {
        Rc::new(Self::Token {
            range,
            kind,
            trivia: vec![],
        })
    }
    #[inline]
    pub fn token(
        range: Range<usize>,
        kind: kinds::Token,
        leading_trivia: NodeRef,
        trailing_trivia: NodeRef,
    ) -> NodeRef {
        let mut trivia = vec![];
        if *leading_trivia != Node::None {
            trivia.push(leading_trivia)
        }
        if *trailing_trivia != Node::None {
            trivia.push(trailing_trivia)
        }
        Rc::new(Self::Token {
            range,
            kind,
            trivia,
        })
    }
    #[inline]
    pub fn group(children: Vec<NodeRef>) -> NodeRef {
        Rc::new(Self::Group { children })
    }
}
impl Visitable for NodeRef {
    fn visit<T: Visitor>(&self, visitor: &mut T) -> VisitorExitResponse {
        match self.as_ref() {
            Node::None => visitor.visit_none(&Rc::new(Node::None)),
            Node::Rule { kind, children } => {
                match visitor.enter_rule(kind, children, self) {
                    VisitorEntryResponse::Quit => return VisitorExitResponse::Quit,
                    VisitorEntryResponse::ContinueButIgnoreChildren => {}
                    VisitorEntryResponse::Continue => {
                        for child in children {
                            match child.visit(visitor) {
                                VisitorExitResponse::Quit => return VisitorExitResponse::Quit,
                                VisitorExitResponse::Continue => {}
                            }
                        }
                    }
                }
                visitor.exit_rule(kind, children, self)
            }
            Node::Token {
                kind,
                range,
                trivia,
            } => {
                match visitor.enter_token(kind, range, trivia, self) {
                    VisitorEntryResponse::Quit => return VisitorExitResponse::Quit,
                    VisitorEntryResponse::ContinueButIgnoreChildren => {}
                    VisitorEntryResponse::Continue => {
                        for child in trivia {
                            match child.visit(visitor) {
                                VisitorExitResponse::Quit => return VisitorExitResponse::Quit,
                                VisitorExitResponse::Continue => {}
                            }
                        }
                    }
                }
                visitor.exit_token(kind, range, trivia, self)
            }
            Node::Group { children } => {
                match visitor.enter_group(children, self) {
                    VisitorEntryResponse::Quit => return VisitorExitResponse::Quit,
                    VisitorEntryResponse::ContinueButIgnoreChildren => {}
                    VisitorEntryResponse::Continue => {
                        for child in children {
                            match child.visit(visitor) {
                                VisitorExitResponse::Quit => return VisitorExitResponse::Quit,
                                VisitorExitResponse::Continue => {}
                            }
                        }
                    }
                }
                visitor.exit_group(children, self)
            }
        }
    }
}
