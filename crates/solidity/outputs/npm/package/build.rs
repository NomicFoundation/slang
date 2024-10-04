use anyhow::Result;
use codegen_runtime_generator::OutputLanguage;
use infra_utils::cargo::CargoWorkspace;
use solidity_language::SolidityDefinition;

fn main() -> Result<()> {
    let language = SolidityDefinition::create();

    let output_dir =
        CargoWorkspace::locate_source_crate("solidity_npm_package")?.join("src/generated");

    OutputLanguage::Npm.generate_runtime(&language, &output_dir, |_| Ok(()))
}
