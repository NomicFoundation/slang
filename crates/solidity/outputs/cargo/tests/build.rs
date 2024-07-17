use anyhow::Result;
use codegen_testing::TestingGeneratorExtensions;
use infra_utils::cargo::CargoWorkspace;
use solidity_language::SolidityDefinition;

fn main() -> Result<()> {
    let lang_def = SolidityDefinition::create();
    let snapshots_crate = CargoWorkspace::locate_source_crate("solidity_testing_snapshots")?;
    let tests_crate = CargoWorkspace::locate_source_crate("solidity_cargo_tests")?;

    lang_def.generate_bindings_assertions_tests(
        &snapshots_crate.join("bindings_assertions"),
        &tests_crate.join("src/bindings_assertions/generated"),
    )?;

    lang_def.generate_bindings_output_tests(
        &snapshots_crate.join("bindings_output"),
        &tests_crate.join("src/bindings_output/generated"),
    )?;

    lang_def.generate_cst_output_tests(
        &snapshots_crate.join("cst_output"),
        &tests_crate.join("src/cst_output/generated"),
    )?;

    Ok(())
}
