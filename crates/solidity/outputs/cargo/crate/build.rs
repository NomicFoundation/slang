//! This build script is only used for local development.
//! It is removed when publishing to crates.io.

use anyhow::Result;
use codegen_runtime_generator::RuntimeGenerator;
use infra_utils::cargo::CargoWorkspace;
use solidity_language::{render_built_ins, SolidityDefinition};

fn main() -> Result<()> {
    let language = SolidityDefinition::create();

    let input_dir =
        CargoWorkspace::locate_source_crate("codegen_runtime_cargo_crate")?.join("src/runtime");

    let output_dir = CargoWorkspace::locate_source_crate("slang_solidity")?.join("src/generated");

    let mut fs = RuntimeGenerator::generate_product(&language, &input_dir, &output_dir)?;
    let built_ins_output_dir = output_dir.join("bindings/generated/built_ins");
    codegen_runtime_generator::render_built_ins(
        &mut fs,
        &language,
        &built_ins_output_dir,
        render_built_ins,
    )
}
