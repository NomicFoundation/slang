//! This build script is only used for local development.
//! It is removed when publishing to crates.io.

use anyhow::Result;
use codegen_runtime_generator::ir::{IrModel, ModelWrapper};
use codegen_runtime_generator::RuntimeGenerator;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenRuntime;
use semver::Version;
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

    let minimum_version = Version::parse("0.8.0").unwrap();

    let runtime = CodegenRuntime::new(&ir_input_dir)?;
    let cst_model = IrModel::from_language("cst", &language, minimum_version);
    let ast_model_wrapper = build_ast_model_wrapper(&cst_model);

    let ast_output_dir = ir_output_dir.join(&ast_model_wrapper.target.name);
    _ = runtime.render_product(&ast_model_wrapper, &ast_output_dir)?;

    let runtime = CodegenRuntime::new(&ir_input_dir)?;
    let l1_model_wrapper = build_l1_model_wrapper(&ast_model_wrapper);
    let l1_output_dir = ir_output_dir.join(&l1_model_wrapper.target.name);
    _ = runtime.render_product(&l1_model_wrapper, &l1_output_dir)?;

    Ok(())
}

fn build_ast_model_wrapper(cst_model: &IrModel) -> ModelWrapper {
    let mut ast_model = IrModel::from_model("ast", cst_model);

    // remove fields from sequences that contain redundant terminal nodes
    for (_, sequence) in &mut ast_model.sequences {
        sequence.fields.retain(|field| {
            field.is_optional
                || !field.is_terminal
                || !ast_model.unique_terminals.contains(&field.r#type)
        });
    }

    ModelWrapper::with_builder(cst_model, ast_model)
}

fn build_l1_model_wrapper(ast_model_wrapper: &ModelWrapper) -> ModelWrapper {
    let ast_model = &ast_model_wrapper.target;
    let mut l1_model = IrModel::from_model("l1", ast_model);

    // L1 is for now only a proof of concept for rendering transfomation code
    // from previous trees. Therefore, the following modifications are (a
    // non-exhaustive list of) samples of what can be done.

    // Flatten contract specifiers and bring the inherited types and storage
    // layout to the contract definition itself.
    l1_model.remove_type("ContractSpecifiers");
    l1_model.add_sequence_field(
        "ContractDefinition",
        "inheritance_types",
        "InheritanceTypes",
        false,
    );
    l1_model.add_sequence_field(
        "ContractDefinition",
        "storage_layout",
        "StorageLayoutSpecifier",
        true,
    );

    ModelWrapper::with_transformer(ast_model, l1_model)
}
