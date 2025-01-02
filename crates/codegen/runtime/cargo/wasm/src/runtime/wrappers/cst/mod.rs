use std::rc::Rc;

use crate::wasm_crate::utils::{
    define_rc_wrapper, define_refcell_wrapper, define_wrapper, enum_to_enum, FromFFI, IntoFFI,
};

mod ffi {
    pub use crate::wasm_crate::bindgen::exports::nomic_foundation::slang::cst::{
        AncestorsIterator, AncestorsIteratorBorrow, Cursor, CursorBorrow, CursorIterator,
        CursorIteratorBorrow, Edge, EdgeLabel, Guest, GuestAncestorsIterator, GuestCursor,
        GuestCursorIterator, GuestNonterminalNode, GuestQuery, GuestQueryMatchIterator,
        GuestTerminalKindExtensions, GuestTerminalNode, Node, NonterminalKind, NonterminalNode,
        NonterminalNodeBorrow, Query, QueryBorrow, QueryError, QueryMatch, QueryMatchIterator,
        QueryMatchIteratorBorrow, TerminalKind, TerminalNode, TerminalNodeBorrow, TextIndex,
        TextRange,
    };
}

mod rust {
    pub use crate::rust_crate::cst::{
        AncestorsIterator, Cursor, CursorIterator, Edge, EdgeLabel, Node, NonterminalKind,
        NonterminalNode, Query, QueryError, QueryMatch, QueryMatchIterator, TerminalKind,
        TerminalNode, TextIndex, TextRange,
    };
}

impl ffi::Guest for crate::wasm_crate::World {
    type TerminalKindExtensions = TerminalKindExtensionsWrapper;

    type NonterminalNode = NonterminalNodeWrapper;
    type TerminalNode = TerminalNodeWrapper;

    type Cursor = CursorWrapper;
    type CursorIterator = CursorIteratorWrapper;
    type AncestorsIterator = AncestorsIteratorWrapper;

    type Query = QueryWrapper;
    type QueryMatchIterator = QueryMatchIteratorWrapper;
}

//================================================
//
// enum nonterminal-kind
//
//================================================

enum_to_enum!(NonterminalKind);

//================================================
//
// enum terminal-kind
//
//================================================

enum_to_enum!(TerminalKind);

//================================================
//
// resource terminal-kind-extensions
//
//================================================

pub struct TerminalKindExtensionsWrapper;

impl ffi::GuestTerminalKindExtensions for TerminalKindExtensionsWrapper {
    fn is_trivia(kind: ffi::TerminalKind) -> bool {
        crate::rust_crate::cst::TerminalKindExtensions::is_trivia(&kind._from_ffi())
    }

    fn is_valid(kind: ffi::TerminalKind) -> bool {
        crate::rust_crate::cst::TerminalKindExtensions::is_valid(&kind._from_ffi())
    }
}

//================================================
//
// enum edge-label
//
//================================================

enum_to_enum!(EdgeLabel);

//================================================
//
// variant node
//
//================================================

impl IntoFFI<ffi::Node> for rust::Node {
    #[inline]
    fn _into_ffi(self) -> ffi::Node {
        match self {
            Self::Nonterminal(node) => ffi::Node::Nonterminal(node._into_ffi()),
            Self::Terminal(node) => ffi::Node::Terminal(node._into_ffi()),
        }
    }
}

//================================================
//
// resource nonterminal-node
//
//================================================

define_rc_wrapper! { NonterminalNode {
    fn id(&self) -> u32 {
        self._borrow_ffi().id().try_into().unwrap()
    }

    fn kind(&self) -> ffi::NonterminalKind {
        self._borrow_ffi().kind._into_ffi()
    }

    fn text_length(&self) -> ffi::TextIndex {
        self._borrow_ffi().text_len._into_ffi()
    }

    fn children(&self) -> Vec<ffi::Edge> {
        self._borrow_ffi().children().iter().cloned().map(IntoFFI::_into_ffi).collect()
    }

    fn descendants(&self) -> ffi::CursorIterator {
        Rc::clone(self._borrow_ffi()).descendants()._into_ffi()
    }

    fn unparse(&self) -> String {
        self._borrow_ffi().unparse()
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self._borrow_ffi()).unwrap()
    }

    fn create_cursor(&self, text_offset: ffi::TextIndex) -> ffi::Cursor {
        std::rc::Rc::clone(self._borrow_ffi()).cursor_with_offset(text_offset._from_ffi())._into_ffi()
    }
} }

//================================================
//
// resource terminal-node
//
//================================================

