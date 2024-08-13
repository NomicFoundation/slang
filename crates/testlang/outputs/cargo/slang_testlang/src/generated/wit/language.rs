// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::wit::{define_wrapper, FromFFI, IntoFFI};

mod ffi {
    pub use crate::wit::bindings::exports::nomic_foundation::slang::language::{
        GuestLanguage, Language, LanguageBorrow, NonterminalKind,
    };
    pub use crate::wit::bindings::exports::nomic_foundation::slang::parse_output::ParseOutput;
}

mod rust {
    pub use crate::language::Language;
}

//================================================
//
// resource language
//
//================================================

define_wrapper! { Language {
    fn new(version: String) -> Result<ffi::Language, String> {
        semver::Version::parse(&version)
            .map_err(|_| format!("Invalid version: {version}"))
            .and_then(|version| rust::Language::new(version).map_err(|e| e.to_string()))
            .map(IntoFFI::_into_ffi)
    }

    fn version(&self) -> String {
        self._borrow_ffi().version.to_string()
    }

    fn supported_versions() -> Vec<String> {
        rust::Language::SUPPORTED_VERSIONS
            .iter()
            .map(|v| v.to_string())
            .collect()
    }

    fn parse(&self, kind: ffi::NonterminalKind, input: String) -> ffi::ParseOutput {
        self._borrow_ffi().parse(kind._from_ffi(), &input)._into_ffi()
    }
} }
