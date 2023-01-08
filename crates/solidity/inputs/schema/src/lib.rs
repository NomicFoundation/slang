use std::path::PathBuf;

use anyhow::Result;
use codegen_schema::types::grammar::Grammar;

pub trait SolidityGrammarExtensions {
    fn load_solidity() -> Result<Grammar>;
}

impl SolidityGrammarExtensions for Grammar {
    fn load_solidity() -> Result<Grammar> {
        let bin_path = PathBuf::from(env!("SLANG_SOLIDITY_INPUT_SCHEMA_BIN"));
        let buffer = std::fs::read(&bin_path)?;
        let grammar: Grammar = bson::from_slice(&buffer)?;

        return Ok(grammar);
    }
}
