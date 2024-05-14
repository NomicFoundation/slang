//! This build script is only used for local development.
//! It is removed when publishing to crates.io.

use anyhow::Result;
use codegen_runtime_generator::OutputLanguage;
use infra_utils::cargo::CargoWorkspace;
use solidity_language::SolidityDefinition;

fn main() -> Result<()> {
    let language = SolidityDefinition::create();

    let output_dir = CargoWorkspace::locate_source_crate("slang_solidity")?.join("src/generated");

    OutputLanguage::Cargo.generate_runtime(&language, &output_dir)
}
