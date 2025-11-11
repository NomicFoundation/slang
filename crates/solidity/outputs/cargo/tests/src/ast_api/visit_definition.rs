use std::rc::Rc;

use slang_solidity::backend::binder::{ContractDefinition, Definition};
use slang_solidity::backend::ir::ir2_flat_contracts::index::IntoIr;
use slang_solidity::backend::ir::ir2_flat_contracts::{self, ContractMember};
use slang_solidity::cst::TextIndex;

use crate::ast_api::collect_definitions::collect_definitions;
use crate::ast_api::follow_all_references::follow_all_references;
use crate::ast_api::CompilationOutput;

pub fn visit_definition<'a>(
    compilation_output: &'a CompilationOutput,
    definition: &'a Definition,
) -> Vec<&'a Definition> {
    match definition {
        Definition::Contract(contract_definition) => {
            visit_contract(compilation_output, contract_definition)
        }
        Definition::Struct(_) | Definition::Library(_) | Definition::Enum(_) => vec![],
        Definition::Function(definition) => follow_all_references(
            &compilation_output.binder,
            &definition.node.create_cursor(TextIndex::ZERO),
        ),
        Definition::Modifier(definition) => follow_all_references(
            &compilation_output.binder,
            &definition.node.create_cursor(TextIndex::ZERO),
        ),
        _ => collect_definitions(
            &compilation_output.binder,
            &definition.node().create_cursor(TextIndex::ZERO),
        ),
    }
}

fn visit_contract<'a>(
    compilation_output: &'a CompilationOutput,
    contract_definition: &ContractDefinition,
) -> Vec<&'a Definition> {
    // for a contract, we need to explicitly follow inheritance specifiers and constructors,
    // and visit state variables and public functions

    let contract = ir2_flat_contracts::ContractDefinition::into_ir(
        &contract_definition.node,
        &compilation_output.index,
    )
    .expect("Contract to be defined");

    let inheritance_nodes = contract
        .inheritance_types
        .iter()
        .map(|f| Rc::clone(&f.node));

    let function_nodes = contract.members.iter().filter_map(|member| match member {
        ir2_flat_contracts::ContractMember::FunctionDefinition(f)
            if f.kind != ir2_flat_contracts::FunctionKind::Regular =>
        {
            Some(Rc::clone(&f.node))
        }

        _ => None,
    });

    let follow_all_references = inheritance_nodes
        .chain(function_nodes)
        .map(|node| node.create_cursor(TextIndex::ZERO))
        .flat_map(|cursor| follow_all_references(&compilation_output.binder, &cursor));

    let public_state_vars_ids = contract.members.iter().filter_map(|member| match member {
        ContractMember::StateVariableDefinition(sv_def)
            if sv_def.visibility == ir2_flat_contracts::StateVariableVisibility::Public =>
        {
            Some(sv_def.name.id())
        }

        _ => None,
    });

    let public_functions_ids = contract.members.iter().filter_map(|member| match member {
        ir2_flat_contracts::ContractMember::FunctionDefinition(f)
            if f.kind == ir2_flat_contracts::FunctionKind::Regular
                && f.visibility == ir2_flat_contracts::FunctionVisibility::Public =>
        {
            f.name.as_ref().map(|node| node.id())
        }

        _ => None,
    });

    let public_definitions = public_state_vars_ids
        .chain(public_functions_ids)
        .filter_map(|id| {
            compilation_output
                .binder
                .find_definition_by_identifier_node_id(id)
        });

    follow_all_references.chain(public_definitions).collect()
}

#[test]
fn test_no_references() {
    const FILE_CONTENT: &str = r#"
contract Ownable {
}"#;
    let output = crate::ast_api::pipeline::one_file_backend_pipeline(FILE_CONTENT).unwrap();
    let defs = output.binder.definitions();
    assert_eq!(defs.len(), 1);
    assert_eq!(
        defs.values()
            .map(|v| v.identifier().unparse())
            .collect::<Vec<_>>(),
        vec!["Ownable".to_string()]
    );
    let result = visit_definition(&output, defs.values().next().unwrap());
    assert!(result.is_empty());
}

#[test]
fn test_no_public_reference() {
    const FILE_CONTENT: &str = r#"
contract Ownable {
  function test() {
  }
}"#;
    let output = crate::ast_api::pipeline::one_file_backend_pipeline(FILE_CONTENT).unwrap();
    let defs = output.binder.definitions();
    let (_, contract) = defs
        .iter()
        .find(|(_, def)| def.identifier().unparse() == "Ownable")
        .unwrap();
    let result = visit_definition(&output, contract);
    assert!(result.is_empty());
}

#[test]
fn test_public_function_reference() {
    const FILE_CONTENT: &str = r#"
contract Ownable {
  function test() public {
  }
}"#;
    let output = crate::ast_api::pipeline::one_file_backend_pipeline(FILE_CONTENT).unwrap();
    let defs = output.binder.definitions();
    let (_, contract) = defs
        .iter()
        .find(|(_, def)| def.identifier().unparse() == "Ownable")
        .unwrap();
    let result = visit_definition(&output, contract);
    assert_eq_defs!(result, ["test"]);
}
