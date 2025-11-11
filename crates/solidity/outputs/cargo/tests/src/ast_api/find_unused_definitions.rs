use std::collections::{HashMap, VecDeque};

use anyhow::{bail, Result};
use slang_solidity::backend::binder::definitions::FunctionVisibility;
use slang_solidity::backend::binder::Definition::{self, Function};
use slang_solidity::cst::TextIndex;

use crate::ast_api::collect_definitions::{collect_all_definitions, collect_definitions};
use crate::ast_api::visit_definition::visit_definition_from_cursor;
use crate::ast_api::CompilationOutput;

pub fn find_unused_definitions<'a>(
    compilation_output: &'a CompilationOutput,
    starting_contract_name: &str,
) -> Vec<&'a Definition> {
    fn is_private_function(def: &Definition) -> bool {
        !matches!(def, Function(_))
            || matches!(def, Function(fundef) if matches!(fundef.visibility, FunctionVisibility::Private) || matches!(fundef.visibility, FunctionVisibility::Internal))
    }

    let all_definitions = collect_all_definitions(
        &compilation_output.compilation_unit,
        &compilation_output.binder,
    );

    // Only consider functions with private visibility as unused candidates
    let mut unused_definitions: HashMap<_, _> = all_definitions
        .iter()
        .map(|def| (def.node_id(), *def))
        .filter(|(_, def)| is_private_function(def))
        .collect();

    let mut visit_queue = VecDeque::new();
    if let Ok(contract) = find_contract_by_name(&all_definitions, starting_contract_name) {
        visit_queue.push_back(contract);
    }

    while let Some(to_visit) = visit_queue.pop_front() {
        let node_id = to_visit.node_id();
        if !unused_definitions.contains_key(&node_id) {
            continue;
        }
        unused_definitions.remove(&node_id);

        let definiens_definition = compilation_output
            .binder
            .find_definition_by_id(node_id)
            .unwrap();

        // TODO: missing way to obtain location from definition
        // assert_user_file_location(definiens_location);

        let node = compilation_output
            .cst_map
            .get(&definiens_definition.node_id())
            .expect("The definiens_location to be in the nodes map");
        let followed = visit_definition_from_cursor(
            &compilation_output.binder,
            &node.clone().create_cursor(TextIndex::ZERO),
        );
        for def in followed {
            visit_queue.push_back(def);
        }
    }

    // For remaining unused definitions, remove any nested definitions inside them
    // to prevent reporting eg. a function and all its parameters
    for def in unused_definitions.values() {
        visit_queue.push_back(*def);
    }
    while let Some(to_visit) = visit_queue.pop_front() {
        if !unused_definitions.contains_key(&to_visit.node_id()) {
            continue;
        }

        // assert_user_file_location(definiens_location);
        let node = compilation_output
            .cst_map
            .get(&to_visit.node_id())
            .expect("to_visit to be in the nodes map");
        let inner_definitions = collect_definitions(
            &compilation_output.binder,
            &node.clone().create_cursor(TextIndex::ZERO),
        );
        for inner in inner_definitions {
            if inner.node_id() == to_visit.node_id() {
                continue;
            }
            unused_definitions.remove(&inner.node_id());
        }
    }

    unused_definitions.values().copied().collect()
}

pub(crate) fn find_contract_by_name<'a>(
    definitions: &Vec<&'a Definition>,
    name: &'a str,
) -> Result<&'a Definition> {
    let matching_name: Vec<_> = definitions
        .iter()
        .filter(|def| {
            if matches!(def, Definition::Contract(_)) {
                def.identifier().text == name
            } else {
                false
            }
        })
        .collect();
    match matching_name.len() {
        0 => bail!("No contract matches name {name}"),
        1 => Ok(matching_name.first().unwrap()),
        _ => bail!("{name} is ambiguous"),
    }
}

#[test]
fn test_one_function() {
    const FILE_CONTENT: &str = r#"
contract Test {
  function test() private {
  }
}
  "#;
    let output = crate::ast_api::pipeline::one_file_backend_pipeline(FILE_CONTENT).unwrap();
    let result = find_unused_definitions(&output, "Test");
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].identifier().text, "test".to_owned());
}

#[test]
fn test_a_field_unused() {
    const FILE_CONTENT: &str = r#"
contract Test {
  uint _count;
  uint _unused;
  
  function count() public view returns (uint) {
    return _count;
  }
}
  "#;
    let output = crate::ast_api::pipeline::one_file_backend_pipeline(FILE_CONTENT).unwrap();
    let result = find_unused_definitions(&output, "Test");
    assert_eq_defs!(result, ["_unused"]);
}
