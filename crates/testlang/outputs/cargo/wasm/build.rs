use anyhow::Result;
use codegen_runtime_generator::RuntimeGenerator;
use infra_utils::cargo::CargoWorkspace;
use testlang_language::TestlangDefinition;

fn main() -> Result<()> {
    let language = TestlangDefinition::create();

    let input_dir =
        CargoWorkspace::locate_source_crate("codegen_runtime_cargo_wasm")?.join("src/runtime");

    let output_dir =
        CargoWorkspace::locate_source_crate("testlang_cargo_wasm")?.join("src/generated");

    RuntimeGenerator::generate_product(&language, &input_dir, &output_dir)
}
