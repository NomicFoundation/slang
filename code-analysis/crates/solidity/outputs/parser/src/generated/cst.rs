// This file is generated via `cargo build`. Please don't edit by hand.

use super::kinds;
use super::parse::Context;
use serde::Serialize;
use std::ops::Range;
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Node<'a> {
    None,
    Rule {
        kind: kinds::Rule,
        children: Vec<&'a Node<'a>>,
    },
    Token {
        kind: kinds::Token,
        #[doc = r" Range doesn't include the trivia"]
        range: Range<usize>,
        #[doc = r" Only Trivia"]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        trivia: Vec<&'a Node<'a>>,
    },
    #[doc = r" For anonymous groups referenced from AST nodes i.e. `delimited_by`"]
    Group {
        #[serde(skip_serializing_if = "Vec::is_empty")]
        children: Vec<&'a Node<'a>>,
    },
}
#[allow(unused_variables)]
pub trait Visitor {
    fn enter_rule(
        &mut self,
        kind: kinds::Rule,
        children: &Vec<&Node<'_>>,
        node: &Node<'_>,
    ) -> VisitorEntryResponse {
        VisitorEntryResponse::StepIn
    }
    fn exit_rule(
        &mut self,
        kind: kinds::Rule,
        children: &Vec<&Node<'_>>,
        node: &Node<'_>,
    ) -> VisitorExitResponse {
        VisitorExitResponse::StepIn
    }
    fn enter_token(
        &mut self,
        kind: kinds::Token,
        range: &Range<usize>,
        trivia: &Vec<&Node<'_>>,
        node: &Node<'_>,
    ) -> VisitorEntryResponse {
        VisitorEntryResponse::StepIn
    }
    fn exit_token(
        &mut self,
        kind: kinds::Token,
        range: &Range<usize>,
        trivia: &Vec<&Node<'_>>,
        node: &Node<'_>,
    ) -> VisitorExitResponse {
        VisitorExitResponse::StepIn
    }
    fn enter_group(&mut self, children: &Vec<&Node<'_>>, node: &Node<'_>) -> VisitorEntryResponse {
        VisitorEntryResponse::StepIn
    }
    fn exit_group(&mut self, children: &Vec<&Node<'_>>, node: &Node<'_>) -> VisitorExitResponse {
        VisitorExitResponse::StepIn
    }
    fn visit_none(&mut self, node: &Node<'_>) -> VisitorExitResponse {
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
impl<'a> Node<'a> {
    #[inline]
    pub fn none(context: &'a Context<'a>) -> &'a Node<'a> {
        context.alloc_cst_node(Self::None)
    }
    #[inline]
    pub fn rule(
        context: &'a Context<'a>,
        kind: kinds::Rule,
        children: Vec<&'a Node<'a>>,
    ) -> &'a Node<'a> {
        context.alloc_cst_node(Self::Rule { kind, children })
    }
    #[inline]
    pub fn trivia_token(
        context: &'a Context<'a>,
        range: Range<usize>,
        kind: kinds::Token,
    ) -> &'a Node<'a> {
        context.alloc_cst_node(Self::Token {
            range,
            kind,
            trivia: vec![],
        })
    }
    #[inline]
    pub fn token(
        context: &'a Context<'a>,
        range: Range<usize>,
        kind: kinds::Token,
        leading_trivia: &'a Node<'a>,
        trailing_trivia: &'a Node<'a>,
    ) -> &'a Node<'a> {
        let mut trivia = vec![];
        if *leading_trivia != Node::None {
            trivia.push(leading_trivia)
        }
        if *trailing_trivia != Node::None {
            trivia.push(trailing_trivia)
        }
        context.alloc_cst_node(Self::Token {
            range,
            kind,
            trivia,
        })
    }
    #[inline]
    pub fn group(context: &'a Context<'a>, children: Vec<&'a Node<'a>>) -> &'a Node<'a> {
        if children.is_empty() {
            Self::none(context)
        } else {
            context.alloc_cst_node(Self::Group { children })
        }
    }
}
impl<'a> Visitable for &'a Node<'a> {
    fn visit<T: Visitor>(&self, visitor: &mut T) -> VisitorExitResponse {
        match self {
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
