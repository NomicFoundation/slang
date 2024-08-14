// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::wit::utils::{define_refcell_wrapper, FromFFI, IntoFFI};

mod ffi {
    pub use crate::wit::interface::exports::nomic_foundation::slang::cursor::{
        Cursor, CursorBorrow, EdgeLabel, Guest, GuestCursor, Node, NonterminalKind,
        NonterminalNode, TerminalKind, TextIndex, TextRange,
    };
}

mod rust {
    pub use crate::cursor::Cursor;
}

impl ffi::Guest for crate::wit::World {
    type Cursor = CursorWrapper;
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

    fn label(&self) -> Option<ffi::EdgeLabel> {
        self._borrow_ffi().label().map(IntoFFI::_into_ffi)
    }

    fn text_offset(&self) -> ffi::TextIndex {
        self._borrow_ffi().text_offset()._into_ffi()
    }

    fn text_range(&self) -> ffi::TextRange {
        self._borrow_ffi().text_range()._into_ffi()
    }

    #[allow(clippy::cast_possible_truncation)]
    fn depth(&self) -> u32 {
        self._borrow_ffi().depth() as u32
    }

    fn ancestors(&self) -> Vec<ffi::NonterminalNode> {
        self._borrow_ffi().ancestors().map(|x|x._into_ffi()).collect()
    }

    fn go_to_next(&self) -> bool {
        self._borrow_mut_ffi().go_to_next()
    }

    fn go_to_next_non_descendent(&self) -> bool {
        self._borrow_mut_ffi().go_to_next_non_descendent()
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

    // TODO: re-enable once we solve circular references:
    // fn query(&self, queries: Vec<ffi::QueryBorrow<'_>>) -> ffi::QueryMatchIterator {
    //     let queries:Vec<rust::Query> = queries.into_iter().map(|q|{
    //         q._borrow_ffi().clone()
    //     }).collect();

    //     self._borrow_ffi().clone().query(queries)._into_ffi()
    // }
} }
