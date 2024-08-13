// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::wit::enum_to_enum;

mod ffi {
    pub use crate::wit::bindings::exports::nomic_foundation::slang::diagnostic::Severity;
}

mod rust {
    pub use crate::diagnostic::Severity;
}

//================================================
//
// enum severity
//
//================================================

enum_to_enum!(Severity);
