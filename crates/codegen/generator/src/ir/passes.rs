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

    // L0: CST:
    let cst_model = IrModel::from_language("cst", language);

    // L1: structured AST:
    let l1_structured_ast_model = build_l1_structured_ast_model(&cst_model);

    // L2: flat contract specifiers:
    let l2_flat_contracts_model = build_l2_flat_contracts_model(&l1_structured_ast_model.target);

    languages.insert(
        "l1_structured_ast".to_string(),
        GenericModel::ModelWithBuilder(l1_structured_ast_model),
    );
    languages.insert(
        "l2_flat_contracts".to_string(),
        GenericModel::ModelWithTransformer(l2_flat_contracts_model),
    );

    languages
}

fn build_l1_structured_ast_model(cst_model: &IrModel) -> ModelWithBuilder {
    let mut l1_structured_ast_model = IrModel::from_model("l1_structured_ast", cst_model);

    // remove fields from sequences that contain redundant terminal nodes
    for (_, sequence) in &mut l1_structured_ast_model.sequences {
        if sequence.multiple_operators {
            // don't remove terminals if the sequence is modelling a precedence
            // expression with multiple variant operators
            continue;
        }
        sequence.fields.retain(|field| {
            field.is_optional
                || !field.is_terminal
                || !l1_structured_ast_model
                    .unique_terminals
                    .contains(&field.r#type)
        });
    }

    ModelWithBuilder::new(cst_model, l1_structured_ast_model)
}

fn build_l2_flat_contracts_model(structured_ast_model: &IrModel) -> ModelWithTransformer {
    let mut l2_flat_contracts_model =
        IrModel::from_model("l2_flat_contracts", structured_ast_model);

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

    ModelWithTransformer::new(structured_ast_model, l2_flat_contracts_model)
}
