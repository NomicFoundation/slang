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
    pub fn to_js(&self, env: &Env) -> Option<JsObject> {
        match self {
            Self::None => None,
            Self::Chars(_) => {
                let mut obj = env.create_object().unwrap();
                obj.set_named_property("flavour", env.create_string("Chars").unwrap())
                    .unwrap();
                Some(obj)
            }
            Self::Choice(_, _) => {
                let mut obj = env.create_object().unwrap();
                obj.set_named_property("flavour", env.create_string("Choice").unwrap())
                    .unwrap();
                Some(obj)
            }
            Self::Sequence(_) => {
                let mut obj = env.create_object().unwrap();
                obj.set_named_property("flavour", env.create_string("Sequence").unwrap())
                    .unwrap();
                Some(obj)
            }
            Self::Named(_, _) => {
                let mut obj = env.create_object().unwrap();
                obj.set_named_property("flavour", env.create_string("Named").unwrap())
                    .unwrap();
                Some(obj)
            }
        }
    }
}
