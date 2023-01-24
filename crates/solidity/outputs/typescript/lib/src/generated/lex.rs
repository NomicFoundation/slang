// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::kinds::TokenKind;
use serde::Serialize;
use std::ops::Range;
use std::rc::Rc;
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Node {
    pub(crate) contents: NodeContents,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) enum NodeContents {
    Chars(Range<usize>),
    Sequence(Vec<Rc<Node>>),
    Named(TokenKind, Rc<Node>),
}
impl Node {
    pub(crate) fn new(contents: NodeContents) -> Self {
        Self { contents }
    }
    pub fn chars(range: Range<usize>) -> Option<Rc<Self>> {
        Some(Rc::new(Self::new(NodeContents::Chars(range))))
    }
    pub fn chars_unwrapped(range: Range<usize>) -> Rc<Self> {
        Rc::new(Self::new(NodeContents::Chars(range)))
    }
    pub fn sequence(elements: Vec<Option<Rc<Self>>>) -> Option<Rc<Self>> {
        let elements: Vec<_> = elements.into_iter().filter_map(|e| e).collect();
        if elements.is_empty() {
            None
        } else {
            Some(Rc::new(Self::new(NodeContents::Sequence(elements))))
        }
    }
    pub fn named(kind: TokenKind, element: Option<Rc<Self>>) -> Option<Rc<Self>> {
        element.map(|e| Rc::new(Self::new(NodeContents::Named(kind, e))))
    }
    pub fn range(&self) -> Range<usize> {
        match &self.contents {
            NodeContents::Chars(range) => range.clone(),
            NodeContents::Sequence(elements) => {
                elements[0].range().start..elements[elements.len() - 1].range().end
            }
            NodeContents::Named(_, element) => element.range(),
        }
    }
}
use napi::bindgen_prelude::*;
use napi::JsObject;
use napi::NapiValue;
#[napi]
pub enum LexNodeType {
    Chars,
    Sequence,
    Named,
}
#[napi(object)]
pub struct TokenRange {
    pub start: u32,
    pub end: u32,
}
#[napi]
pub struct LexCharsNode(Rc<Node>);
#[napi]
pub struct LexSequenceNode(Rc<Node>);
#[napi]
pub struct LexNamedNode(Rc<Node>);
#[napi]
impl LexCharsNode {
    #[napi(getter, js_name = "type", ts_return_type = "LexNodeType.Chars")]
    pub fn tipe(&self) -> LexNodeType {
        LexNodeType::Chars
    }
    #[napi(getter)]
    pub fn range(&self) -> TokenRange {
        match &self.0.contents {
            NodeContents::Chars(range) => TokenRange {
                start: range.start as u32,
                end: range.end as u32,
            },
            _ => unreachable!(),
        }
    }
}
#[napi]
impl LexSequenceNode {
    #[napi(getter, js_name = "type", ts_return_type = "LexNodeType.Sequence")]
    pub fn tipe(&self) -> LexNodeType {
        LexNodeType::Sequence
    }
    #[napi(getter)]
    pub fn range(&self) -> TokenRange {
        let range = self.0.range();
        TokenRange {
            start: range.start as u32,
            end: range.end as u32,
        }
    }
    #[napi(ts_return_type = "(LexCharsNode | LexSequenceNode | LexNamedNode)[]")]
    pub fn children(&self, env: Env) -> Vec<JsObject> {
        match &self.0.contents {
            NodeContents::Sequence(children) => {
                children.iter().map(|child| child.to_js(&env)).collect()
            }
            _ => unreachable!(),
        }
    }
}
#[napi]
impl LexNamedNode {
    #[napi(getter, js_name = "type", ts_return_type = "LexNodeType.Named")]
    pub fn tipe(&self) -> LexNodeType {
        LexNodeType::Named
    }
    #[napi(getter)]
    pub fn range(&self) -> TokenRange {
        let range = self.0.range();
        TokenRange {
            start: range.start as u32,
            end: range.end as u32,
        }
    }
    #[napi(getter)]
    pub fn kind(&self) -> TokenKind {
        match &self.0.contents {
            NodeContents::Named(kind, _) => *kind,
            _ => unreachable!(),
        }
    }
    #[napi(ts_return_type = "LexCharsNode | LexSequenceNode | LexNamedNode")]
    pub fn child(&self, env: Env) -> JsObject {
        match &self.0.contents {
            NodeContents::Named(_, child) => child.to_js(&env),
            _ => unreachable!(),
        }
    }
}
pub trait RcNodeExtensions {
    fn to_js(&self, env: &Env) -> JsObject;
}
impl RcNodeExtensions for Rc<Node> {
    fn to_js(&self, env: &Env) -> JsObject {
        let obj = match &self.contents {
            NodeContents::Chars(_) => unsafe {
                <LexCharsNode as ToNapiValue>::to_napi_value(env.raw(), LexCharsNode(self.clone()))
            },
            NodeContents::Sequence(_) => unsafe {
                <LexSequenceNode as ToNapiValue>::to_napi_value(
                    env.raw(),
                    LexSequenceNode(self.clone()),
                )
            },
            NodeContents::Named(_, _) => unsafe {
                <LexNamedNode as ToNapiValue>::to_napi_value(env.raw(), LexNamedNode(self.clone()))
            },
        };
        return unsafe { JsObject::from_raw_unchecked(env.raw(), obj.unwrap()) };
    }
}
