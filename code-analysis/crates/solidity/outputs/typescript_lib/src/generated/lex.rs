// This file is generated via `cargo build`. Please don't edit by hand.

use super::kinds;
use napi::bindgen_prelude::*;
use napi::JsObject;
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
#[napi]
pub enum LexNodeType {
    None,
    Chars,
    Choice,
    Sequence,
    Named,
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
    pub fn to_js(&self, env: &Env) -> JsObject {
        let mut obj = env.create_object().unwrap();
        match self {
            Self::None => {
                obj.set_named_property(
                    "flavour",
                    env.create_uint32(LexNodeType::None as u32).unwrap(),
                )
                .unwrap();
            }
            Self::Chars(_) => {
                obj.set_named_property(
                    "flavour",
                    env.create_uint32(LexNodeType::Chars as u32).unwrap(),
                )
                .unwrap();
            }
            Self::Choice(_, _) => {
                obj.set_named_property(
                    "flavour",
                    env.create_uint32(LexNodeType::Choice as u32).unwrap(),
                )
                .unwrap();
            }
            Self::Sequence(_) => {
                obj.set_named_property(
                    "flavour",
                    env.create_uint32(LexNodeType::Sequence as u32).unwrap(),
                )
                .unwrap();
            }
            Self::Named(_, _) => {
                obj.set_named_property(
                    "flavour",
                    env.create_uint32(LexNodeType::Named as u32).unwrap(),
                )
                .unwrap();
            }
        }
        obj
    }
}
