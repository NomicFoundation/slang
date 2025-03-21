//! This build script is only used for local development.
//! It is removed when publishing to crates.io.

use anyhow::Result;
use codegen_runtime_generator::ir::IrModel;
use codegen_runtime_generator::RuntimeGenerator;
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

    let built_ins_output_dir = output_dir.join("extensions");
    let built_ins_path = built_ins_output_dir.join("built_ins.rs");
    let contents = render_built_ins(&language)?;
    fs.write_file(built_ins_path, contents)?;

    // Intermediate Representation Languages for the backend

    let ir_input_dir =
        CargoWorkspace::locate_source_crate("codegen_runtime_cargo_crate")?.join("src/ir");
    let ir_output_dir = CargoWorkspace::locate_source_crate("slang_solidity")?.join("src/backend");

    let runtime = CodegenRuntime::new(&ir_input_dir)?;
    let ast_model = ModelWrapper {
        target: IrModel::from_language(&language),
    };
    let ast_output_dir = ir_output_dir.join("ast/generated");
    _ = runtime.render_product(&ast_model, &ast_output_dir)?;

    let runtime = CodegenRuntime::new(&ir_input_dir)?;
    let l1_model = ModelWrapper {
        target: ast_model.target.clone(),
    };
    let l1_output_dir = ir_output_dir.join("l1/generated");
    _ = runtime.render_product(&l1_model, &l1_output_dir)?;

    Ok(())
}

#[derive(Serialize)]
struct ModelWrapper {
    target: IrModel,
}
