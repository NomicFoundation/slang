use crate::wit::utils::{FromFFI, IntoFFI};

mod ffi {
    pub use crate::wit::interface::exports::nomic_foundation::slang::text_index::{
        TextIndex, TextRange,
    };
}

mod rust {
    pub use crate::text_index::{TextIndex, TextRange};
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
