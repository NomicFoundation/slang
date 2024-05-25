// NAPI-exposed functions have to accept owned values
#![allow(clippy::needless_pass_by_value)]
// The functions are meant to be definitions for export, so they're not really used
#![allow(clippy::return_self_not_must_use)]

use napi::Either;
use napi_derive::napi;
use text_index::{TextIndex, TextRange};

use crate::napi_interface::cst::{self, NAPINodeExtensions, NonterminalNode, TerminalNode};
use crate::napi_interface::{text_index, EdgeLabel, NonterminalKind, RustCursor, TerminalKind};

#[napi(namespace = "cursor")]
pub struct Cursor(pub(super) RustCursor);

impl From<RustCursor> for Cursor {
    fn from(value: RustCursor) -> Self {
        Self(value)
    }
}

#[napi(namespace = "cursor")]
impl Cursor {
    pub(crate) fn new(cursor: RustCursor) -> Self {
        Self(cursor)
    }

    #[napi(catch_unwind)]
    pub fn reset(&mut self) {
        self.0.reset();
    }

    #[napi(catch_unwind)]
    pub fn complete(&mut self) {
        self.0.complete();
    }

    #[napi(catch_unwind)]
    #[allow(clippy::should_implement_trait)] // These are meant to be explicitly exposed to NAPI
    pub fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    #[napi(catch_unwind)]
    pub fn spawn(&self) -> Self {
        Self::new(self.0.spawn())
    }

    #[napi(getter, catch_unwind)]
    pub fn is_completed(&self) -> bool {
        self.0.is_completed()
    }

    #[napi(ts_return_type = "cst.Node", catch_unwind)]
    pub fn node(&self) -> Either<NonterminalNode, TerminalNode> {
        self.0.node().into_js_either_node()
    }

    #[napi(getter, ts_return_type = "kinds.EdgeLabel", catch_unwind)]
    pub fn label(&self) -> Option<EdgeLabel> {
        self.0.label()
    }

    #[napi(getter, ts_return_type = "text_index.TextIndex", catch_unwind)]
    pub fn text_offset(&self) -> TextIndex {
        self.0.text_offset().into()
    }

    #[napi(getter, ts_return_type = "text_index.TextRange", catch_unwind)]
    pub fn text_range(&self) -> TextRange {
        self.0.text_range().into()
    }

    #[allow(clippy::cast_possible_truncation)] // Cursor depth can't reasonably be larger than u32
    #[napi(getter, catch_unwind)]
    pub fn depth(&self) -> u32 {
        self.0.depth() as u32
    }

    #[napi(ts_return_type = "Array<cst.NonterminalNode>", catch_unwind)]
    pub fn ancestors(&self) -> Vec<cst::NonterminalNode> {
        self.0.ancestors().map(cst::NonterminalNode).collect()
    }

    #[napi(catch_unwind)]
    pub fn go_to_next(&mut self) -> bool {
        self.0.go_to_next()
    }

    #[napi(catch_unwind)]
    pub fn go_to_next_non_descendent(&mut self) -> bool {
        self.0.go_to_next_non_descendent()
    }

    #[napi(catch_unwind)]
    pub fn go_to_previous(&mut self) -> bool {
        self.0.go_to_previous()
    }

    #[napi(catch_unwind)]
    pub fn go_to_parent(&mut self) -> bool {
        self.0.go_to_parent()
    }

    #[napi(catch_unwind)]
    pub fn go_to_first_child(&mut self) -> bool {
        self.0.go_to_first_child()
    }

    #[napi(catch_unwind)]
    pub fn go_to_last_child(&mut self) -> bool {
        self.0.go_to_last_child()
    }

    #[napi(catch_unwind)]
    pub fn go_to_nth_child(&mut self, child_number: u32) -> bool {
        self.0.go_to_nth_child(child_number as usize)
    }

    #[napi(catch_unwind)]
    pub fn go_to_next_sibling(&mut self) -> bool {
        self.0.go_to_next_sibling()
    }

    #[napi(catch_unwind)]
    pub fn go_to_previous_sibling(&mut self) -> bool {
        self.0.go_to_previous_sibling()
    }

    #[napi(catch_unwind)]
    pub fn go_to_next_terminal(&mut self) -> bool {
        self.0.go_to_next_terminal()
    }

    #[napi(catch_unwind)]
    pub fn go_to_next_terminal_with_kind(
        &mut self,
        #[napi(ts_arg_type = "kinds.TerminalKind")] kind: TerminalKind,
    ) -> bool {
        self.0.go_to_next_terminal_with_kind(kind)
    }

    #[napi(catch_unwind)]
    pub fn go_to_next_terminal_with_kinds(
        &mut self,
        #[napi(ts_arg_type = "Array<kinds.TerminalKind>")] kinds: Vec<TerminalKind>,
    ) -> bool {
        self.0.go_to_next_terminal_with_kinds(&kinds)
    }

    #[napi(catch_unwind)]
    pub fn go_to_next_nonterminal(&mut self) -> bool {
        self.0.go_to_next_nonterminal()
    }

    #[napi(catch_unwind)]
    pub fn go_to_next_nonterminal_with_kind(
        &mut self,
        #[napi(ts_arg_type = "kinds.NonterminalKind")] kind: NonterminalKind,
    ) -> bool {
        self.0.go_to_next_nonterminal_with_kind(kind)
    }

    #[napi(catch_unwind)]
    pub fn go_to_next_nonterminal_with_kinds(
        &mut self,
        #[napi(ts_arg_type = "Array<kinds.NonterminalKind>")] kinds: Vec<NonterminalKind>,
    ) -> bool {
        self.0.go_to_next_nonterminal_with_kinds(&kinds)
    }
}
