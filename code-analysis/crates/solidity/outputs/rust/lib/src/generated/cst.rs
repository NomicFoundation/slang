// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::kinds;
use super::lex;
use serde::Serialize;
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
        lex_node: Rc<lex::Node>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        trivia: Vec<Rc<Node>>,
    },
    #[doc = r" For anonymous groups referenced from AST nodes i.e. `delimited_by`"]
    Group {
        #[serde(skip_serializing_if = "Vec::is_empty")]
        children: Vec<Rc<Node>>,
    },
}
impl Node {
    pub fn none() -> Rc<Self> {
        Rc::new(Self::None)
    }
    pub fn rule(kind: kinds::Rule, children: Vec<Rc<Self>>) -> Rc<Self> {
        Rc::new(Self::Rule { kind, children })
    }
    pub fn trivia_token(kind: kinds::Token, lex_node: Rc<lex::Node>) -> Rc<Self> {
        Rc::new(Self::Token {
            kind,
            lex_node,
            trivia: vec![],
        })
    }
    pub fn token(
        kind: kinds::Token,
        lex_node: Rc<lex::Node>,
        leading_trivia: Rc<Self>,
        trailing_trivia: Rc<Self>,
    ) -> Rc<Self> {
        let mut trivia = vec![];
        if *leading_trivia != Self::None {
            trivia.push(leading_trivia)
        }
        if *trailing_trivia != Self::None {
            trivia.push(trailing_trivia)
        }
        Rc::new(Self::Token {
            kind,
            lex_node,
            trivia,
        })
    }
    pub fn group(children: Vec<Rc<Self>>) -> Rc<Self> {
        if children.is_empty() {
            Self::none()
        } else {
            Rc::new(Self::Group { children })
        }
    }
}
#[allow(unused_variables)]
pub trait Visitor {
    fn visit_none(&mut self, node: &Rc<Node>, path: &Vec<Rc<Node>>) -> VisitorExitResponse {
        VisitorExitResponse::StepIn
    }
    fn enter_rule(
        &mut self,
        kind: kinds::Rule,
        children: &Vec<Rc<Node>>,
        node: &Rc<Node>,
        path: &Vec<Rc<Node>>,
    ) -> VisitorEntryResponse {
        VisitorEntryResponse::StepIn
    }
    fn exit_rule(
        &mut self,
        kind: kinds::Rule,
        children: &Vec<Rc<Node>>,
        node: &Rc<Node>,
        path: &Vec<Rc<Node>>,
    ) -> VisitorExitResponse {
        VisitorExitResponse::StepIn
    }
    fn enter_token(
        &mut self,
        kind: kinds::Token,
        lex_node: &Rc<lex::Node>,
        trivia: &Vec<Rc<Node>>,
        node: &Rc<Node>,
        path: &Vec<Rc<Node>>,
    ) -> VisitorEntryResponse {
        VisitorEntryResponse::StepIn
    }
    fn exit_token(
        &mut self,
        kind: kinds::Token,
        lex_node: &Rc<lex::Node>,
        trivia: &Vec<Rc<Node>>,
        node: &Rc<Node>,
        path: &Vec<Rc<Node>>,
    ) -> VisitorExitResponse {
        VisitorExitResponse::StepIn
    }
    fn enter_group(
        &mut self,
        children: &Vec<Rc<Node>>,
        node: &Rc<Node>,
        path: &Vec<Rc<Node>>,
    ) -> VisitorEntryResponse {
        VisitorEntryResponse::StepIn
    }
    fn exit_group(
        &mut self,
        children: &Vec<Rc<Node>>,
        node: &Rc<Node>,
        path: &Vec<Rc<Node>>,
    ) -> VisitorExitResponse {
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
    fn visit_with_path<T: Visitor>(
        &self,
        visitor: &mut T,
        path: &mut Vec<Rc<Node>>,
    ) -> VisitorExitResponse;
}
impl Visitable for Rc<Node> {
    fn visit<T: Visitor>(&self, visitor: &mut T) -> VisitorExitResponse {
        self.visit_with_path(visitor, &mut Vec::new())
    }
    fn visit_with_path<T: Visitor>(
        &self,
        visitor: &mut T,
        path: &mut Vec<Rc<Node>>,
    ) -> VisitorExitResponse {
        match self.as_ref() {
            Node::None => visitor.visit_none(self, path),
            Node::Rule { kind, children } => {
                match visitor.enter_rule(*kind, children, self, path) {
                    VisitorEntryResponse::Quit => return VisitorExitResponse::Quit,
                    VisitorEntryResponse::StepOver => {}
                    VisitorEntryResponse::StepIn => {
                        path.push(self.clone());
                        for child in children {
                            match child.visit_with_path(visitor, path) {
                                VisitorExitResponse::Quit => {
                                    path.pop();
                                    return VisitorExitResponse::Quit;
                                }
                                VisitorExitResponse::StepIn => {}
                            }
                        }
                        path.pop();
                    }
                }
                visitor.exit_rule(*kind, children, self, path)
            }
            Node::Token {
                kind,
                lex_node,
                trivia,
            } => {
                match visitor.enter_token(*kind, lex_node, trivia, self, path) {
                    VisitorEntryResponse::Quit => return VisitorExitResponse::Quit,
                    VisitorEntryResponse::StepOver => {}
                    VisitorEntryResponse::StepIn => {
                        path.push(self.clone());
                        for child in trivia {
                            match child.visit_with_path(visitor, path) {
                                VisitorExitResponse::Quit => {
                                    path.pop();
                                    return VisitorExitResponse::Quit;
                                }
                                VisitorExitResponse::StepIn => {}
                            }
                        }
                        path.pop();
                    }
                }
                visitor.exit_token(*kind, lex_node, trivia, self, path)
            }
            Node::Group { children } => {
                match visitor.enter_group(children, self, path) {
                    VisitorEntryResponse::Quit => return VisitorExitResponse::Quit,
                    VisitorEntryResponse::StepOver => {}
                    VisitorEntryResponse::StepIn => {
                        path.push(self.clone());
                        for child in children {
                            match child.visit_with_path(visitor, path) {
                                VisitorExitResponse::Quit => {
                                    path.pop();
                                    return VisitorExitResponse::Quit;
                                }
                                VisitorExitResponse::StepIn => {}
                            }
                        }
                        path.pop();
                    }
                }
                visitor.exit_group(children, self, path)
            }
        }
    }
}
