use crate::wit::utils::{
    define_rc_wrapper, define_refcell_wrapper, define_wrapper, enum_to_enum, FromFFI, IntoFFI,
};

mod ffi {
    pub use crate::wit::interface::exports::nomic_foundation::slang::cst::{
        Cursor, CursorBorrow, EdgeLabel, Guest, GuestCursor, GuestNonterminalNode, GuestQuery,
        GuestQueryMatchIterator, GuestTerminalNode, Node, NonterminalKind, NonterminalNode,
        NonterminalNodeBorrow, Query, QueryBorrow, QueryError, QueryMatch, QueryMatchIterator,
        QueryMatchIteratorBorrow, TerminalKind, TerminalNode, TerminalNodeBorrow, TextIndex,
        TextRange,
    };
}

mod rust {
    pub use crate::cst::{
        Cursor, EdgeLabel, Node, NonterminalKind, NonterminalNode, Query, QueryError, QueryMatch,
        QueryMatchIterator, TerminalKind, TerminalNode, TextIndex, TextRange,
    };
}

impl ffi::Guest for crate::wit::World {
    type NonterminalNode = NonterminalNodeWrapper;
    type TerminalNode = TerminalNodeWrapper;

    type Cursor = CursorWrapper;

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
// enum edge-label
//
//================================================

enum_to_enum!(EdgeLabel);

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
        self._borrow_ffi().children.iter().map(|edge| edge.node.clone()._into_ffi()).collect()
    }

    fn create_cursor(&self, text_offset: ffi::TextIndex) -> ffi::Cursor {
        std::rc::Rc::clone(self._borrow_ffi()).cursor_with_offset(text_offset._from_ffi())._into_ffi()
    }

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

    fn query(&self, queries: Vec<ffi::QueryBorrow<'_>>) -> ffi::QueryMatchIterator {
        let queries:Vec<rust::Query> = queries.into_iter().map(|q|{
            q._borrow_ffi().clone()
        }).collect();

        self._borrow_ffi().clone().query(queries)._into_ffi()
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
        #[allow(clippy::cast_possible_truncation)]
        ffi::QueryError {
            message: self.message,
            row: self.row as u32,
            column: self.column as u32,
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
            #[allow(clippy::cast_possible_truncation)]
            query_number: self.query_number as u32,
            captures: self
                .captures
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(|c| c._into_ffi()).collect()))
                .collect(),
        }
    }
}

//================================================
//
// record text-index
//
//================================================

impl IntoFFI<ffi::TextIndex> for rust::TextIndex {
    #[inline]
    fn _into_ffi(self) -> ffi::TextIndex {
        #[allow(clippy::cast_possible_truncation)]
        ffi::TextIndex {
            utf8: self.utf8 as u32,
            utf16: self.utf16 as u32,
            line: self.line as u32,
            column: self.column as u32,
        }
    }
}

impl IntoFFI<ffi::TextIndex> for &rust::TextIndex {
    #[inline]
    fn _into_ffi(self) -> ffi::TextIndex {
        (*self)._into_ffi()
    }
}

impl FromFFI<rust::TextIndex> for ffi::TextIndex {
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

impl IntoFFI<ffi::TextRange> for rust::TextRange {
    #[inline]
    fn _into_ffi(self) -> ffi::TextRange {
        ffi::TextRange {
            start: self.start._into_ffi(),
            end: self.end._into_ffi(),
        }
    }
}

impl IntoFFI<ffi::TextRange> for &rust::TextRange {
    #[inline]
    fn _into_ffi(self) -> ffi::TextRange {
        ffi::TextRange {
            start: self.start._into_ffi(),
            end: self.end._into_ffi(),
        }
    }
}

impl FromFFI<rust::TextRange> for ffi::TextRange {
    #[inline]
    fn _from_ffi(self) -> rust::TextRange {
        rust::TextRange {
            start: self.start._from_ffi(),
            end: self.end._from_ffi(),
        }
    }
}
