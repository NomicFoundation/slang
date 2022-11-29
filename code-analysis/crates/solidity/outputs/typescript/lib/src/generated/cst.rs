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
use super::lex::RcNodeExtensions as LexRcNodeExtensions;
use napi::bindgen_prelude::*;
use napi::JsObject;
use napi::NapiValue;
#[napi]
pub enum CSTNodeType {
    None,
    Rule,
    Token,
    Group,
}
#[napi]
pub struct CSTNoneNode;
#[napi]
pub struct CSTRuleNode(Rc<Node>);
#[napi]
pub struct CSTTokenNode(Rc<Node>);
#[napi]
pub struct CSTGroupNode(Rc<Node>);
#[napi]
impl CSTNoneNode {
    #[napi(getter, js_name = "type", ts_return_type = "CSTNodeType.None")]
    pub fn tipe(&self) -> CSTNodeType {
        CSTNodeType::None
    }
}
#[napi]
impl CSTRuleNode {
    #[napi(getter, js_name = "type", ts_return_type = "CSTNodeType.Rule")]
    pub fn tipe(&self) -> CSTNodeType {
        CSTNodeType::Rule
    }
    #[napi(getter)]
    pub fn kind(&self) -> kinds::Rule {
        match self.0.as_ref() {
            Node::Rule { kind, .. } => *kind,
            _ => unreachable!(),
        }
    }
    #[napi(ts_return_type = "(CSTNoneNode | CSTRuleNode | CSTTokenNode | CSTGroupNode)[]")]
    pub fn children(&self, env: Env) -> Vec<JsObject> {
        match self.0.as_ref() {
            Node::Rule { children, .. } => children.iter().map(|child| child.to_js(&env)).collect(),
            _ => unreachable!(),
        }
    }
}
#[napi]
impl CSTTokenNode {
    #[napi(getter, js_name = "type", ts_return_type = "CSTNodeType.Token")]
    pub fn tipe(&self) -> CSTNodeType {
        CSTNodeType::Token
    }
    #[napi(getter)]
    pub fn kind(&self) -> kinds::Token {
        match self.0.as_ref() {
            Node::Token { kind, .. } => *kind,
            _ => unreachable!(),
        }
    }
    #[napi(
        ts_return_type = "LexNoneNode | LexCharsNode | LexChoiceNode | LexSequenceNode | LexNamedNode"
    )]
    pub fn lex_node(&self, env: Env) -> JsObject {
        match self.0.as_ref() {
            Node::Token { lex_node, .. } => lex_node.to_js(&env),
            _ => unreachable!(),
        }
    }
    #[napi(ts_return_type = "(CSTNoneNode | CSTRuleNode | CSTTokenNode | CSTGroupNode)[]")]
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
impl CSTGroupNode {
    #[napi(getter, js_name = "type", ts_return_type = "CSTNodeType.Group")]
    pub fn tipe(&self) -> CSTNodeType {
        CSTNodeType::Group
    }
    #[napi(ts_return_type = "(CSTNoneNode | CSTRuleNode | CSTTokenNode | CSTGroupNode)[]")]
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
            Node::None => unsafe {
                <CSTNoneNode as ToNapiValue>::to_napi_value(env.raw(), CSTNoneNode)
            },
            Node::Rule { .. } => unsafe {
                <CSTRuleNode as ToNapiValue>::to_napi_value(env.raw(), CSTRuleNode(self.clone()))
            },
            Node::Token { .. } => unsafe {
                <CSTTokenNode as ToNapiValue>::to_napi_value(env.raw(), CSTTokenNode(self.clone()))
            },
            Node::Group { .. } => unsafe {
                <CSTGroupNode as ToNapiValue>::to_napi_value(env.raw(), CSTGroupNode(self.clone()))
            },
        };
        return unsafe { JsObject::from_raw_unchecked(env.raw(), obj.unwrap()) };
    }
}
