// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::{
    cst::{Node, RuleNode as RustRuleNode, TokenNode as RustTokenNode},
    kinds::*,
    text_index::TextIndex,
};

use std::rc::Rc;

use napi::bindgen_prelude::*;
use napi::JsObject;
use napi::NapiValue;

#[napi]
pub enum NodeType {
    Rule,
    Token,
}

#[napi]
pub struct RuleNode(Rc<RustRuleNode>);

#[napi]
pub struct TokenNode(Rc<RustTokenNode>);

#[napi]
impl RuleNode {
    #[napi(getter, js_name = "type", ts_return_type = "NodeType.Rule")]
    pub fn tipe(&self) -> NodeType {
        NodeType::Rule
    }

    #[napi(getter)]
    pub fn kind(&self) -> RuleKind {
        self.0.kind
    }

    #[napi(
        getter,
        ts_return_type = "[ utf8: number, utf16: number, char: number]"
    )]
    pub fn text_len(&self) -> [u32; 3] {
        [
            self.0.text_len.utf8 as u32,
            self.0.text_len.utf16 as u32,
            self.0.text_len.char as u32,
        ]
    }

    #[napi(getter, ts_return_type = "(RuleNode | TokenNode)[]")]
    pub fn children(&self, env: Env) -> Vec<JsObject> {
        self.0
            .children
            .iter()
            .map(|child| child.to_js(&env))
            .collect()
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
        self.0.kind
    }

    #[napi(
        getter,
        ts_return_type = "[ utf8: number, utf16: number, char: number]"
    )]
    pub fn text_len(&self) -> [u32; 3] {
        let text_len: TextIndex = (&self.0.text).into();
        [
            text_len.utf8 as u32,
            text_len.utf16 as u32,
            text_len.char as u32,
        ]
    }
}

impl Node {
    pub fn to_js(&self, env: &Env) -> JsObject {
        let obj = match self {
            Node::Rule(node) => unsafe {
                <RuleNode as ToNapiValue>::to_napi_value(env.raw(), RuleNode(node.clone()))
            },
            Node::Token(node) => unsafe {
                <TokenNode as ToNapiValue>::to_napi_value(env.raw(), TokenNode(node.clone()))
            },
        };
        return unsafe { JsObject::from_raw_unchecked(env.raw(), obj.unwrap()) };
    }
}
