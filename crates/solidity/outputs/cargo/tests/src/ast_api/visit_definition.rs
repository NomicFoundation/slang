use slang_solidity::backend::binder::definitions::ContractDefinition;
use slang_solidity::backend::binder::{Binder, Definition};
use slang_solidity::backend::l2_flat_contracts::{
    ContractMember, StateVariableAttribute, TypeName,
};
use slang_solidity::cst::{Cursor, NodeId, NonterminalKind, TerminalKindExtensions};

use crate::ast_api::CompilationOutput;
use crate::binder;

pub fn visit_definition<'a>(
    compilation_output: &'a CompilationOutput,
    definition: &'a Definition,
) -> Vec<&'a Definition> {
    match definition {
        Definition::Contract(contract_definition) => {
            visit_contract(compilation_output, contract_definition)
        }
        Definition::Function(function_definition) => todo!(),
        Definition::Modifier(modifier_definition) => todo!(),
        _ => todo!(),
    }
}

fn type_name_node_id(type_name: &TypeName) -> Option<NodeId> {
    match type_name {
        TypeName::ArrayTypeName(array_type_name_struct) => Some(array_type_name_struct.node_id),
        TypeName::FunctionType(function_type_struct) => Some(function_type_struct.node_id),
        TypeName::MappingType(mapping_type_struct) => Some(mapping_type_struct.node_id),
        TypeName::ElementaryType(_elementary_type) => None,
        TypeName::IdentifierPath(terminal_nodes) => terminal_nodes.last().map(|last| last.id()),
    }
}

fn visit_contract<'a>(
    compilation_output: &'a CompilationOutput,
    contract_definition: &ContractDefinition,
) -> Vec<&'a Definition> {
    // for a contract, we need to explicitly follow inheritance specifiers and constructors,
    // and visit state variables and public functions

    let contract = compilation_output
        .ast_map
        .get(&contract_definition.node_id)
        .expect("Node to be defined")
        .as_contract_definition();

    let inheritance_definitions =
        contract
            .inheritance_types
            .iter()
            .map(|f| f.node_id)
            .map(|node_id| {
                compilation_output
                    .binder
                    .naviagate_to(node_id)
                    .expect("reference to be defined")
            });

    let state_vars_definitions = contract
        .members
        .iter()
        .filter_map(|member| match member {
            ContractMember::StateVariableDefinition(sv_def)
                if sv_def
                    .attributes
                    .contains(&StateVariableAttribute::PublicKeyword) =>
            {
                let type_node_id = type_name_node_id(&sv_def.type_name);
                if let Some(type_node_id) = type_node_id {
                    Some(vec![sv_def.node_id, type_node_id])
                } else {
                    Some(vec![sv_def.node_id])
                }
            }
            _ => None,
        })
        .flatten()
        .filter_map(|node_id| compilation_output.binder.naviagate_to(node_id));

    // contract_definition.bases
    todo!()
}

/// Visits a definition and returns all definitions that are referenced from its body.
/// This is a translation of the TypeScript logic from visit-definition.mts.
pub fn visit_definition_from_cursor<'a>(binder: &'a Binder, root: &Cursor) -> Vec<&'a Definition> {
    let mut referenced_definitions = Vec::new();
    let mut cursor = root.spawn();
    while cursor.go_to_next_terminal() {
        let node = cursor.node();
        if !node.is_terminal() {
            continue;
        }
        if !node.as_terminal().unwrap().kind.is_identifier() {
            continue;
        }

        if let Some(def) = binder
            .find_reference_by_identifier_node_id(node.id())
            .and_then(|reference| reference.resolution.as_definition_id())
            .and_then(|def_id| binder.find_definition_by_id(def_id))
        {
            referenced_definitions.push(def);
        }
    }
    referenced_definitions
}

#[test]
fn test_no_references() {
    const FILE_CONTENT: &str = r#"
abstract contract Ownable {
}"#;
    let output = crate::ast_api::pipeline::one_file_backend_pipeline(FILE_CONTENT).unwrap();
    let cursor = output.compilation_unit.files()[0].create_tree_cursor();
    let result = visit_definition_from_cursor(&output.binder, &cursor);
    assert!(result.is_empty());
}

#[test]
fn test_one_reference() {
    const FILE_CONTENT: &str = r#"
function test() {
  test();
}"#;
    let output = crate::ast_api::pipeline::one_file_backend_pipeline(FILE_CONTENT).unwrap();
    let cursor = output.compilation_unit.files()[0].create_tree_cursor();
    let result = visit_definition_from_cursor(&output.binder, &cursor);
    assert_eq_defs!(result, ["test"]);
}

#[test]
fn test_function_reference() {
    const FILE_CONTENT: &str = r#"
function test() {
  test2();
}

function test2() {
  test();
}
  "#;
    let output = crate::ast_api::pipeline::one_file_backend_pipeline(FILE_CONTENT).unwrap();
    let mut cursor = output.compilation_unit.files()[0].create_tree_cursor();
    assert!(cursor.go_to_next_nonterminal_with_kind(NonterminalKind::FunctionDefinition));
    let result = visit_definition_from_cursor(&output.binder, &cursor);
    assert_eq_defs!(result, ["test2"]);
}

#[test]
fn test_contract() {
    const FILE_CONTENT: &str = r#"
contract Test {
  function test() {
    test2();
  }

  function test2() {
    test();
  }
}
  "#;
    let output = crate::ast_api::pipeline::one_file_backend_pipeline(FILE_CONTENT).unwrap();
    let mut cursor = output.compilation_unit.files()[0].create_tree_cursor();
    assert!(cursor.go_to_next_nonterminal_with_kind(NonterminalKind::ContractDefinition));
    let result = visit_definition_from_cursor(&output.binder, &cursor);
    assert_eq_defs!(result, vec!["test2", "test"]);
}
