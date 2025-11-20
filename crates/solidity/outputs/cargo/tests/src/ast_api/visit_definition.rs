use std::rc::Rc;

use slang_solidity::backend::ast::index::IntoIr;
use slang_solidity::backend::ast::{self, ContractMember};
use slang_solidity::backend::binder::{ContractDefinition, Definition};
use slang_solidity::backend::BinderOutput;
use slang_solidity::cst::TextIndex;

use crate::ast_api::collect_definitions::collect_definitions;
use crate::ast_api::follow_all_references::follow_all_references;

pub fn visit_definition<'a>(
    compilation_output: &'a BinderOutput,
    definition: &'a Definition,
) -> Vec<&'a Definition> {
    match definition {
        Definition::Contract(contract_definition) => {
            // special case contracts; see below
            visit_contract(compilation_output, contract_definition)
        }
        Definition::Struct(_) | Definition::Library(_) | Definition::Enum(_) => {
            // members must be explicitly referenced
            vec![]
        }
        Definition::Function(definition) => {
            // follow any references inside, but don't automatically reference any new
            // definitions (eg. a parameter)
            follow_all_references(
                &compilation_output.binder,
                &definition.node.create_cursor(TextIndex::ZERO),
            )
        }
        Definition::Modifier(definition) => {
            // ditto
            follow_all_references(
                &compilation_output.binder,
                &definition.node.create_cursor(TextIndex::ZERO),
            )
        }
        _ => {
            // anything else (events, errors, interfaces) should be considered fully
            // referenced (including inner definitions) and we need to follow any
            // references inside them
            collect_definitions(
                &compilation_output.binder,
                &definition.node().create_cursor(TextIndex::ZERO),
            )
        }
    }
}

fn visit_contract<'a>(
    compilation_output: &'a BinderOutput,
    contract_definition: &ContractDefinition,
) -> Vec<&'a Definition> {
    // for a contract, we need to explicitly follow inheritance specifiers and constructors,
    // and visit state variables and public functions

    let contract =
        ast::ContractDefinition::into_ir(&contract_definition.node, &compilation_output.index)
            .expect("Contract to be defined");

    let inheritance_nodes = contract
        .inheritance_types
        .iter()
        .map(|f| Rc::clone(&f.node));

    // Special functions (receiver, fallback, unnamed, constructors)
    let function_nodes = contract.members.iter().filter_map(|member| match member {
        ast::ContractMember::FunctionDefinition(f)
            if !matches!(f.kind, ast::FunctionKind::Regular) =>
        {
            Some(Rc::clone(&f.node))
        }

        _ => None,
    });

    // For special functions and bases, we follow each reference
    let follow_all_references = inheritance_nodes
        .chain(function_nodes)
        .map(|node| node.create_cursor(TextIndex::ZERO))
        .flat_map(|cursor| follow_all_references(&compilation_output.binder, &cursor));

    let public_state_vars_ids = contract.members.iter().filter_map(|member| match member {
        ContractMember::StateVariableDefinition(sv_def)
            if matches!(sv_def.visibility, ast::StateVariableVisibility::Public) =>
        {
            Some(sv_def.name.id())
        }

        _ => None,
    });

    let public_functions_ids = contract.members.iter().filter_map(|member| match member {
        ast::ContractMember::FunctionDefinition(f)
            if matches!(f.kind, ast::FunctionKind::Regular)
                && matches!(f.visibility, ast::FunctionVisibility::Public) =>
        {
            f.name.as_ref().map(|node| node.id())
        }

        _ => None,
    });

    // For public functions and state vars we add their definition to the list, to recursively
    // consider them
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
    let output = crate::ast_api::pipeline::compile_one_file(FILE_CONTENT).unwrap();
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
    let output = crate::ast_api::pipeline::compile_one_file(FILE_CONTENT).unwrap();
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
    let output = crate::ast_api::pipeline::compile_one_file(FILE_CONTENT).unwrap();
    let defs = output.binder.definitions();
    let (_, contract) = defs
        .iter()
        .find(|(_, def)| def.identifier().unparse() == "Ownable")
        .unwrap();
    let result = visit_definition(&output, contract);
    assert_eq_defs!(result, ["test"]);
}
