// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::collections::BTreeSet;

use super::{cst, kinds::TokenKind, parse_error::render_error_report, text_index::TextRange};
use napi::bindgen_prelude::*;

#[napi]
pub struct ParseOutput {
    pub(crate) parse_tree: cst::Node,
    pub(crate) errors: Vec<ParseError>,
}

#[napi]
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

#[napi]
#[derive(PartialEq, Clone)]
pub struct ParseError {
    pub(crate) range: TextRange,
    pub(crate) tokens_that_would_have_allowed_more_progress: Vec<TokenKind>,
}

#[napi]
impl ParseError {
    #[napi(
        getter,
        ts_return_type = "[ start: [ utf8: number, utf16: number, char: number], end: [ utf8: number, utf16: number, char: number] ]"
    )]
    pub fn range(&self) -> [[u32; 3]; 2] {
        return [
            [
                self.range.start.utf8 as u32,
                self.range.start.utf16 as u32,
                self.range.start.char as u32,
            ],
            [
                self.range.end.utf8 as u32,
                self.range.end.utf16 as u32,
                self.range.end.char as u32,
            ],
        ];
    }

    #[napi(getter)]
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

    #[napi]
    pub fn to_error_report(&self, source_id: String, source: String, with_colour: bool) -> String {
        return render_error_report(self, &source_id, &source, with_colour);
    }
}
