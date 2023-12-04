// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use {
    napi::{bindgen_prelude::Env, JsObject},
    napi_derive::napi,
};

use super::{napi_cst, napi_text_index, RuleKind, RustCursor, TokenKind};
use napi_cst::ToJS;
use napi_text_index::{TextIndex, TextRange};

#[napi(namespace = "cursor")]
pub struct Cursor(Box<RustCursor>);

impl From<RustCursor> for Cursor {
    fn from(value: RustCursor) -> Self {
        Self(value.into())
    }
}

#[napi(namespace = "cursor")]
impl Cursor {
    pub(crate) fn new(cursor: RustCursor) -> Self {
        Self(Box::new(cursor))
    }

    #[napi]
    pub fn reset(&mut self) {
        self.0.reset()
    }

    #[napi]
    pub fn complete(&mut self) {
        self.0.complete()
    }

    #[napi]
    #[allow(clippy::should_implement_trait)] // These are meant to be explicitly exposed to NAPI
    pub fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    #[napi]
    pub fn spawn(&self) -> Self {
        Self::new(self.0.spawn())
    }

    #[napi(getter)]
    pub fn is_completed(&self) -> bool {
        self.0.is_completed()
    }

    #[napi(ts_return_type = "cst.Node")]
    pub fn node(&self, env: Env) -> JsObject {
        self.0.node().to_js(&env)
    }

    #[napi(getter, ts_return_type = "text_index.TextIndex")]
    pub fn text_offset(&self) -> TextIndex {
        (&self.0.text_offset()).into()
    }

    #[napi(getter, ts_return_type = "text_index.TextRange")]
    pub fn text_range(&self) -> TextRange {
        (&self.0.text_range()).into()
    }

    #[napi(getter)]
    pub fn depth(&self) -> u32 {
        self.0.depth() as u32
    }

    #[napi(ts_return_type = "Array<cst.RuleNode>")]
    pub fn ancestors(&self, env: Env) -> Vec<JsObject> {
        self.0
            .ancestors()
            .map(|rust_rule_node| rust_rule_node.to_js(&env))
            .collect()
    }

    #[napi]
    pub fn go_to_next(&mut self) -> bool {
        self.0.go_to_next()
    }

    #[napi]
    pub fn go_to_next_non_descendent(&mut self) -> bool {
        self.0.go_to_next_non_descendent()
    }

    #[napi]
    pub fn go_to_previous(&mut self) -> bool {
        self.0.go_to_previous()
    }

    #[napi]
    pub fn go_to_parent(&mut self) -> bool {
        self.0.go_to_parent()
    }

    #[napi]
    pub fn go_to_first_child(&mut self) -> bool {
        self.0.go_to_first_child()
    }

    #[napi]
    pub fn go_to_last_child(&mut self) -> bool {
        self.0.go_to_last_child()
    }

    #[napi]
    pub fn go_to_nth_child(&mut self, child_number: u32) -> bool {
        self.0.go_to_nth_child(child_number as usize)
    }

    #[napi]
    pub fn go_to_next_sibling(&mut self) -> bool {
        self.0.go_to_next_sibling()
    }

    #[napi]
    pub fn go_to_previous_sibling(&mut self) -> bool {
        self.0.go_to_previous_sibling()
    }

    #[napi]
    pub fn go_to_next_token(&mut self) -> bool {
        self.0.go_to_next_token()
    }

    #[napi]
    pub fn go_to_next_token_with_kinds(
        &mut self,
        #[napi(ts_arg_type = "Array<kinds.TokenKind>")] kinds: Vec<TokenKind>,
    ) -> bool {
        self.0.go_to_next_token_with_kinds(&kinds)
    }

    #[napi]
    pub fn go_to_next_rule(&mut self) -> bool {
        self.0.go_to_next_rule()
    }

    #[napi]
    pub fn go_to_next_rule_with_kinds(
        &mut self,
        #[napi(ts_arg_type = "Array<kinds.RuleKind>")] kinds: Vec<RuleKind>,
    ) -> bool {
        self.0.go_to_next_rule_with_kinds(&kinds)
    }
}
