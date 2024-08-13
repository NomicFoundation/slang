// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::wit::{define_rc_wrapper, IntoFFI};

mod ffi {
    pub use crate::wit::bindings::exports::nomic_foundation::slang::cst::{
        GuestNonterminalNode, GuestTerminalNode, Node, NonterminalNode, NonterminalNodeBorrow,
        TerminalNode, TerminalNodeBorrow,
    };
    pub use crate::wit::bindings::exports::nomic_foundation::slang::kinds::{
        NonterminalKind, TerminalKind,
    };
    pub use crate::wit::bindings::exports::nomic_foundation::slang::text_index::TextIndex;
}

mod rust {
    pub use crate::cst::{Node, NonterminalNode, TerminalNode};
    pub use crate::text_index::TextIndex;
}

//================================================
//
// resource nonterminal-node
//
//================================================

define_rc_wrapper! { NonterminalNode {
    fn kind(&self) -> ffi::NonterminalKind {
        self._borrow_ffi().kind._into_ffi()
    }

    fn text_len(&self) -> ffi::TextIndex {
        self._borrow_ffi().text_len._into_ffi()
    }

    fn children(&self) -> Vec<ffi::Node> {
        todo!()
    }

    // TODO: re-enable once we solve circular references:
    // fn create_cursor(&self, text_offset: ffi::TextIndex) -> ffi::Cursor {
    //     std::rc::Rc::clone(self._borrow_ffi()).cursor_with_offset(text_offset._from_ffi())._into_ffi()
    // }

    fn unparse(&self) -> String {
        std::rc::Rc::clone(self._borrow_ffi()).unparse()
    }
} }

//================================================
//
// resource terminal-node
//
//================================================

define_rc_wrapper! { TerminalNode {
    fn kind(&self) -> ffi::TerminalKind {
        self._borrow_ffi().kind._into_ffi()
    }

    fn text(&self) -> String {
        self._borrow_ffi().text.clone()
    }

    fn text_len(&self) -> ffi::TextIndex {
        rust::TextIndex::from(&self._borrow_ffi().text)._into_ffi()
    }
} }

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
