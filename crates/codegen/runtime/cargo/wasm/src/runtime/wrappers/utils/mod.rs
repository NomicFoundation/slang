use semver::Version;

use crate::wasm_crate::utils::define_wrapper;

mod ffi {
    pub use crate::wasm_crate::bindgen::exports::nomic_foundation::slang::utils::{
        Guest, GuestLanguageFacts, LanguageFacts, LanguageFactsBorrow,
    };
}

mod rust {
    pub use crate::rust_crate::utils::LanguageFacts;
}

impl ffi::Guest for crate::wasm_crate::World {
    type LanguageFacts = LanguageFactsWrapper;
}

//================================================
//
// resource language-facts
//
//================================================

define_wrapper! { LanguageFacts {
    fn all_versions() -> Vec<String> {
        rust::LanguageFacts::ALL_VERSIONS
            .iter()
            .map(Version::to_string)
            .collect()
    }

    fn earliest_version() -> String {
        rust::LanguageFacts::EARLIEST_VERSION.to_string()
    }

    fn latest_version() -> String {
        rust::LanguageFacts::LATEST_VERSION.to_string()
    }

    fn infer_language_versions(src: String) -> Vec<String> {
        rust::LanguageFacts::infer_language_versions(&src).map(|v| v.to_string()).collect()
    }
} }
