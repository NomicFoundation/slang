//! Various Slang language utilities.

#[path = "generated/language_facts.rs"]
mod language_facts;

#[path = "generated/versions.rs"]
pub(crate) mod versions;

pub use language_facts::LanguageFacts;
