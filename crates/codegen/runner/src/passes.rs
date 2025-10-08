use anyhow::Result;
use codegen_generator::ir::{IrModel, IrModelMutator, ModelWithBuilder, ModelWithTransformer};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::{CodegenFileSystem, CodegenRuntime};
use language_definition::model::Language;

pub fn generate_passes(
    fs: &mut CodegenFileSystem,
    language: &Language,
    crate_name: &str,
) -> Result<()> {
    let crate_path = CargoWorkspace::locate_source_crate(crate_name)?;
    let ir_input_dir = crate_path.join("src/ir");
    let ir_output_dir = crate_path.join("src/backend");

    // L0: CST:
    let cst_model = IrModel::from_language("cst", language);

    // L1: structured AST:
    let l1_structured_ast_model = build_l1_structured_ast_model(&cst_model);
    let l1_structured_ast_output_dir = ir_output_dir.join(&l1_structured_ast_model.target.name);
    CodegenRuntime::render_ir(
        fs,
        &ir_input_dir,
        &l1_structured_ast_output_dir,
        &l1_structured_ast_model,
    )?;

    // L2: flat contract specifiers:
    let l2_flat_contracts_model = build_l2_flat_contracts_model(&l1_structured_ast_model.target);
    let l2_flat_contracts_output_dir = ir_output_dir.join(&l2_flat_contracts_model.target.name);
    CodegenRuntime::render_ir(
        fs,
        &ir_input_dir,
        &l2_flat_contracts_output_dir,
        &l2_flat_contracts_model,
    )?;

    Ok(())
}

fn build_l1_structured_ast_model(cst_model: &IrModel) -> ModelWithBuilder {
    let mut mutator = IrModelMutator::create_from("l1_structured_ast", cst_model);

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

fn build_l2_flat_contracts_model(structured_ast_model: &IrModel) -> ModelWithTransformer {
    let mut mutator = IrModelMutator::create_from("l2_flat_contracts", structured_ast_model);

    // L2 is for now only a proof of concept for rendering transfomation code
    // from previous trees. Therefore, the following modifications are (a
    // non-exhaustive list of) samples of what can be done.

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

    // Remove extra holder nodes for parameters and returns declarations,
    // flattenning all the function declarations
    mutator.remove_type("ParametersDeclaration");
    mutator.remove_type("ReturnsDeclaration");

    mutator.add_sequence_field("FunctionDefinition", "parameters", "Parameters", false);
    mutator.add_sequence_field("FunctionDefinition", "returns", "Parameters", true);
    mutator.add_sequence_field("FunctionType", "parameters", "Parameters", false);
    mutator.add_sequence_field("FunctionType", "returns", "Parameters", true);

    // We need to patch up try/catch which use parameters type
    mutator.add_sequence_field("TryStatement", "returns", "Parameters", true);
    mutator.add_sequence_field("CatchClauseError", "parameters", "Parameters", false);

    // Ditto for Yul parameters
    mutator.remove_type("YulParametersDeclaration");
    mutator.add_sequence_field(
        "YulFunctionDefinition",
        "parameters",
        "YulParameters",
        false,
    );
    mutator.remove_type("YulReturnsDeclaration");
    mutator.add_sequence_field("YulFunctionDefinition", "returns", "YulVariableNames", true);

    // And event and error definitions
    mutator.remove_type("EventParametersDeclaration");
    mutator.add_sequence_field("EventDefinition", "parameters", "EventParameters", false);

    mutator.remove_type("ErrorParametersDeclaration");
    mutator.add_sequence_field("ErrorDefinition", "parameters", "ErrorParameters", false);

    // Remove unnecessary ImportAlias node
    mutator.remove_type("ImportAlias");
    mutator.add_sequence_field("PathImport", "alias", "Identifier", true);
    mutator.add_sequence_field("NamedImport", "alias", "Identifier", false);
    mutator.add_sequence_field("ImportDeconstructionSymbol", "alias", "Identifier", true);

    mutator.into()
}
