use std::collections::BTreeMap;

use language_definition::model::Language;
use serde::Serialize;

use super::{IrModel, ModelWithBuilder, ModelWithTransformer};

#[derive(Serialize)]
#[serde(untagged)]
pub enum GenericModel {
    ModelWithBuilder(ModelWithBuilder),
    ModelWithTransformer(ModelWithTransformer),
}

pub fn build_languages(language: &Language) -> BTreeMap<String, GenericModel> {
    let mut languages = BTreeMap::new();

    // IR0: CST:
    let cst_model = IrModel::from_language(language);

    // IR1: structured AST:
    let ir1_structured_ast_model = build_ir1_structured_ast_model(&cst_model);

    // IR2: flat contract specifiers:
    let ir2_flat_contracts_model = build_ir2_flat_contracts_model(&ir1_structured_ast_model.target);

    languages.insert(
        "ir1_structured_ast".to_string(),
        GenericModel::ModelWithBuilder(ir1_structured_ast_model),
    );
    languages.insert(
        "ir2_flat_contracts".to_string(),
        GenericModel::ModelWithTransformer(ir2_flat_contracts_model),
    );

    languages
}

fn build_ir1_structured_ast_model(cst_model: &IrModel) -> ModelWithBuilder {
    let mut ir1_structured_ast_model = IrModel::from_model(cst_model);

    // remove fields from sequences that contain redundant terminal nodes
    for (_, sequence) in &mut ir1_structured_ast_model.sequences {
        if sequence.multiple_operators {
            // don't remove terminals if the sequence is modelling a precedence
            // expression with multiple variant operators
            continue;
        }
        sequence.fields.retain(|field| {
            field.is_optional
                || !field.is_terminal
                || !ir1_structured_ast_model
                    .unique_terminals
                    .contains(&field.r#type)
        });
    }

    ModelWithBuilder::new(cst_model, ir1_structured_ast_model)
}

fn build_ir2_flat_contracts_model(structured_ast_model: &IrModel) -> ModelWithTransformer {
    let mut ir2_flat_contracts_model = IrModel::from_model(structured_ast_model);

    // L2 is for now only a proof of concept for rendering transfomation code
    // from previous trees. Therefore, the following modifications are (a
    // non-exhaustive list of) samples of what can be done.

    // Flatten contract specifiers and bring the inherited types and storage
    // layout to the contract definition itself.
    ir2_flat_contracts_model.remove_type("ContractSpecifiers");
    ir2_flat_contracts_model.add_sequence_field(
        "ContractDefinition",
        "inheritance_types",
        "InheritanceTypes",
        false,
    );
    ir2_flat_contracts_model.add_sequence_field(
        "ContractDefinition",
        "storage_layout",
        "StorageLayoutSpecifier",
        true,
    );

    ModelWithTransformer::new(structured_ast_model, ir2_flat_contracts_model)
}
