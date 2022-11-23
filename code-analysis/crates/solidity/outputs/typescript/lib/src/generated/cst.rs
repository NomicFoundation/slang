// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

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
    #[napi(getter)]
    pub fn start(&self) -> usize {
        match self.0.as_ref() {
            Node::Token { range, .. } => range.start,
            _ => unreachable!(),
        }
    }
    #[napi(getter)]
    pub fn end(&self) -> usize {
        match self.0.as_ref() {
            Node::Token { range, .. } => range.end,
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
