use std::path::Path;

use anyhow::Result;
use cargo_emit::{rerun_if_changed, rustc_env};
use codegen_schema::types::{LanguageDefinition, LanguageDefinitionRef};
use infra_utils::{cargo::CargoWorkspace, paths::PathExtensions};

fn main() -> Result<()> {
    let crate_dir = CargoWorkspace::locate_source_crate("solidity_language")?;

    LanguageDefinition::generate_json_schema(&crate_dir.join("generated"))?;

    let language = compile_language(&crate_dir)?;
    let bin_file_path = crate_dir.join("target/language-definition.bin");

    write_binary(&language, &bin_file_path)?;

    rustc_env!(
        "COMPILED_SOLIDITY_LANGUAGE_DEFINITION_BIN",
        "{}",
        bin_file_path.unwrap_str()
    );
    rerun_if_changed!(bin_file_path.unwrap_str());

    return Ok(());
}

fn compile_language(crate_dir: &Path) -> Result<LanguageDefinitionRef> {
    let definition_dir = crate_dir.join("definition");

    return LanguageDefinition::compile(definition_dir);
}

fn write_binary(language: &LanguageDefinitionRef, bin_file_path: &Path) -> Result<()> {
    let mut buffer = Vec::new();
    bson::to_document(&language)?.to_writer(&mut buffer)?;

    if bin_file_path.exists() {
        let existing_buffer = std::fs::read(bin_file_path)?;

        if buffer == existing_buffer {
            // Don't overwrite, in order not to trigger a rebuild by Cargo:
            return Ok(());
        }
    }

    std::fs::create_dir_all(bin_file_path.unwrap_parent())?;

    std::fs::write(bin_file_path, &buffer)?;

    return Ok(());
}
