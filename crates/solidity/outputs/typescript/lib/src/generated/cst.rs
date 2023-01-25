// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::kinds::{RuleKind, TokenKind};
use super::lex;
use serde::Serialize;
use std::rc::Rc;
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Node {
    Rule {
        kind: RuleKind,
        children: Vec<Rc<Node>>,
    },
    Token {
        kind: TokenKind,
        lex_node: Rc<lex::Node>,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        trivia: Vec<Rc<Node>>,
    },
    #[doc = r" For anonymous groups referenced from AST nodes i.e. `delimited_by`"]
    Group { children: Vec<Rc<Node>> },
}
impl Node {
    pub fn rule(kind: RuleKind, children: Vec<Option<Rc<Self>>>) -> Option<Rc<Self>> {
        let children: Vec<_> = children.into_iter().filter_map(|e| e).collect();
        if children.is_empty() {
            return None;
        }
        if children.len() == 1 {
            if let Self::Group { children } = children[0].as_ref() {
                return Some(Rc::new(Self::Rule {
                    kind,
                    children: children.clone(),
                }));
            }
        }
        return Some(Rc::new(Self::Rule { kind, children }));
    }
    pub fn trivia_token(kind: TokenKind, lex_node: Rc<lex::Node>) -> Option<Rc<Self>> {
        Some(Rc::new(Self::Token {
            kind,
            lex_node,
            trivia: vec![],
        }))
    }
    pub fn token(
        kind: TokenKind,
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
            if let lex::NodeContents::Named(kind, lex_node) = &lex_node.contents {
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
    pub fn top_level_rule(kind: RuleKind, node: Option<Rc<Self>>) -> Rc<Self> {
        node.unwrap_or_else(|| {
            Rc::new(Self::Rule {
                kind,
                children: vec![],
            })
        })
    }
}
use super::lex::RcNodeExtensions as LexRcNodeExtensions;
use napi::bindgen_prelude::*;
use napi::JsObject;
use napi::NapiValue;
#[napi]
pub enum NodeType {
    Rule,
    Token,
    Group,
}
#[napi]
pub struct RuleNode(Rc<Node>);
#[napi]
pub struct TokenNode(Rc<Node>);
#[napi]
pub struct GroupNode(Rc<Node>);
#[napi]
impl RuleNode {
    #[napi(getter, js_name = "type", ts_return_type = "NodeType.Rule")]
    pub fn tipe(&self) -> NodeType {
        NodeType::Rule
    }
    #[napi(getter)]
    pub fn kind(&self) -> RuleKind {
        match self.0.as_ref() {
            Node::Rule { kind, .. } => *kind,
            _ => unreachable!(),
        }
    }
    #[napi(ts_return_type = "(RuleNode | TokenNode | GroupNode)[]")]
    pub fn children(&self, env: Env) -> Vec<JsObject> {
        match self.0.as_ref() {
            Node::Rule { children, .. } => children.iter().map(|child| child.to_js(&env)).collect(),
            _ => unreachable!(),
        }
    }
}
#[napi]
impl TokenNode {
    #[napi(getter, js_name = "type", ts_return_type = "NodeType.Token")]
    pub fn tipe(&self) -> NodeType {
        NodeType::Token
    }
    #[napi(getter)]
    pub fn kind(&self) -> TokenKind {
        match self.0.as_ref() {
            Node::Token { kind, .. } => *kind,
            _ => unreachable!(),
        }
    }
    #[napi(ts_return_type = "LexCharsNode | LexChoiceNode | LexSequenceNode | LexNamedNode")]
    pub fn lex_node(&self, env: Env) -> JsObject {
        match self.0.as_ref() {
            Node::Token { lex_node, .. } => lex_node.to_js(&env),
            _ => unreachable!(),
        }
    }
    #[napi(ts_return_type = "(RuleNode | TokenNode | GroupNode)[]")]
    pub fn trivia(&self, env: Env) -> Vec<JsObject> {
        match self.0.as_ref() {
            Node::Token { trivia, .. } => {
                trivia.iter().map(|trivium| trivium.to_js(&env)).collect()
            }
            _ => unreachable!(),
        }
    }
}
#[napi]
impl GroupNode {
    #[napi(getter, js_name = "type", ts_return_type = "NodeType.Group")]
    pub fn tipe(&self) -> NodeType {
        NodeType::Group
    }
    #[napi(ts_return_type = "(RuleNode | TokenNode | GroupNode)[]")]
    pub fn children(&self, env: Env) -> Vec<JsObject> {
        match self.0.as_ref() {
            Node::Group { children, .. } => {
                children.iter().map(|child| child.to_js(&env)).collect()
            }
            _ => unreachable!(),
        }
    }
}
pub trait RcNodeExtensions {
    fn to_js(&self, env: &Env) -> JsObject;
}
impl RcNodeExtensions for Rc<Node> {
    fn to_js(&self, env: &Env) -> JsObject {
        let obj = match self.as_ref() {
            Node::Rule { .. } => unsafe {
                <RuleNode as ToNapiValue>::to_napi_value(env.raw(), RuleNode(self.clone()))
            },
            Node::Token { .. } => unsafe {
                <TokenNode as ToNapiValue>::to_napi_value(env.raw(), TokenNode(self.clone()))
            },
            Node::Group { .. } => unsafe {
                <GroupNode as ToNapiValue>::to_napi_value(env.raw(), GroupNode(self.clone()))
            },
        };
        return unsafe { JsObject::from_raw_unchecked(env.raw(), obj.unwrap()) };
    }
}
