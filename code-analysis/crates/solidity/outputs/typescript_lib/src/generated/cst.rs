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
    Rule {
        kind: kinds::Rule,
        #[serde(skip_serializing_if = "Vec::is_empty")]
        children: Vec<Rc<Node>>,
    },
    Token {
        kind: kinds::Token,
        #[doc = r" Range doesn't include the trivia"]
        range: Range<usize>,
        #[doc = r" Only Trivia"]
        #[serde(skip_serializing_if = "Vec::is_empty")]
        trivia: Vec<Rc<Node>>,
    },
    #[doc = r" For anonymous groups referenced from AST nodes i.e. `delimited_by`"]
    Group {
        #[serde(skip_serializing_if = "Vec::is_empty")]
        children: Vec<Rc<Node>>,
    },
}
#[napi]
pub enum CSTNodeType {
    None,
    Rule,
    Token,
    Group,
}
impl Node {
    pub fn to_js(&self, env: &Env) -> JsObject {
        let mut obj = env.create_object().unwrap();
        match self {
            Self::None => {
                obj.set_named_property(
                    "flavour",
                    env.create_uint32(CSTNodeType::None as u32).unwrap(),
                )
                .unwrap();
            }
            Self::Rule { kind, children } => {
                obj.set_named_property(
                    "flavour",
                    env.create_uint32(CSTNodeType::Rule as u32).unwrap(),
                )
                .unwrap();
                obj.set_named_property("kind", env.create_uint32(*kind as u32).unwrap())
                    .unwrap();
                let mut js_children = env.create_array_with_length(children.len()).unwrap();
                children.iter().enumerate().for_each(|(i, child)| {
                    js_children.set_element(i as u32, child.to_js(env)).unwrap();
                });
                obj.set_named_property("children", js_children).unwrap();
            }
            Self::Token {
                kind,
                range,
                trivia,
            } => {
                obj.set_named_property(
                    "flavour",
                    env.create_uint32(CSTNodeType::Token as u32).unwrap(),
                )
                .unwrap();
                obj.set_named_property("kind", env.create_uint32(*kind as u32).unwrap())
                    .unwrap();
                obj.set_named_property(
                    "start",
                    env.create_uint32(range.start.try_into().unwrap()).unwrap(),
                )
                .unwrap();
                obj.set_named_property(
                    "end",
                    env.create_uint32(range.end.try_into().unwrap()).unwrap(),
                )
                .unwrap();
                let mut js_trivia = env.create_array_with_length(trivia.len()).unwrap();
                trivia.iter().enumerate().for_each(|(i, trivium)| {
                    js_trivia.set_element(i as u32, trivium.to_js(env)).unwrap();
                });
                obj.set_named_property("trivia", js_trivia).unwrap();
            }
            Self::Group { children } => {
                obj.set_named_property(
                    "flavour",
                    env.create_uint32(CSTNodeType::Group as u32).unwrap(),
                )
                .unwrap();
                let mut js_children = env.create_array_with_length(children.len()).unwrap();
                children.iter().enumerate().for_each(|(i, child)| {
                    js_children.set_element(i as u32, child.to_js(env)).unwrap();
                });
                obj.set_named_property("children", js_children).unwrap();
            }
        }
        obj
    }
}
