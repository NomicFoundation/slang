use anyhow::Result;
use codegen_runtime_generator::RuntimeGenerator;
use infra_utils::cargo::CargoWorkspace;
use solidity_language::SolidityDefinition;

fn main() -> Result<()> {
    let language = SolidityDefinition::create();

    let input_dir =
        CargoWorkspace::locate_source_crate("codegen_runtime_cargo_wasm")?.join("src/runtime");

    let output_dir =
        CargoWorkspace::locate_source_crate("solidity_cargo_wasm")?.join("src/generated");

    RuntimeGenerator::generate_product(&language, &input_dir, &output_dir).map(|_| ())
}
