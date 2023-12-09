mod common;
mod read_write;
mod write_only;

use std::path::PathBuf;

use anyhow::Result;
pub use read_write::*;
pub use write_only::*;

pub struct Codegen;

impl Codegen {
    pub fn write_only() -> Result<CodegenWriteOnly> {
        CodegenWriteOnly::new()
    }

    pub fn read_write(input_dir: impl Into<PathBuf>) -> Result<CodegenReadWrite> {
        CodegenReadWrite::new(input_dir)
    }
}
