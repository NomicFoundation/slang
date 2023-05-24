use std::path::PathBuf;

use anyhow::Result;
use codegen_schema::types::schema::Schema;
use codegen_utils::context::CodegenContext;

fn main() -> Result<()> {
    return CodegenContext::with_context(|codegen| {
        let schema_dir = codegen
            .repo_root
            .join("crates/solidity/inputs/schema/definition");

        // Rebuild if input files are added/removed
        codegen.track_input_dir(&schema_dir);

        let schema = Schema::compile(codegen, schema_dir)?;

        let mut buffer = Vec::new();
        bson::to_document(&schema)?.to_writer(&mut buffer)?;

        let output_path = PathBuf::from(std::env::var("OUT_DIR")?).join("generated/schema.bin");

        std::fs::create_dir_all(output_path.parent().unwrap())?;
        std::fs::write(&output_path, &buffer)?;

        println!(
            "cargo:rustc-env=SLANG_SOLIDITY_INPUT_SCHEMA_BIN={path}",
            path = output_path.to_str().unwrap()
        );

        return Ok(());
    });
}
