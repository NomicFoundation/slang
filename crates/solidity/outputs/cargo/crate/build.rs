//! This build script is only used for local development.
//! It is removed when publishing to crates.io.

use anyhow::Result;
use codegen_runtime_generator::ir::{IrModel, ModelWithBuilder, ModelWithTransformer};
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

    // L0: CST
    let minimum_version = Version::parse("0.8.0").unwrap();
    let cst_model = IrModel::from_language("cst", &language, minimum_version);

    // L1: typed CST
    let runtime = CodegenRuntime::new(&ir_input_dir)?;
    let l1_typed_cst_model = build_l1_typed_cst_model(&cst_model);
    let l1_typed_cst_output_dir = ir_output_dir.join(&l1_typed_cst_model.target.name);
    _ = runtime.render_product(&l1_typed_cst_model, &l1_typed_cst_output_dir)?;

    // L2: flat contract specifiers
    let runtime = CodegenRuntime::new(&ir_input_dir)?;
    let l2_flat_contracts_model = build_l2_flat_contracts_model(&l1_typed_cst_model.target);
    let l2_flat_contracts_output_dir = ir_output_dir.join(&l2_flat_contracts_model.target.name);
    _ = runtime.render_product(&l2_flat_contracts_model, &l2_flat_contracts_output_dir)?;

    Ok(())
}

fn build_l1_typed_cst_model(cst_model: &IrModel) -> ModelWithBuilder {
    let mut l1_typed_cst_model = IrModel::from_model("l1_typed_cst", cst_model);

    // remove fields from sequences that contain redundant terminal nodes
    for (_, sequence) in &mut l1_typed_cst_model.sequences {
        sequence.fields.retain(|field| {
            field.is_optional
                || !field.is_terminal
                || !l1_typed_cst_model.unique_terminals.contains(&field.r#type)
        });
    }

    ModelWithBuilder::new(cst_model, l1_typed_cst_model)
}

fn build_l2_flat_contracts_model(typed_cst_model: &IrModel) -> ModelWithTransformer {
    let mut l2_flat_contracts_model = IrModel::from_model("l2_flat_contracts", typed_cst_model);

    // L2 is for now only a proof of concept for rendering transfomation code
    // from previous trees. Therefore, the following modifications are (a
    // non-exhaustive list of) samples of what can be done.

    // Flatten contract specifiers and bring the inherited types and storage
    // layout to the contract definition itself.
    l2_flat_contracts_model.remove_type("ContractSpecifiers");
    l2_flat_contracts_model.add_sequence_field(
        "ContractDefinition",
        "inheritance_types",
        "InheritanceTypes",
        false,
    );
    l2_flat_contracts_model.add_sequence_field(
        "ContractDefinition",
        "storage_layout",
        "StorageLayoutSpecifier",
        true,
    );

    ModelWithTransformer::new(typed_cst_model, l2_flat_contracts_model)
}
