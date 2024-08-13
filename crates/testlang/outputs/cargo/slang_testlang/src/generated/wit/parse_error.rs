// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::diagnostic::Diagnostic;
use crate::wit::{define_wrapper, IntoFFI};

mod ffi {
    pub use crate::wit::bindings::exports::nomic_foundation::slang::diagnostic::Severity;
    pub use crate::wit::bindings::exports::nomic_foundation::slang::parse_error::GuestParseError;
    pub use crate::wit::bindings::exports::nomic_foundation::slang::parse_output::{
        ParseError, ParseErrorBorrow,
    };
    pub use crate::wit::bindings::exports::nomic_foundation::slang::text_index::TextRange;
}

mod rust {
    pub use crate::parse_error::ParseError;
}

//================================================
//
// resource parse-error
//
//================================================

define_wrapper! { ParseError {
  fn severity(&self) -> ffi::Severity {
      self._borrow_ffi().severity()._into_ffi()
  }

  fn text_range(&self) -> ffi::TextRange {
      self._borrow_ffi().text_range()._into_ffi()
  }

  fn message(&self) -> String {
      self._borrow_ffi().message()
  }
} }
