//! This build script is only used for local development.
//! It is removed when publishing to crates.io.

use anyhow::Result;
use codegen_parser_generator::RustGenerator;
use infra_utils::cargo::CargoWorkspace;
use solidity_language::SolidityDefinition;

fn main() -> Result<()> {
    let language = SolidityDefinition::create();

    RustGenerator::generate(
        &language,
        &CargoWorkspace::locate_source_crate("slang_solidity")?.join("src/generated"),
    )
}
