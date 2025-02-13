use anyhow::Result;
use codegen_runtime_generator::RuntimeGenerator;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::ldw;
use testlang_language::TestlangDefinition;

fn main() -> Result<()> {
    let language = TestlangDefinition::create();

    let input_dir =
        CargoWorkspace::locate_source_crate("codegen_runtime_cargo_crate")?.join("src/runtime");

    let crate_path = CargoWorkspace::locate_source_crate("slang_testlang")?;
    let output_dir = crate_path.join("src/generated");

    let mut fs = RuntimeGenerator::generate_product(&language, &input_dir, &output_dir)?;

    ldw::process_models(
        &mut fs,
        crate_path.join("src/generated/ldw-models"),
        crate_path.join("src/generated/ldw"),
    )
}
