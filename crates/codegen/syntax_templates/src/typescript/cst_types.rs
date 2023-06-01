use super::cst::Node;
use super::kinds::*;
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
pub struct RuleNode(Rc<Node>);

#[napi]
pub struct TokenNode(Rc<Node>);

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

    #[napi(getter, ts_return_type = "[ start: number, end: number ]")]
    pub fn utf8_range(&self) -> [u32; 2] {
        let range = self.0.range();
        [range.start.utf8 as u32, range.end.utf8 as u32]
    }

    #[napi(getter, ts_return_type = "[ start: number, end: number ]")]
    pub fn utf16_range(&self) -> [u32; 2] {
        let range = self.0.range();
        [range.start.utf16 as u32, range.end.utf16 as u32]
    }

    #[napi(getter, ts_return_type = "[ start: number, end: number ]")]
    pub fn char_range(&self) -> [u32; 2] {
        let range = self.0.range();
        [range.start.char as u32, range.end.char as u32]
    }

    #[napi(getter, ts_return_type = "[ start: number, end: number ]")]
    pub fn utf8_range_including_trivia(&self) -> [u32; 2] {
        let range = self.0.range_including_trivia();
        [range.start.utf8 as u32, range.end.utf8 as u32]
    }

    #[napi(getter, ts_return_type = "[ start: number, end: number ]")]
    pub fn utf16_range_including_trivia(&self) -> [u32; 2] {
        let range = self.0.range_including_trivia();
        [range.start.utf16 as u32, range.end.utf16 as u32]
    }

    #[napi(getter, ts_return_type = "[ start: number, end: number ]")]
    pub fn char_range_including_trivia(&self) -> [u32; 2] {
        let range = self.0.range_including_trivia();
        [range.start.char as u32, range.end.char as u32]
    }

    #[napi(getter, ts_return_type = "(RuleNode | TokenNode)[]")]
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

    #[napi(getter, ts_return_type = "[ start: number, end: number ]")]
    pub fn utf8_range(&self) -> [u32; 2] {
        let range = self.0.range();
        [range.start.utf8 as u32, range.end.utf8 as u32]
    }

    #[napi(getter, ts_return_type = "[ start: number, end: number ]")]
    pub fn utf16_range(&self) -> [u32; 2] {
        let range = self.0.range();
        [range.start.utf16 as u32, range.end.utf16 as u32]
    }

    #[napi(getter, ts_return_type = "[ start: number, end: number ]")]
    pub fn char_range(&self) -> [u32; 2] {
        let range = self.0.range();
        [range.start.char as u32, range.end.char as u32]
    }

    #[napi(getter, ts_return_type = "[ start: number, end: number ]")]
    pub fn utf8_range_including_trivia(&self) -> [u32; 2] {
        let range = self.0.range_including_trivia();
        [range.start.utf8 as u32, range.end.utf8 as u32]
    }

    #[napi(getter, ts_return_type = "[ start: number, end: number ]")]
    pub fn utf16_range_including_trivia(&self) -> [u32; 2] {
        let range = self.0.range_including_trivia();
        [range.start.utf16 as u32, range.end.utf16 as u32]
    }

    #[napi(getter, ts_return_type = "[ start: number, end: number ]")]
    pub fn char_range_including_trivia(&self) -> [u32; 2] {
        let range = self.0.range_including_trivia();
        [range.start.char as u32, range.end.char as u32]
    }

    #[napi(getter, ts_return_type = "(RuleNode | TokenNode)[]")]
    pub fn trivia(&self, env: Env) -> Vec<JsObject> {
        match self.0.as_ref() {
            Node::Token { trivia, .. } => {
                trivia.iter().map(|trivium| trivium.to_js(&env)).collect()
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
        };
        return unsafe { JsObject::from_raw_unchecked(env.raw(), obj.unwrap()) };
    }
}
