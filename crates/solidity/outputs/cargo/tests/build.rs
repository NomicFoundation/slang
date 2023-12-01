use anyhow::Result;
use codegen_testing::TestingGeneratorExtensions;
use infra_utils::cargo::CargoWorkspace;

fn main() -> Result<()> {
    solidity_language::LANGUAGE_DEF.with(|language| {
        language.generate_cst_output_tests(
            &CargoWorkspace::locate_source_crate("solidity_testing_snapshots")?.join("cst_output"),
            &CargoWorkspace::locate_source_crate("solidity_cargo_tests")?
                .join("src/cst_output/generated"),
        )
    })?;

    return Ok(());
}
