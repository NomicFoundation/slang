use crate::wit::{define_refcell_wrapper, define_wrapper, IntoFFI};

mod ffi {
    pub use crate::wit::bindings::exports::nomic_foundation::slang::query::{
        GuestQuery, GuestQueryMatchIterator, Query, QueryBorrow, QueryError, QueryMatch,
        QueryMatchIterator, QueryMatchIteratorBorrow,
    };
}

mod rust {
    pub use crate::query::{Query, QueryError, QueryMatch, QueryMatchIterator};
}

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
