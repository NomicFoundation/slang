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

    // Unifiy function definition types
    ir2_flat_contracts_model.add_choice_type("FunctionKind");
    ir2_flat_contracts_model.add_choice_unique_terminal("FunctionKind", "Regular");
    ir2_flat_contracts_model.add_choice_unique_terminal("FunctionKind", "Constructor");
    ir2_flat_contracts_model.add_choice_unique_terminal("FunctionKind", "Unnamed");
    ir2_flat_contracts_model.add_choice_unique_terminal("FunctionKind", "Fallback");
    ir2_flat_contracts_model.add_choice_unique_terminal("FunctionKind", "Receive");
    ir2_flat_contracts_model.add_choice_unique_terminal("FunctionKind", "Modifier");

    // Add the kind to the FunctionDefinition type, which will now hold all kinds
    ir2_flat_contracts_model.add_sequence_field("FunctionDefinition", "kind", "FunctionKind", false);

    // And remove other specific function types and related attributes
    ir2_flat_contracts_model.remove_type("ConstructorDefinition");
    ir2_flat_contracts_model.remove_type("ConstructorAttributes");
    ir2_flat_contracts_model.remove_type("ConstructorAttribute");

    ir2_flat_contracts_model.remove_type("UnnamedFunctionDefinition");
    ir2_flat_contracts_model.remove_type("UnnamedFunctionAttributes");
    ir2_flat_contracts_model.remove_type("UnnamedFunctionAttribute");

    ir2_flat_contracts_model.remove_type("FallbackFunctionDefinition");
    ir2_flat_contracts_model.remove_type("FallbackFunctionAttributes");
    ir2_flat_contracts_model.remove_type("FallbackFunctionAttribute");

    ir2_flat_contracts_model.remove_type("ReceiveFunctionDefinition");
    ir2_flat_contracts_model.remove_type("ReceiveFunctionAttributes");
    ir2_flat_contracts_model.remove_type("ReceiveFunctionAttribute");

    ir2_flat_contracts_model.remove_type("ModifierDefinition");
    ir2_flat_contracts_model.remove_type("ModifierAttributes");
    ir2_flat_contracts_model.remove_type("ModifierAttribute");

    // This also requires modifying the name and body fields
    ir2_flat_contracts_model.remove_sequence_field("FunctionDefinition", "name");
    ir2_flat_contracts_model.add_sequence_field("FunctionDefinition", "name", "Identifier", true);
    ir2_flat_contracts_model.remove_sequence_field("FunctionDefinition", "body");
    ir2_flat_contracts_model.add_sequence_field("FunctionDefinition", "body", "Block", true);

    // We don't need FunctionName or FunctionBody anymore
    ir2_flat_contracts_model.remove_type("FunctionName");
    ir2_flat_contracts_model.remove_type("FunctionBody");

    // Remove extra holder nodes for parameters and returns declarations,
    // flattenning all the function declarations
    ir2_flat_contracts_model.remove_type("ParametersDeclaration");
    ir2_flat_contracts_model.remove_type("ReturnsDeclaration");

    ir2_flat_contracts_model.add_sequence_field(
        "FunctionDefinition",
        "parameters",
        "Parameters",
        false,
    );
    ir2_flat_contracts_model.add_sequence_field("FunctionDefinition", "returns", "Parameters", true);
    ir2_flat_contracts_model.add_sequence_field("FunctionType", "parameters", "Parameters", false);
    ir2_flat_contracts_model.add_sequence_field("FunctionType", "returns", "Parameters", true);

    // We need to patch up try/catch which use parameters type
    ir2_flat_contracts_model.add_sequence_field("TryStatement", "returns", "Parameters", true);
    ir2_flat_contracts_model.add_sequence_field(
        "CatchClauseError",
        "parameters",
        "Parameters",
        false,
    );

    // Ditto for Yul parameters
    ir2_flat_contracts_model.remove_type("YulParametersDeclaration");
    ir2_flat_contracts_model.add_sequence_field(
        "YulFunctionDefinition",
        "parameters",
        "YulParameters",
        false,
    );
    ir2_flat_contracts_model.remove_type("YulReturnsDeclaration");
    ir2_flat_contracts_model.add_sequence_field(
        "YulFunctionDefinition",
        "returns",
        "YulVariableNames",
        true,
    );

    ModelWithTransformer::new(structured_ast_model, ir2_flat_contracts_model)
}
