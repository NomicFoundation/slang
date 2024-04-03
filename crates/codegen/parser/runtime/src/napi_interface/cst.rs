use std::rc::Rc;

use napi::bindgen_prelude::Env;
use napi::JsObject;
use napi_derive::napi;

use crate::napi_interface::cursor::Cursor;
use crate::napi_interface::text_index::TextIndex;
use crate::napi_interface::{
    RuleKind, RustNode, RustRuleNode, RustTextIndex, RustTokenNode, TokenKind,
};

#[napi(namespace = "cst")]
pub enum NodeType {
    Rule,
    Token,
}

#[napi(namespace = "cst")]
pub struct RuleNode(pub(crate) Rc<RustRuleNode>);

#[napi(namespace = "cst")]
pub struct TokenNode(pub(crate) Rc<RustTokenNode>);

#[napi(namespace = "cst")]
impl RuleNode {
    #[napi(
        getter,
        js_name = "type",
        ts_return_type = "NodeType.Rule",
        catch_unwind
    )]
    pub fn tipe(&self) -> NodeType {
        NodeType::Rule
    }

    #[napi(getter, ts_return_type = "kinds.RuleKind", catch_unwind)]
    pub fn kind(&self) -> RuleKind {
        self.0.kind
    }

    #[napi(
        getter,
        js_name = "textLength",
        ts_return_type = "text_index.TextIndex",
        catch_unwind
    )]
    pub fn text_len(&self) -> TextIndex {
        (&self.0.text_len).into()
    }

    #[napi(ts_return_type = "Array<cst.Node>", catch_unwind)]
    pub fn children(&self, env: Env) -> Vec<JsObject> {
        self.0
            .children
            .iter()
            .map(|child| child.to_js(env))
            .collect()
    }

    #[napi(ts_return_type = "cursor.Cursor", catch_unwind)]
    pub fn create_cursor(
        &self,
        #[napi(ts_arg_type = "text_index.TextIndex")] text_offset: TextIndex,
    ) -> Cursor {
        RustNode::Rule(self.0.clone())
            .cursor_with_offset((&text_offset).into())
            .into()
    }

    #[napi(catch_unwind)]
    pub fn unparse(&self) -> String {
        self.0.clone().unparse()
    }
}

#[napi(namespace = "cst")]
impl TokenNode {
    #[napi(
        getter,
        js_name = "type",
        ts_return_type = "NodeType.Token",
        catch_unwind
    )]
    pub fn tipe(&self) -> NodeType {
        NodeType::Token
    }

    #[napi(getter, ts_return_type = "kinds.TokenKind", catch_unwind)]
    pub fn kind(&self) -> TokenKind {
        self.0.kind
    }

    #[napi(
        getter,
        js_name = "textLength",
        ts_return_type = "text_index.TextIndex",
        catch_unwind
    )]
    pub fn text_len(&self) -> TextIndex {
        let text_len: RustTextIndex = (&self.0.text).into();
        (&text_len).into()
    }

    #[napi(getter, catch_unwind)]
    pub fn text(&self) -> String {
        self.0.text.clone()
    }

    #[napi(ts_return_type = "cursor.Cursor", catch_unwind)]
    pub fn create_cursor(
        &self,
        #[napi(ts_arg_type = "text_index.TextIndex")] text_offset: TextIndex,
    ) -> Cursor {
        RustNode::Token(self.0.clone())
            .cursor_with_offset((&text_offset).into())
            .into()
    }
}

pub(crate) trait ToJS {
    fn to_js(&self, env: Env) -> JsObject;
}

impl ToJS for Rc<RustRuleNode> {
    fn to_js(&self, env: Env) -> JsObject {
        RuleNode(self.clone())
            .into_instance(env)
            .expect("Class constructor to be defined by #[napi]")
            .as_object(env)
    }
}

impl ToJS for Rc<RustTokenNode> {
    fn to_js(&self, env: Env) -> JsObject {
        TokenNode(self.clone())
            .into_instance(env)
            .expect("Class constructor to be defined by #[napi]")
            .as_object(env)
    }
}

impl ToJS for RustNode {
    fn to_js(&self, env: Env) -> JsObject {
        match self {
            RustNode::Rule(rust_rule_node) => rust_rule_node.to_js(env),
            RustNode::Token(rust_token_node) => rust_token_node.to_js(env),
        }
    }
}
