use anyhow::Result;
use codegen_schema::types::LanguageDefinition;
use codegen_spec::SpecGeneratorExtensions;
use infra_utils::cargo::CargoWorkspace;
use solidity_language::SolidityLanguageExtensions;

fn main() -> Result<()> {
    let language = LanguageDefinition::load_solidity()?;
    let output_dir = CargoWorkspace::locate_source_crate("solidity_spec")?.join("generated");

    language.generate_spec(&output_dir)
}
