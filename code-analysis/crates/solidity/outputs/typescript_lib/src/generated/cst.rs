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
impl Node {
    pub fn to_js(&self, env: &Env) -> Option<JsObject> {
        match self {
            Self::None => None,
            Self::Rule { kind, children } => {
                let mut obj = env.create_object().unwrap();
                obj.set_named_property("flavour", env.create_string("Rule").unwrap())
                    .unwrap();
                obj.set_named_property("kind", env.create_uint32(*kind as u32).unwrap())
                    .unwrap();
                let mut js_children = env.create_array_with_length(children.len()).unwrap();
                children
                    .iter()
                    .enumerate()
                    .for_each(|(i, child)| match child.to_js(env) {
                        Some(js_child) => js_children.set_element(i as u32, js_child).unwrap(),
                        None => js_children
                            .set_element(i as u32, env.get_null().unwrap())
                            .unwrap(),
                    });
                obj.set_named_property("children", js_children).unwrap();
                Some(obj)
            }
            Self::Token {
                kind,
                range,
                trivia,
            } => {
                let mut obj = env.create_object().unwrap();
                obj.set_named_property("flavour", env.create_string("Token").unwrap())
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
                trivia
                    .iter()
                    .enumerate()
                    .for_each(|(i, trivium)| match trivium.to_js(env) {
                        Some(js_trivium) => js_trivia.set_element(i as u32, js_trivium).unwrap(),
                        None => js_trivia
                            .set_element(i as u32, env.get_null().unwrap())
                            .unwrap(),
                    });
                obj.set_named_property("trivia", js_trivia).unwrap();
                Some(obj)
            }
            Self::Group { children } => {
                let mut obj = env.create_object().unwrap();
                obj.set_named_property("flavour", env.create_string("Group").unwrap())
                    .unwrap();
                let mut js_children = env.create_array_with_length(children.len()).unwrap();
                children
                    .iter()
                    .enumerate()
                    .for_each(|(i, child)| match child.to_js(env) {
                        Some(js_child) => js_children.set_element(i as u32, js_child).unwrap(),
                        None => js_children
                            .set_element(i as u32, env.get_null().unwrap())
                            .unwrap(),
                    });
                obj.set_named_property("children", js_children).unwrap();
                Some(obj)
            }
        }
    }
}
