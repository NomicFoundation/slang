// This file is @generated automatically by infrastructure scripts. Please don't edit by hand.

use super::{
    cst::{Node as RustNode, RuleNode as RustRuleNode, TokenNode as RustTokenNode},
    cursor_ts_wrappers::Cursor,
    text_index::{TextIndex as RustTextIndex, TextRange as RustTextRange},
};

use std::rc::Rc;

use napi::bindgen_prelude::*;
use napi::JsObject;
use napi::NapiValue;

use crate::syntax::nodes::{RuleKind, TokenKind};

#[napi(object, namespace = "legacy")]
#[derive(Copy, Clone)]
pub struct TextIndex {
    pub utf8: u32,
    pub utf16: u32,
    pub char: u32,
}

impl From<&RustTextIndex> for TextIndex {
    fn from(value: &RustTextIndex) -> Self {
        Self {
            utf8: value.utf8 as u32,
            utf16: value.utf16 as u32,
            char: value.char as u32,
        }
    }
}

#[napi(object, namespace = "legacy")]
#[derive(Copy, Clone)]
pub struct TextRange {
    pub start: TextIndex,
    pub end: TextIndex,
}

impl From<&RustTextRange> for TextRange {
    fn from(value: &RustTextRange) -> Self {
        Self {
            start: (&value.start).into(),
            end: (&value.end).into(),
        }
    }
}

#[napi(object, namespace = "legacy")]
pub enum NodeType {
    Rule,
    Token,
}

#[napi(namespace = "legacy")]
pub struct RuleNode(Rc<RustRuleNode>);

#[napi(namespace = "legacy")]
pub struct TokenNode(Rc<RustTokenNode>);

#[napi(namespace = "legacy")]
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

    pub fn cursor(&self) -> Cursor {
        Cursor::new(RustNode::Rule(self.0.clone()).cursor())
    }
}

#[napi(namespace = "legacy")]
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
