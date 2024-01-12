use anyhow::Result;
use codegen_parser_generator::{AstModel, TypeScriptGenerator};
use infra_utils::cargo::CargoWorkspace;
use solidity_language::SolidityDefinition;

fn main() -> Result<()> {
    let language = SolidityDefinition::create();
    let ast_model = AstModel::create(&language);

    TypeScriptGenerator::generate(
        &ast_model,
        &CargoWorkspace::locate_source_crate("solidity_npm_package")?,
    )
}
