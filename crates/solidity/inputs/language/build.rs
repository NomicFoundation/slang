use std::path::PathBuf;

use anyhow::Result;
use codegen_schema::types::LanguageDefinition;
use codegen_utils::context::CodegenContext;

fn main() -> Result<()> {
    return CodegenContext::with_context(|codegen| {
        let crate_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?);

        LanguageDefinition::generate_json_schema(codegen, crate_dir.join("generated"))?;

        compile_language(codegen, crate_dir.join("definition"))?;

        return Ok(());
    });
}

fn compile_language(codegen: &mut CodegenContext, definition_dir: PathBuf) -> Result<()> {
    // Rebuild if input files are added/removed
    codegen.track_input_dir(&definition_dir);

    let language = LanguageDefinition::compile(codegen, definition_dir)?;

    let mut buffer = Vec::new();
    bson::to_document(&language)?.to_writer(&mut buffer)?;

    let output_path =
        PathBuf::from(std::env::var("OUT_DIR")?).join("generated/language-definition.bin");
    std::fs::create_dir_all(output_path.parent().unwrap())?;
    std::fs::write(&output_path, &buffer)?;

    println!(
        "cargo:rustc-env=SLANG_SOLIDITY_LANGUAGE_DEFINITION_BIN={path}",
        path = output_path.to_str().unwrap()
    );

    return Ok(());
}
