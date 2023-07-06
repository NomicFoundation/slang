use super::{
    cst::{Node, RuleNode as RustRuleNode, TokenNode as RustTokenNode},
    kinds::*,
    text_index::{TextIndex as RustTextIndex, TextRange as RustTextRange},
};

use std::rc::Rc;

use napi::bindgen_prelude::*;
use napi::JsObject;
use napi::NapiValue;

#[napi(object)]
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

#[napi(object)]
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

#[napi(object)]
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

    #[napi(getter, js_name = "textLength")]
    pub fn text_len(&self) -> TextIndex {
        (&self.0.text_len).into()
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

    #[napi(getter, js_name = "textLength")]
    pub fn text_len(&self) -> TextIndex {
        let text_len: RustTextIndex = (&self.0.text).into();
        (&text_len).into()
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
