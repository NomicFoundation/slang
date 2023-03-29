use std::rc::Rc;

use super::{
    cst,
    cst_types::RcNodeExtensions as CSTRcNodeExtensions,
    language::{render_error_report, ParseError},
};
use napi::bindgen_prelude::*;

#[napi]
pub struct ParserOutput {
    pub(crate) parse_tree: Option<Rc<cst::Node>>,
    pub(crate) errors: Vec<ParseError>,
}

#[napi]
impl ParserOutput {
    #[napi(ts_return_type = "RuleNode | TokenNode | null")]
    pub fn parse_tree(&self, env: Env) -> Option<napi::JsObject> {
        self.parse_tree.clone().map(|n| n.to_js(&env))
    }

    #[napi]
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    #[napi]
    pub fn errors_as_strings(
        &self,
        source_id: String,
        source: String,
        with_colour: bool,
    ) -> Vec<String> {
        return self
            .errors
            .iter()
            .map(|error| render_error_report(error, &source_id, &source, with_colour))
            .collect();
    }

    #[napi]
    pub fn is_valid(&self) -> bool {
        self.parse_tree.is_some() && self.errors.is_empty()
    }
}
