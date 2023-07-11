use std::collections::BTreeSet;

use napi::bindgen_prelude::*;

use super::{
    cst,
    cst_ts_wrappers::{TextRange, ToJS},
    parse_error::render_error_report,
    text_index::TextRange as RustTextRange,
};

use crate::syntax::nodes::TokenKind;

#[napi(namespace = "legacy")]
pub struct ParseOutput {
    pub(crate) parse_tree: cst::Node,
    pub(crate) errors: Vec<ParseError>,
}

#[napi(namespace = "legacy")]
impl ParseOutput {
    #[napi(getter, ts_return_type = "RuleNode | TokenNode | null")]
    pub fn parse_tree(&self, env: Env) -> napi::JsObject {
        return self.parse_tree.clone().to_js(&env);
    }

    #[napi(getter)]
    pub fn errors(&self) -> Vec<ParseError> {
        return self.errors.clone();
    }

    #[napi(getter)]
    pub fn is_valid(&self) -> bool {
        return self.errors.is_empty();
    }
}

#[napi(namespace = "legacy")]
#[derive(PartialEq, Clone)]
pub struct ParseError {
    pub(crate) text_range: RustTextRange,
    pub(crate) tokens_that_would_have_allowed_more_progress: Vec<TokenKind>,
}

#[napi(namespace = "legacy")]
impl ParseError {
    #[napi(getter)]
    pub fn text_range(&self) -> TextRange {
        (&self.text_range).into()
    }

    pub fn tokens_that_would_have_allowed_more_progress(&self) -> Vec<String> {
        let tokens_that_would_have_allowed_more_progress = self
            .tokens_that_would_have_allowed_more_progress
            .iter()
            .map(|kind| kind.as_ref())
            .collect::<BTreeSet<_>>();
        return tokens_that_would_have_allowed_more_progress
            .into_iter()
            .map(|token| token.to_string())
            .collect();
    }

    #[napi(namespace = "legacy")]
    pub fn to_error_report(&self, source_id: String, source: String, with_colour: bool) -> String {
        return render_error_report(self, &source_id, &source, with_colour);
    }
}
