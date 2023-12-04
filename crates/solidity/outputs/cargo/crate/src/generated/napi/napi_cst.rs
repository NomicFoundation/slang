// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::rc::Rc;

use {
    napi::{
        bindgen_prelude::{Env, FromNapiValue, ToNapiValue},
        JsObject, NapiValue,
    },
    napi_derive::napi,
};

use super::{
    napi_cursor, napi_text_index, RuleKind, RustNode, RustRuleNode, RustTextIndex, RustTokenNode,
    TokenKind,
};
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

    #[napi(getter, ts_return_type = "kinds.RuleKind")]
    pub fn kind(&self) -> RuleKind {
        self.0.kind
    }

    #[napi(
        getter,
        js_name = "textLength",
        ts_return_type = "text_index.TextIndex"
    )]
    pub fn text_len(&self) -> TextIndex {
        (&self.0.text_len).into()
    }

    #[napi(ts_return_type = "Array<cst.Node>")]
    pub fn children(&self, env: Env) -> Vec<JsObject> {
        self.0
            .children
            .iter()
            .map(|child| child.to_js(&env))
            .collect()
    }

    #[napi(ts_return_type = "cursor.Cursor")]
    pub fn create_cursor(
        &self,
        #[napi(ts_arg_type = "text_index.TextIndex")] text_offset: TextIndex,
    ) -> Cursor {
        RustNode::Rule(self.0.clone())
            .cursor_with_offset((&text_offset).into())
            .into()
    }
}

#[napi(namespace = "cst")]
impl TokenNode {
    #[napi(getter, js_name = "type", ts_return_type = "NodeType.Token")]
    pub fn tipe(&self) -> NodeType {
        NodeType::Token
    }

    #[napi(getter, ts_return_type = "kinds.TokenKind")]
    pub fn kind(&self) -> TokenKind {
        self.0.kind
    }
    #[napi(
        getter,
        js_name = "textLength",
        ts_return_type = "text_index.TextIndex"
    )]
    pub fn text_len(&self) -> TextIndex {
        let text_len: RustTextIndex = (&self.0.text).into();
        (&text_len).into()
    }

    #[napi(getter)]
    pub fn text(&self) -> String {
        self.0.text.clone()
    }

    #[napi(ts_return_type = "cursor.Cursor")]
    pub fn create_cursor(
        &self,
        #[napi(ts_arg_type = "text_index.TextIndex")] text_offset: TextIndex,
    ) -> Cursor {
        RustNode::Token(self.0.clone())
            .cursor_with_offset((&text_offset).into())
            .into()
    }
}

pub trait ToJS {
    fn to_js(&self, env: &Env) -> JsObject;
}

impl ToJS for Rc<RustRuleNode> {
    fn to_js(&self, env: &Env) -> JsObject {
        let obj =
            unsafe { <RuleNode as ToNapiValue>::to_napi_value(env.raw(), RuleNode(self.clone())) };
        unsafe { JsObject::from_raw_unchecked(env.raw(), obj.unwrap()) }
    }
}

impl ToJS for Rc<RustTokenNode> {
    fn to_js(&self, env: &Env) -> JsObject {
        let obj = unsafe {
            <TokenNode as ToNapiValue>::to_napi_value(env.raw(), TokenNode(self.clone()))
        };
        unsafe { JsObject::from_raw_unchecked(env.raw(), obj.unwrap()) }
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
