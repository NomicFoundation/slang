use crate::wit::{define_wrapper, IntoFFI};

mod ffi {
    pub use crate::wit::bindings::exports::nomic_foundation::slang::cst::Node;
    pub use crate::wit::bindings::exports::nomic_foundation::slang::cursor::Cursor;
    pub use crate::wit::bindings::exports::nomic_foundation::slang::parse_error::ParseError;
    pub use crate::wit::bindings::exports::nomic_foundation::slang::parse_output::{
        GuestParseOutput, ParseOutput, ParseOutputBorrow,
    };
}

mod rust {
    pub use crate::parse_output::ParseOutput;
}

//================================================
//
// resource parse-output
//
//================================================

define_wrapper! { ParseOutput {
  fn tree(&self) -> ffi::Node {
      self._borrow_ffi().tree()._into_ffi()
  }

  fn errors(&self) -> Vec<ffi::ParseError> {
      self._borrow_ffi().errors().iter().map(|e| e.clone()._into_ffi()).collect()
  }

  fn is_valid(&self) -> bool {
      self._borrow_ffi().is_valid()
  }

  fn create_tree_cursor(&self) -> ffi::Cursor {
      self._borrow_ffi().create_tree_cursor()._into_ffi()
  }
} }
