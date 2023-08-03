use anyhow::Result;
use codegen_schema::types::{LanguageDefinition, LanguageDefinitionRef};

pub trait SolidityLanguageExtensions {
    fn load_solidity() -> Result<LanguageDefinitionRef>;
}

// Set by the build script.
static LANGUAGE_DEFINITION_BIN: &'static str = env!("SLANG_SOLIDITY_LANGUAGE_DEFINITION_BIN");

impl SolidityLanguageExtensions for LanguageDefinition {
    /// We compile the language definition only once, and then expose it here to all downstream crates.
    /// This ensures that:
    /// 1. Expensive parsing and validation is done only once.
    /// 2. Errors are reported only once, instead of repeating for every crate.
    fn load_solidity() -> Result<LanguageDefinitionRef> {
        let buffer = std::fs::read(LANGUAGE_DEFINITION_BIN)?;
        let language: LanguageDefinition = bson::from_slice(&buffer)?;

        return Ok(LanguageDefinitionRef::new(language));
    }
}
