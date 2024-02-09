use anyhow::Result;
use codegen_parser_generator::TypeScriptGenerator;
use infra_utils::cargo::CargoWorkspace;
use solidity_language::SolidityDefinition;

fn main() -> Result<()> {
    let language = SolidityDefinition::create();

    TypeScriptGenerator::generate(
        &language,
        &CargoWorkspace::locate_source_crate("solidity_npm_package")?,
    )
}