define_rc_wrapper! { TerminalNode {
    fn id(&self) -> u32 {
        self._borrow_ffi().id().try_into().unwrap()
    }

    fn kind(&self) -> ffi::TerminalKind {
        self._borrow_ffi().kind._into_ffi()
    }

    fn text_length(&self) -> ffi::TextIndex {
        rust::TextIndex::from(&self._borrow_ffi().text)._into_ffi()
    }

    fn children(&self) -> Vec<ffi::Edge> {
        self._borrow_ffi().children().iter().cloned().map(IntoFFI::_into_ffi).collect()
    }

    fn descendants(&self) -> ffi::CursorIterator {
        Rc::clone(self._borrow_ffi()).descendants()._into_ffi()
    }

    fn unparse(&self) -> String {
        self._borrow_ffi().unparse()
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self._borrow_ffi()).unwrap()
    }
} }

//================================================
//
// record edge
//
//================================================

impl IntoFFI<ffi::Edge> for rust::Edge {
    #[inline]
    fn _into_ffi(self) -> ffi::Edge {
        ffi::Edge {
            label: self.label._into_ffi(),
            node: self.node._into_ffi(),
        }
    }
}

//================================================
//
// resource cursor
//
//================================================

define_refcell_wrapper! { Cursor {
    fn reset(&self) {
        self._borrow_mut_ffi().reset();
    }

    fn complete(&self) {
        self._borrow_mut_ffi().complete();
    }

    fn is_completed(&self) -> bool {
        self._borrow_ffi().is_completed()
    }

    fn clone(&self) -> ffi::Cursor {
        self._borrow_ffi().clone()._into_ffi()
    }

    fn spawn(&self) -> ffi::Cursor {
        self._borrow_ffi().spawn()._into_ffi()
    }

    fn node(&self) -> ffi::Node {
        self._borrow_ffi().node()._into_ffi()
    }

    fn label(&self) -> ffi::EdgeLabel {
        self._borrow_ffi().label()._into_ffi()
    }

    fn text_offset(&self) -> ffi::TextIndex {
        self._borrow_ffi().text_offset()._into_ffi()
    }

    fn text_range(&self) -> ffi::TextRange {
        self._borrow_ffi().text_range()._into_ffi()
    }

    fn depth(&self) -> u32 {
        self._borrow_ffi().depth().try_into().unwrap()
    }

    fn children(&self) -> Vec<ffi::Edge> {
        self._borrow_ffi().children().iter().cloned().map(IntoFFI::_into_ffi).collect()
    }

    fn descendants(&self) -> ffi::CursorIterator {
        self._borrow_ffi().descendants()._into_ffi()
    }

    fn remaining_nodes(&self) -> ffi::CursorIterator {
        self._borrow_ffi().remaining_nodes()._into_ffi()
    }

    fn ancestors(&self) -> ffi::AncestorsIterator {
        self._borrow_ffi().ancestors()._into_ffi()
    }

    fn go_to_next(&self) -> bool {
        self._borrow_mut_ffi().go_to_next()
    }

    fn go_to_next_non_descendant(&self) -> bool {
        self._borrow_mut_ffi().go_to_next_non_descendant()
    }

    fn go_to_previous(&self) -> bool {
        self._borrow_mut_ffi().go_to_previous()
    }

    fn go_to_parent(&self) -> bool {
        self._borrow_mut_ffi().go_to_parent()
    }

    fn go_to_first_child(&self) -> bool {
        self._borrow_mut_ffi().go_to_first_child()
    }

    fn go_to_last_child(&self) -> bool {
        self._borrow_mut_ffi().go_to_last_child()
    }

    fn go_to_nth_child(&self, child_number: u32) -> bool {
        self._borrow_mut_ffi().go_to_nth_child(child_number as usize)
    }

    fn go_to_next_sibling(&self) -> bool {
        self._borrow_mut_ffi().go_to_next_sibling()
    }

    fn go_to_previous_sibling(&self) -> bool {
        self._borrow_mut_ffi().go_to_previous_sibling()
    }

    fn go_to_next_terminal(&self) -> bool {
        self._borrow_mut_ffi().go_to_next_terminal()
    }

    fn go_to_next_terminal_with_kind(&self, kind: ffi::TerminalKind) -> bool {
        self._borrow_mut_ffi().go_to_next_terminal_with_kind(kind._from_ffi())
    }

    fn go_to_next_terminal_with_kinds(&self, kinds: Vec<ffi::TerminalKind>) -> bool {
        let kinds = kinds.into_iter().map(FromFFI::_from_ffi).collect::<Vec<_>>();
        self._borrow_mut_ffi().go_to_next_terminal_with_kinds(&kinds)
    }

    fn go_to_next_nonterminal(&self) -> bool {
        self._borrow_mut_ffi().go_to_next_nonterminal()
    }

    fn go_to_next_nonterminal_with_kind(&self, kind: ffi::NonterminalKind) -> bool {
        self._borrow_mut_ffi().go_to_next_nonterminal_with_kind(kind._from_ffi())
    }

    fn go_to_next_nonterminal_with_kinds(&self, kinds: Vec<ffi::NonterminalKind>) -> bool {
        let kinds = kinds.into_iter().map(FromFFI::_from_ffi).collect::<Vec<_>>();
        self._borrow_mut_ffi().go_to_next_nonterminal_with_kinds(&kinds)
    }

    fn query(&self, queries: Vec<ffi::QueryBorrow<'_>>) -> ffi::QueryMatchIterator {
        let queries:Vec<rust::Query> = queries.into_iter().map(|q|{
            q._borrow_ffi().clone()
        }).collect();

        self._borrow_ffi().clone().query(queries)._into_ffi()
    }
} }

