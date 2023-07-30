use std::path::PathBuf;

use anyhow::Result;
use cargo_emit::rerun_if_changed;
use codegen_schema::types::{LanguageDefinition, LanguageDefinitionRef};
use infra_utils::paths::PathExtensions;

pub trait SolidityLanguageExtensions {
    fn load_solidity() -> Result<LanguageDefinitionRef>;
}

impl SolidityLanguageExtensions for LanguageDefinition {
    /// We compile the language definition only once, and then expose it here to all downstream crates.
    /// This ensures that:
    /// 1. Expensive parsing and validation is done only once.
    /// 2. Errors are reported only once, instead of repeating for every crate.
    fn load_solidity() -> Result<LanguageDefinitionRef> {
        let bin_file_path = PathBuf::from(env!("COMPILED_SOLIDITY_LANGUAGE_DEFINITION_BIN"));

        rerun_if_changed!(bin_file_path.unwrap_str());

        let buffer = std::fs::read(&bin_file_path)?;
        let language: LanguageDefinition = bson::from_slice(&buffer)?;

        return Ok(LanguageDefinitionRef::new(language));
    }
}
