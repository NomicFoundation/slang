use std::collections::BTreeMap;

use language_definition::model::Language;
use serde::Serialize;

use super::{IrModel, IrModelMutator, ModelWithBuilder, ModelWithTransformer};

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
    let mut mutator = IrModelMutator::create_from(cst_model);

    // remove fields from sequences that contain redundant terminal nodes
    for (sequence_id, sequence) in &cst_model.sequences {
        if sequence.multiple_operators {
            // don't remove terminals if the sequence is modelling a precedence
            // expression with multiple variant operators
            continue;
        }
        for field in &sequence.fields {
            if !field.is_optional
                && field.is_terminal
                && cst_model.unique_terminals.contains(&field.r#type)
            {
                mutator.remove_sequence_field(sequence_id, &field.label);
            }
        }
    }

    mutator.into()
}

fn build_ir2_flat_contracts_model(structured_ast_model: &IrModel) -> ModelWithTransformer {
    let mut mutator = IrModelMutator::create_from(structured_ast_model);

    // Flatten contract specifiers and bring the inherited types and storage
    // layout to the contract definition itself.
    mutator.remove_type("ContractSpecifiers");
    mutator.add_sequence_field(
        "ContractDefinition",
        "inheritance_types",
        "InheritanceTypes",
        false,
    );
    mutator.add_sequence_field(
        "ContractDefinition",
        "storage_layout",
        "StorageLayoutSpecifier",
        true,
    );

    // Unifiy function definition types
    mutator.add_choice_type("FunctionKind");
    mutator.add_choice_unique_terminal("FunctionKind", "Regular");
    mutator.add_choice_unique_terminal("FunctionKind", "Constructor");
    mutator.add_choice_unique_terminal("FunctionKind", "Unnamed");
    mutator.add_choice_unique_terminal("FunctionKind", "Fallback");
    mutator.add_choice_unique_terminal("FunctionKind", "Receive");
    mutator.add_choice_unique_terminal("FunctionKind", "Modifier");

    // Add the kind to the FunctionDefinition type, which will now hold all kinds
    mutator.add_sequence_field("FunctionDefinition", "kind", "FunctionKind", false);

    // And remove other specific function types and related attributes
    mutator.remove_type("ConstructorDefinition");
    mutator.remove_type("ConstructorAttributes");
    mutator.remove_type("ConstructorAttribute");

    mutator.remove_type("UnnamedFunctionDefinition");
    mutator.remove_type("UnnamedFunctionAttributes");
    mutator.remove_type("UnnamedFunctionAttribute");

    mutator.remove_type("FallbackFunctionDefinition");
    mutator.remove_type("FallbackFunctionAttributes");
    mutator.remove_type("FallbackFunctionAttribute");

    mutator.remove_type("ReceiveFunctionDefinition");
    mutator.remove_type("ReceiveFunctionAttributes");
    mutator.remove_type("ReceiveFunctionAttribute");

    mutator.remove_type("ModifierDefinition");
    mutator.remove_type("ModifierAttributes");
    mutator.remove_type("ModifierAttribute");

    // This also requires modifying the name and body fields
    mutator.remove_sequence_field("FunctionDefinition", "name");
    mutator.add_sequence_field("FunctionDefinition", "name", "Identifier", true);
    mutator.remove_sequence_field("FunctionDefinition", "body");
    mutator.add_sequence_field("FunctionDefinition", "body", "Block", true);

    // We don't need FunctionName or FunctionBody anymore
    mutator.remove_type("FunctionName");
    mutator.remove_type("FunctionBody");

    // Collapse redundant node types
    mutator.collapse_sequence("ParametersDeclaration");
    mutator.collapse_sequence("ReturnsDeclaration");
    mutator.collapse_sequence("YulParametersDeclaration");
    mutator.collapse_sequence("YulReturnsDeclaration");
    mutator.collapse_sequence("EventParametersDeclaration");
    mutator.collapse_sequence("ErrorParametersDeclaration");
    mutator.collapse_sequence("ImportAlias");
    mutator.collapse_sequence("ElseBranch");

    mutator.into()
}
