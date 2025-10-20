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
                && field.r#type.is_terminal()
                && cst_model.terminals[field.r#type.as_identifier()]
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
    mutator.remove_type("ContractSpecifier");
    mutator.collapse_sequence("InheritanceSpecifier");
    mutator.collapse_sequence("StorageLayoutSpecifier");
    mutator.add_sequence_field(
        "ContractDefinition",
        "inheritance_types",
        "InheritanceTypes",
        false,
    );
    mutator.add_sequence_field("ContractDefinition", "storage_layout", "Expression", true);

    // Unifiy function definition types
    mutator.add_enum_type(
        "FunctionKind",
        &[
            "Regular",
            "Constructor",
            "Unnamed",
            "Fallback",
            "Receive",
            "Modifier",
        ],
    );

    // Function visibility, computed from a subset of the attributes
    mutator.add_enum_type(
        "FunctionVisibility",
        &["Public", "Private", "Internal", "External"],
    );

    // Function mutability, computed from a subset of the attributes
    mutator.add_enum_type(
        "FunctionMutability",
        &["Pure", "View", "NonPayable", "Payable"],
    );

    // Add the kind to the FunctionDefinition type, which will now hold all kinds
    mutator.add_sequence_field("FunctionDefinition", "kind", "FunctionKind", false);
    mutator.add_sequence_field(
        "FunctionDefinition",
        "visibility",
        "FunctionVisibility",
        false,
    );
    mutator.add_sequence_field(
        "FunctionDefinition",
        "mutability",
        "FunctionMutability",
        false,
    );
    // We use an optional unique terminal to effectively have a boolean
    mutator.add_sequence_field(
        "FunctionDefinition",
        "virtual_keyword",
        "VirtualKeyword",
        true,
    );

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
    mutator.collapse_sequence("UsingAlias");
    mutator.collapse_sequence("StateVariableDefinitionValue");
    mutator.collapse_sequence("OverridePathsDeclaration");
    mutator.collapse_sequence("AssemblyFlagsDeclaration");
    mutator.collapse_sequence("VariableDeclarationValue");
    mutator.collapse_sequence("NamedArgumentGroup");

    // Collapse IndexAccessEnd manually (requires code in the transformer
    // implementation) because it's an optional containing an optional, and that
    // complicates automatic code generation in the transformer.
    mutator.remove_type("IndexAccessEnd");
    mutator.add_sequence_field("IndexAccessExpression", "end", "Expression", true);

    // Collapse the middle node in ArgumentsDeclaration
    mutator.remove_type("PositionalArgumentsDeclaration");
    mutator.remove_type("NamedArgumentsDeclaration");
    mutator.add_choice_variant("ArgumentsDeclaration", "PositionalArguments");
    mutator.add_choice_variant("ArgumentsDeclaration", "NamedArguments");

    mutator.into()
}
