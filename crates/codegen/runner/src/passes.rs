use anyhow::Result;
use codegen_language_definition::model::Language;
use codegen_runtime_generator::ir::{IrModel, ModelWithBuilder, ModelWithTransformer};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::{CodegenFileSystem, CodegenRuntime};
use semver::Version;

pub fn generate_passes(
    fs: &mut CodegenFileSystem,
    language: &Language,
    input_crate: &str,
    output_crate: &str,
) -> Result<()> {
    let ir_input_dir = CargoWorkspace::locate_source_crate(input_crate)?.join("src/ir");
    let ir_output_dir = CargoWorkspace::locate_source_crate(output_crate)?.join("src/backend");

    // L0: CST:
    let minimum_version = Version::parse("0.8.0").unwrap();
    let cst_model = IrModel::from_language("cst", language, minimum_version);

    // L1: typed CST:
    let l1_typed_cst_model = build_l1_typed_cst_model(&cst_model);
    let l1_typed_cst_output_dir = ir_output_dir.join(&l1_typed_cst_model.target.name);
    CodegenRuntime::render_product(
        fs,
        &ir_input_dir,
        &l1_typed_cst_output_dir,
        &l1_typed_cst_model,
    )?;

    // L2: flat contract specifiers:
    let l2_flat_contracts_model = build_l2_flat_contracts_model(&l1_typed_cst_model.target);
    let l2_flat_contracts_output_dir = ir_output_dir.join(&l2_flat_contracts_model.target.name);
    CodegenRuntime::render_product(
        fs,
        &ir_input_dir,
        &l2_flat_contracts_output_dir,
        &l2_flat_contracts_model,
    )?;

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
