//! This build script is only used for local development.
//! It is removed when publishing to crates.io.

use anyhow::Result;
use codegen_runtime_generator::{AstModel, RuntimeGenerator};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenRuntime;
use serde::Serialize;
use solidity_language::{render_built_ins, SolidityDefinition};

fn main() -> Result<()> {
    let language = SolidityDefinition::create();

    let input_dir =
        CargoWorkspace::locate_source_crate("codegen_runtime_cargo_crate")?.join("src/runtime");

    let output_dir = CargoWorkspace::locate_source_crate("slang_solidity")?.join("src/generated");

    let mut fs = RuntimeGenerator::generate_product(&language, &input_dir, &output_dir)?;

    let language_input_dir =
        CargoWorkspace::locate_source_crate("codegen_runtime_cargo_crate")?.join("src/ir");
    let language_output_dir =
        CargoWorkspace::locate_source_crate("slang_solidity")?.join("src/backend/generated/ast");
    let language_model = ModelWrapper {
        target: AstModel::from_language(&language),
    };
    let runtime = CodegenRuntime::new(&language_input_dir)?;
    runtime.render_product(&language_model, &language_output_dir)?;

    let built_ins_output_dir = output_dir.join("extensions");
    let built_ins_path = built_ins_output_dir.join("built_ins.rs");
    let contents = render_built_ins(&language)?;
    fs.write_file(built_ins_path, contents)
}

#[derive(Serialize)]
struct ModelWrapper {
    target: AstModel,
}
