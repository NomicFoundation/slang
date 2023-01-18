use std::path::PathBuf;

use anyhow::Result;
use codegen_schema::types::grammar::Grammar;
use codegen_utils::context::CodegenContext;

fn main() -> Result<()> {
    return CodegenContext::with_context(|codegen| {
        let manifest_path = codegen
            .repo_root
            .join("crates/solidity/inputs/schema/grammar/manifest.yml");

        let grammar = Grammar::compile(codegen, manifest_path)?;

        let mut buffer = Vec::new();
        bson::to_document(&grammar)?.to_writer(&mut buffer)?;

        let output_path = PathBuf::from(std::env::var("OUT_DIR")?).join("generated/grammar.bin");

        std::fs::create_dir_all(output_path.parent().unwrap())?;
        std::fs::write(&output_path, &buffer)?;

        println!(
            "cargo:rustc-env=SLANG_SOLIDITY_INPUT_SCHEMA_BIN={}",
            output_path.to_str().unwrap()
        );

        return Ok(());
    });
}
