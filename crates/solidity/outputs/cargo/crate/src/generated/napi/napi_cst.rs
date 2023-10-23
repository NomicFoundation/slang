// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::rc::Rc;

use {
    napi::{bindgen_prelude::*, JsObject, NapiValue},
    napi_derive::napi,
};

use super::*;
use napi_cursor::Cursor;
use napi_text_index::TextIndex;

#[napi(object, namespace = "cst")]
pub enum NodeType {
    Rule,
    Token,
}

#[napi(namespace = "cst")]
pub struct RuleNode(Rc<RustRuleNode>);

#[napi(namespace = "cst")]
pub struct TokenNode(Rc<RustTokenNode>);

#[napi(namespace = "cst")]
impl RuleNode {
    #[napi(getter, js_name = "type", ts_return_type = "NodeType.Rule")]
    pub fn tipe(&self) -> NodeType {
        NodeType::Rule
    }

    #[napi(getter)]
    pub fn kind(&self) -> RuleKind {
        self.0.kind
    }

    #[napi(getter, js_name = "textLength")]
    pub fn text_len(&self) -> TextIndex {
        (&self.0.text_len).into()
    }

    #[napi(getter, ts_return_type = "Array<RuleNode | TokenNode>")]
    pub fn children(&self, env: Env) -> Vec<JsObject> {
        self.0
            .children
            .iter()
            .map(|child| child.to_js(&env))
            .collect()
    }

    #[napi(getter)]
    pub fn cursor(&self) -> Cursor {
        Cursor::new(RustNode::Rule(self.0.clone()).cursor())
    }
}

#[napi(namespace = "cst")]
impl TokenNode {
    #[napi(getter, js_name = "type", ts_return_type = "NodeType.Token")]
    pub fn tipe(&self) -> NodeType {
        NodeType::Token
    }

    #[napi(getter)]
    pub fn kind(&self) -> TokenKind {
        self.0.kind
    }
    #[napi(getter, js_name = "textLength")]
    pub fn text_len(&self) -> TextIndex {
        let text_len: RustTextIndex = (&self.0.text).into();
        (&text_len).into()
    }

    #[napi(getter)]
    pub fn text(&self) -> String {
        self.0.text.clone()
    }

    #[napi(getter)]
    pub fn cursor(&self) -> Cursor {
        Cursor::new(RustNode::Token(self.0.clone()).cursor())
    }
}

pub trait ToJS {
    fn to_js(&self, env: &Env) -> JsObject;
}

impl ToJS for Rc<RustRuleNode> {
    fn to_js(&self, env: &Env) -> JsObject {
        let obj =
            unsafe { <RuleNode as ToNapiValue>::to_napi_value(env.raw(), RuleNode(self.clone())) };
        return unsafe { JsObject::from_raw_unchecked(env.raw(), obj.unwrap()) };
    }
}

impl ToJS for Rc<RustTokenNode> {
    fn to_js(&self, env: &Env) -> JsObject {
        let obj = unsafe {
            <TokenNode as ToNapiValue>::to_napi_value(env.raw(), TokenNode(self.clone()))
        };
        return unsafe { JsObject::from_raw_unchecked(env.raw(), obj.unwrap()) };
    }
}

impl ToJS for RustNode {
    fn to_js(&self, env: &Env) -> JsObject {
        match self {
            RustNode::Rule(rust_rule_node) => rust_rule_node.to_js(env),
            RustNode::Token(rust_token_node) => rust_token_node.to_js(env),
        }
    }
}
