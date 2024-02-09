use anyhow::Result;
use codegen_spec::Spec;
use infra_utils::cargo::CargoWorkspace;
use solidity_language::SolidityDefinition;

fn main() -> Result<()> {
    let language = SolidityDefinition::create();

    let output_dir = CargoWorkspace::locate_source_crate("solidity_spec")?.join("generated");

    Spec::generate(language, &output_dir)
}
