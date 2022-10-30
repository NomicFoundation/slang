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
        if let Node::Rule { kind, .. } = self.0.as_ref() {
            *kind
        } else {
            unreachable!("Expected a rule node")
        }
    }
    #[napi(ts_return_type = "(CSTNoneNode | CSTRuleNode | CSTTokenNode | CSTGroupNode)[]")]
    pub fn children(&self, env: Env) -> Vec<JsObject> {
        if let Node::Rule { children, .. } = self.0.as_ref() {
            children.iter().map(|child| child.to_js(&env)).collect()
        } else {
            unreachable!("Expected a rule node")
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
        if let Node::Token { kind, .. } = self.0.as_ref() {
            *kind
        } else {
            unreachable!("Expected a token node")
        }
    }
    #[napi(getter)]
    pub fn start(&self) -> usize {
        if let Node::Token { range, .. } = self.0.as_ref() {
            range.start
        } else {
            unreachable!("Expected a token node")
        }
    }
    #[napi(getter)]
    pub fn end(&self) -> usize {
        if let Node::Token { range, .. } = self.0.as_ref() {
            range.end
        } else {
            unreachable!("Expected a token node")
        }
    }
    #[napi(ts_return_type = "(CSTNoneNode | CSTRuleNode | CSTTokenNode | CSTGroupNode)[]")]
    pub fn trivia(&self, env: Env) -> Vec<JsObject> {
        if let Node::Token { trivia, .. } = self.0.as_ref() {
            trivia.iter().map(|trivium| trivium.to_js(&env)).collect()
        } else {
            unreachable!("Expected a token node")
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
        if let Node::Group { children, .. } = self.0.as_ref() {
            children.iter().map(|child| child.to_js(&env)).collect()
        } else {
            unreachable!("Expected a group node")
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
