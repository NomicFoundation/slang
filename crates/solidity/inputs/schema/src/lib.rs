use std::path::PathBuf;

use anyhow::Result;
use codegen_schema::types::schema::Schema;

pub trait SoliditySchemaExtensions {
    fn load_solidity() -> Result<Schema>;
}

impl SoliditySchemaExtensions for Schema {
    /// We compile the schema only once, and then expose it here to all downstream crates.
    /// This ensures that:
    /// 1. Expensive parsing and validation is done only once.
    /// 2. Errors are reported only once, instead of repeating for every crate.
    fn load_solidity() -> Result<Schema> {
        let bin_path = PathBuf::from(env!("SLANG_SOLIDITY_INPUT_SCHEMA_BIN"));
        let buffer = std::fs::read(&bin_path)?;
        let schema: Schema = bson::from_slice(&buffer)?;

        return Ok(schema);
    }
}
