//! This crate is responsible for compiling the language definition and exposing it to downstream crates.
//!
//! While it's possible to directly compile the language, we do it here once, to ensure that:
//! 1. Expensive parsing and validation is done only once.
//! 2. Errors are reported only once, instead of repeating for every crate.
//!
//! Call the [`SolidityLanguageExtensions::load_solidity`] method to load the precompiled language definition.

mod definition;

use anyhow::Result;
use codegen_schema::types::{LanguageDefinition, LanguageDefinitionRef};
pub use definition::SolidityDefinition;

pub trait SolidityLanguageExtensions {
    /// Loads the precompiled Solidity language definition.
    fn load_solidity() -> Result<LanguageDefinitionRef>;
}

// Set by the build script.
static LANGUAGE_DEFINITION_BIN: &str = env!("COMPILED_SOLIDITY_LANGUAGE_DEFINITION_BIN");

impl SolidityLanguageExtensions for LanguageDefinition {
    fn load_solidity() -> Result<LanguageDefinitionRef> {
        let buffer = std::fs::read(LANGUAGE_DEFINITION_BIN)?;
        let language: LanguageDefinition = bson::from_slice(&buffer)?;

        Ok(LanguageDefinitionRef::new(language))
    }
}
