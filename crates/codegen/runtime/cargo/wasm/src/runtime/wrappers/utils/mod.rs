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
    fn supported_versions() -> Vec<String> {
        rust::LanguageFacts::SUPPORTED_VERSIONS
            .iter()
            .map(|v| v.to_string())
            .collect()
    }
} }
