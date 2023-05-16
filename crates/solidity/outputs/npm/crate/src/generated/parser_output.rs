// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::{collections::BTreeSet, rc::Rc};

use super::{
    cst,
    cst_types::RcNodeExtensions as CSTRcNodeExtensions,
    language::{render_error_report, TextPosition},
};
use napi::bindgen_prelude::*;

#[napi]
pub struct ParseOutput {
    pub(crate) parse_tree: Option<Rc<cst::Node>>,
    pub(crate) errors: Vec<ParseError>,
}

#[napi]
impl ParseOutput {
    #[napi(getter, ts_return_type = "RuleNode | TokenNode | null")]
    pub fn parse_tree(&self, env: Env) -> Option<napi::JsObject> {
        return self.parse_tree.clone().map(|n| n.to_js(&env));
    }

    #[napi(getter)]
    pub fn errors(&self) -> Vec<ParseError> {
        return self.errors.clone();
    }

    #[napi(getter)]
    pub fn is_valid(&self) -> bool {
        return self.parse_tree.is_some() && self.errors.is_empty();
    }
}

#[napi]
#[derive(PartialEq, Clone)]
pub struct ParseError {
    pub(crate) position: TextPosition,
    pub(crate) expected: BTreeSet<String>,
}

#[napi]
impl ParseError {
    #[napi(getter)]
    pub fn byte_position(&self) -> u32 {
        return self.position.byte as u32;
    }

    #[napi(getter)]
    pub fn char_position(&self) -> u32 {
        return self.position.char as u32;
    }

    #[napi(getter)]
    pub fn expected(&self) -> Vec<String> {
        return self.expected.iter().cloned().collect();
    }

    #[napi]
    pub fn to_error_report(&self, source_id: String, source: String, with_colour: bool) -> String {
        return render_error_report(self, &source_id, &source, with_colour);
    }
}
