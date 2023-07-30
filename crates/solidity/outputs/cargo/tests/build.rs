use anyhow::Result;
use codegen_schema::types::LanguageDefinition;
use codegen_testing::TestingGeneratorExtensions;
use infra_utils::cargo::CargoWorkspace;
use solidity_language::SolidityLanguageExtensions;

fn main() -> Result<()> {
    let language = LanguageDefinition::load_solidity()?;

    language.generate_cst_output_tests(
        &CargoWorkspace::locate_source_crate("solidity_testing_snapshots")?.join("cst_output"),
        &CargoWorkspace::locate_source_crate("solidity_cargo_tests")?
            .join("src/cst_output/generated"),
    )?;

    return Ok(());
}
