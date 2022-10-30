// This file is generated via `cargo build`. Please don't edit by hand.

use super::kinds;
use serde::Serialize;
use std::ops::Range;
use std::rc::Rc;
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Node {
    None,
    Chars(Range<usize>),
    Choice(usize, Rc<Node>),
    Sequence(Vec<Rc<Node>>),
    Named(kinds::Token, Rc<Node>),
}
impl Node {
    pub fn range(&self) -> Range<usize> {
        match self {
            Node::None => 0..0,
            Node::Chars(range) => range.clone(),
            Node::Choice(_, element) => element.range(),
            Node::Sequence(elements) => {
                elements[0].range().start..elements[elements.len() - 1].range().end
            }
            Node::Named(_, element) => element.range(),
        }
    }
}
use napi::bindgen_prelude::*;
use napi::JsObject;
use napi::NapiValue;
#[napi]
pub enum LexNodeType {
    None,
    Chars,
    Choice,
    Sequence,
    Named,
}
#[napi]
pub struct LexNoneNode;
#[napi]
pub struct LexCharsNode(Rc<Node>);
#[napi]
pub struct LexChoiceNode(Rc<Node>);
#[napi]
pub struct LexSequenceNode(Rc<Node>);
#[napi]
pub struct LexNamedNode(Rc<Node>);
#[napi]
impl LexNoneNode {
    #[napi(getter, js_name = "type", ts_return_type = "LexNodeType.None")]
    pub fn tipe(&self) -> LexNodeType {
        LexNodeType::None
    }
}
#[napi]
impl LexCharsNode {
    #[napi(getter, js_name = "type", ts_return_type = "LexNodeType.Chars")]
    pub fn tipe(&self) -> LexNodeType {
        LexNodeType::Chars
    }
    #[napi(getter)]
    pub fn start(&self) -> usize {
        if let Node::Chars(range) = self.0.as_ref() {
            range.start
        } else {
            unreachable!("Expected a chars node")
        }
    }
    #[napi(getter)]
    pub fn end(&self) -> usize {
        if let Node::Chars(range) = self.0.as_ref() {
            range.end
        } else {
            unreachable!("Expected a chars node")
        }
    }
}
#[napi]
impl LexChoiceNode {
    #[napi(getter, js_name = "type", ts_return_type = "LexNodeType.Choice")]
    pub fn tipe(&self) -> LexNodeType {
        LexNodeType::Choice
    }
    #[napi(getter)]
    pub fn index(&self) -> usize {
        if let Node::Choice(index, _) = self.0.as_ref() {
            *index
        } else {
            unreachable!("Expected a choice node")
        }
    }
    #[napi(
        ts_return_type = "LexNoneNode | LexCharsNode | LexChoiceNode | LexSequenceNode | LexNamedNode"
    )]
    pub fn child(&self, env: Env) -> JsObject {
        if let Node::Choice(_, child) = self.0.as_ref() {
            child.to_js(&env)
        } else {
            unreachable!("Expected a choice node")
        }
    }
}
#[napi]
impl LexSequenceNode {
    #[napi(getter, js_name = "type", ts_return_type = "LexNodeType.Sequence")]
    pub fn tipe(&self) -> LexNodeType {
        LexNodeType::Sequence
    }
    #[napi(
        ts_return_type = "(LexNoneNode | LexCharsNode | LexChoiceNode | LexSequenceNode | LexNamedNode)[]"
    )]
    pub fn children(&self, env: Env) -> Vec<JsObject> {
        if let Node::Sequence(children) = self.0.as_ref() {
            children.iter().map(|child| child.to_js(&env)).collect()
        } else {
            unreachable!("Expected a sequence node")
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
    pub fn kind(&self) -> kinds::Token {
        if let Node::Named(kind, _) = self.0.as_ref() {
            *kind
        } else {
            unreachable!("Expected a named node")
        }
    }
    #[napi(
        ts_return_type = "LexNoneNode | LexCharsNode | LexChoiceNode | LexSequenceNode | LexNamedNode"
    )]
    pub fn child(&self, env: Env) -> JsObject {
        if let Node::Named(_, child) = self.0.as_ref() {
            child.to_js(&env)
        } else {
            unreachable!("Expected a named node")
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
                <LexNoneNode as ToNapiValue>::to_napi_value(env.raw(), LexNoneNode)
            },
            Node::Chars(_) => unsafe {
                <LexCharsNode as ToNapiValue>::to_napi_value(env.raw(), LexCharsNode(self.clone()))
            },
            Node::Choice(_, _) => unsafe {
                <LexChoiceNode as ToNapiValue>::to_napi_value(
                    env.raw(),
                    LexChoiceNode(self.clone()),
                )
            },
            Node::Sequence(_) => unsafe {
                <LexSequenceNode as ToNapiValue>::to_napi_value(
                    env.raw(),
                    LexSequenceNode(self.clone()),
                )
            },
            Node::Named(_, _) => unsafe {
                <LexNamedNode as ToNapiValue>::to_napi_value(env.raw(), LexNamedNode(self.clone()))
            },
        };
        return unsafe { JsObject::from_raw_unchecked(env.raw(), obj.unwrap()) };
    }
}
