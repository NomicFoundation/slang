use std::path::PathBuf;

use anyhow::Result;
use codegen_schema::types::{LanguageDefinition, LanguageDefinitionRef};

pub trait SolidityLanguageExtensions {
    fn load_solidity() -> Result<LanguageDefinitionRef>;
}

impl SolidityLanguageExtensions for LanguageDefinition {
    /// We compile the language definition only once, and then expose it here to all downstream crates.
    /// This ensures that:
    /// 1. Expensive parsing and validation is done only once.
    /// 2. Errors are reported only once, instead of repeating for every crate.
    fn load_solidity() -> Result<LanguageDefinitionRef> {
        let bin_path = env!("SLANG_SOLIDITY_LANGUAGE_DEFINITION_BIN");

        println!("cargo:rerun-if-changed={bin_path}");

        let bin_path = PathBuf::from(bin_path);
        let buffer = std::fs::read(&bin_path)?;
        let language: LanguageDefinition = bson::from_slice(&buffer)?;

        return Ok(LanguageDefinitionRef::new(language));
    }
}