//================================================
//
// resource cursor-iterator
//
//================================================

define_refcell_wrapper! { CursorIterator {
    fn next(&self) -> Option<ffi::Edge> {
        self._borrow_mut_ffi().next().map(IntoFFI::_into_ffi)
    }
} }

//================================================
//
// resource ancestors-iterator
//
//================================================

define_refcell_wrapper! { AncestorsIterator {
    fn next(&self) -> Option<ffi::NonterminalNode> {
        self._borrow_mut_ffi().next().map(IntoFFI::_into_ffi)
    }
} }

//================================================
//
// resource query
//
//================================================

define_wrapper! { Query {
    fn parse(text: String) -> Result<ffi::Query, ffi::QueryError> {
        rust::Query::parse(&text).map_err(IntoFFI::_into_ffi).map(IntoFFI::_into_ffi)
    }
} }

//================================================
//
// record query-error
//
//================================================

impl IntoFFI<ffi::QueryError> for rust::QueryError {
    #[inline]
    fn _into_ffi(self) -> ffi::QueryError {
        ffi::QueryError {
            message: self.message,
            line: self.line.try_into().unwrap(),
            column: self.column.try_into().unwrap(),
        }
    }
}

//================================================
//
// resource query-match-iterator
//
//================================================

define_refcell_wrapper! { QueryMatchIterator {
    fn next(&self) -> Option<ffi::QueryMatch> {
        self._borrow_mut_ffi().next().map(IntoFFI::_into_ffi)
    }
} }

//================================================
//
// record query-match
//
//================================================

impl IntoFFI<ffi::QueryMatch> for rust::QueryMatch {
    #[inline]
    fn _into_ffi(self) -> ffi::QueryMatch {
        ffi::QueryMatch {
            query_number: self.query_number.try_into().unwrap(),
            captures: self
                .captures
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(IntoFFI::_into_ffi).collect()))
                .collect(),
        }
    }
}

//================================================
//
// record text-index
//
//================================================

impl IntoFFI<ffi::TextIndex> for &rust::TextIndex {
    #[inline]
    fn _into_ffi(self) -> ffi::TextIndex {
        ffi::TextIndex {
            utf8: self.utf8.try_into().unwrap(),
            utf16: self.utf16.try_into().unwrap(),
            line: self.line.try_into().unwrap(),
            column: self.column.try_into().unwrap(),
        }
    }
}

impl FromFFI<rust::TextIndex> for &ffi::TextIndex {
    #[inline]
    fn _from_ffi(self) -> rust::TextIndex {
        rust::TextIndex {
            utf8: self.utf8 as usize,
            utf16: self.utf16 as usize,
            line: self.line as usize,
            column: self.column as usize,
        }
    }
}

//================================================
//
// record text-range
//
//================================================

impl IntoFFI<ffi::TextRange> for &rust::TextRange {
    #[inline]
    fn _into_ffi(self) -> ffi::TextRange {
        ffi::TextRange {
            start: self.start._into_ffi(),
            end: self.end._into_ffi(),
        }
    }
}

impl FromFFI<rust::TextRange> for &ffi::TextRange {
    #[inline]
    fn _from_ffi(self) -> rust::TextRange {
        rust::TextRange {
            start: self.start._from_ffi(),
            end: self.end._from_ffi(),
        }
    }
}
