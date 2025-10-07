use anyhow::Result;
use codegen_generator::ir::{IrModel, ModelWithBuilder, ModelWithTransformer};
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

    // Unifiy function definition types
    l2_flat_contracts_model.add_choice_type("FunctionKind");
    l2_flat_contracts_model.add_choice_unique_terminal("FunctionKind", "Regular");
    l2_flat_contracts_model.add_choice_unique_terminal("FunctionKind", "Constructor");
    l2_flat_contracts_model.add_choice_unique_terminal("FunctionKind", "Unnamed");
    l2_flat_contracts_model.add_choice_unique_terminal("FunctionKind", "Fallback");
    l2_flat_contracts_model.add_choice_unique_terminal("FunctionKind", "Receive");
    l2_flat_contracts_model.add_choice_unique_terminal("FunctionKind", "Modifier");

    // Add the kind to the FunctionDefinition type, which will now hold all kinds
    l2_flat_contracts_model.add_sequence_field("FunctionDefinition", "kind", "FunctionKind", false);

    // And remove other specific function types and related attributes
    l2_flat_contracts_model.remove_type("ConstructorDefinition");
    l2_flat_contracts_model.remove_type("ConstructorAttributes");
    l2_flat_contracts_model.remove_type("ConstructorAttribute");

    l2_flat_contracts_model.remove_type("UnnamedFunctionDefinition");
    l2_flat_contracts_model.remove_type("UnnamedFunctionAttributes");
    l2_flat_contracts_model.remove_type("UnnamedFunctionAttribute");

    l2_flat_contracts_model.remove_type("FallbackFunctionDefinition");
    l2_flat_contracts_model.remove_type("FallbackFunctionAttributes");
    l2_flat_contracts_model.remove_type("FallbackFunctionAttribute");

    l2_flat_contracts_model.remove_type("ReceiveFunctionDefinition");
    l2_flat_contracts_model.remove_type("ReceiveFunctionAttributes");
    l2_flat_contracts_model.remove_type("ReceiveFunctionAttribute");

    l2_flat_contracts_model.remove_type("ModifierDefinition");
    l2_flat_contracts_model.remove_type("ModifierAttributes");
    l2_flat_contracts_model.remove_type("ModifierAttribute");

    // This also requires modifying the name and body fields
    l2_flat_contracts_model.remove_sequence_field("FunctionDefinition", "name");
    l2_flat_contracts_model.add_sequence_field("FunctionDefinition", "name", "Identifier", true);
    l2_flat_contracts_model.remove_sequence_field("FunctionDefinition", "body");
    l2_flat_contracts_model.add_sequence_field("FunctionDefinition", "body", "Block", true);

    // We don't need FunctionName or FunctionBody anymore
    l2_flat_contracts_model.remove_type("FunctionName");
    l2_flat_contracts_model.remove_type("FunctionBody");

    // Remove extra holder nodes for parameters and returns declarations,
    // flattenning all the function declarations
    l2_flat_contracts_model.remove_type("ParametersDeclaration");
    l2_flat_contracts_model.remove_type("ReturnsDeclaration");

    l2_flat_contracts_model.add_sequence_field(
        "FunctionDefinition",
        "parameters",
        "Parameters",
        false,
    );
    l2_flat_contracts_model.add_sequence_field("FunctionDefinition", "returns", "Parameters", true);
    l2_flat_contracts_model.add_sequence_field("FunctionType", "parameters", "Parameters", false);
    l2_flat_contracts_model.add_sequence_field("FunctionType", "returns", "Parameters", true);

    // We need to patch up try/catch which use parameters type
    l2_flat_contracts_model.add_sequence_field("TryStatement", "returns", "Parameters", true);
    l2_flat_contracts_model.add_sequence_field(
        "CatchClauseError",
        "parameters",
        "Parameters",
        false,
    );

    // Ditto for Yul parameters
    l2_flat_contracts_model.remove_type("YulParametersDeclaration");
    l2_flat_contracts_model.add_sequence_field(
        "YulFunctionDefinition",
        "parameters",
        "YulParameters",
        false,
    );
    l2_flat_contracts_model.remove_type("YulReturnsDeclaration");
    l2_flat_contracts_model.add_sequence_field(
        "YulFunctionDefinition",
        "returns",
        "YulVariableNames",
        true,
    );

    ModelWithTransformer::new(structured_ast_model, l2_flat_contracts_model)
}
