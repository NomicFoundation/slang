mod common;
mod read_write;
mod write_only;

pub use read_write::*;
pub use write_only::*;

use std::path::PathBuf;

use anyhow::Result;

pub struct Codegen;

impl Codegen {
    pub fn write_only() -> Result<CodegenWriteOnly> {
        return CodegenWriteOnly::new();
    }

    pub fn read_write(input_dir: impl Into<PathBuf>) -> Result<CodegenReadWrite> {
        return CodegenReadWrite::new(input_dir);
    }
}
