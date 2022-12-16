// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::kinds;
use super::lex;
use serde::Serialize;
use std::rc::Rc;
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Node {
    Rule {
        kind: kinds::Rule,
        children: Vec<Rc<Node>>,
    },
    Token {
        kind: kinds::Token,
        lex_node: Rc<lex::Node>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        trivia: Vec<Rc<Node>>,
    },
    #[doc = r" For anonymous groups referenced from AST nodes i.e. `delimited_by`"]
    Group { children: Vec<Rc<Node>> },
}
impl Node {
    pub fn rule(kind: kinds::Rule, children: Vec<Option<Rc<Self>>>) -> Option<Rc<Self>> {
        let children: Vec<_> = children.into_iter().filter_map(|e| e).collect();
        if children.is_empty() {
            None
        } else {
            Some(Rc::new(Self::Rule { kind, children }))
        }
    }
    pub fn trivia_token(kind: kinds::Token, lex_node: Rc<lex::Node>) -> Option<Rc<Self>> {
        Some(Rc::new(Self::Token {
            kind,
            lex_node,
            trivia: vec![],
        }))
    }
    pub fn token(
        kind: kinds::Token,
        lex_node: Rc<lex::Node>,
        leading_trivia: Option<Rc<Self>>,
        trailing_trivia: Option<Rc<Self>>,
    ) -> Option<Rc<Self>> {
        let mut trivia = vec![];
        if let Some(leading_trivia) = leading_trivia {
            trivia.push(leading_trivia)
        }
        if let Some(trailing_trivia) = trailing_trivia {
            trivia.push(trailing_trivia)
        }
        Some(Rc::new(Self::Token {
            kind,
            lex_node,
            trivia,
        }))
    }
    pub fn group(children: Vec<Option<Rc<Self>>>) -> Option<Rc<Self>> {
        let children: Vec<_> = children.into_iter().filter_map(|e| e).collect();
        if children.is_empty() {
            None
        } else {
            Some(Rc::new(Self::Group { children }))
        }
    }
    pub fn top_level_token(lex_node: Option<Rc<lex::Node>>) -> Rc<Self> {
        if let Some(lex_node) = lex_node {
            if let lex::Node::Named(kind, lex_node) = lex_node.as_ref() {
                Rc::new(Self::Token {
                    kind: *kind,
                    lex_node: lex_node.clone(),
                    trivia: vec![],
                })
            } else {
                unreachable!("Top level token unexpected result: {:?}", lex_node)
            }
        } else {
            unreachable!("Top level token unexpected None")
        }
    }
    pub fn top_level_rule(kind: kinds::Rule, node: Option<Rc<Self>>) -> Rc<Self> {
        node.unwrap_or_else(|| {
            Rc::new(Self::Rule {
                kind,
                children: vec![],
            })
        })
    }
}
#[allow(unused_variables)]
pub trait Visitor<E> {
    fn enter_rule(
        &mut self,
        kind: kinds::Rule,
        children: &Vec<Rc<Node>>,
        node: &Rc<Node>,
        path: &Vec<Rc<Node>>,
    ) -> Result<VisitorEntryResponse, E> {
        Ok(VisitorEntryResponse::StepIn)
    }
    fn exit_rule(
        &mut self,
        kind: kinds::Rule,
        children: &Vec<Rc<Node>>,
        node: &Rc<Node>,
        path: &Vec<Rc<Node>>,
    ) -> Result<VisitorExitResponse, E> {
        Ok(VisitorExitResponse::StepIn)
    }
    fn enter_token(
        &mut self,
        kind: kinds::Token,
        lex_node: &Rc<lex::Node>,
        trivia: &Vec<Rc<Node>>,
        node: &Rc<Node>,
        path: &Vec<Rc<Node>>,
    ) -> Result<VisitorEntryResponse, E> {
        Ok(VisitorEntryResponse::StepIn)
    }
    fn exit_token(
        &mut self,
        kind: kinds::Token,
        lex_node: &Rc<lex::Node>,
        trivia: &Vec<Rc<Node>>,
        node: &Rc<Node>,
        path: &Vec<Rc<Node>>,
    ) -> Result<VisitorExitResponse, E> {
        Ok(VisitorExitResponse::StepIn)
    }
    fn enter_group(
        &mut self,
        children: &Vec<Rc<Node>>,
        node: &Rc<Node>,
        path: &Vec<Rc<Node>>,
    ) -> Result<VisitorEntryResponse, E> {
        Ok(VisitorEntryResponse::StepIn)
    }
    fn exit_group(
        &mut self,
        children: &Vec<Rc<Node>>,
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
            Node::Rule { kind, children } => {
                match visitor.enter_rule(*kind, children, self, path)? {
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
                visitor.exit_rule(*kind, children, self, path)
            }
            Node::Token {
                kind,
                lex_node,
                trivia,
            } => {
                match visitor.enter_token(*kind, lex_node, trivia, self, path)? {
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
                visitor.exit_token(*kind, lex_node, trivia, self, path)
            }
            Node::Group { children } => {
                match visitor.enter_group(children, self, path)? {
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
                visitor.exit_group(children, self, path)
            }
        }
    }
}
