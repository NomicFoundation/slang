use std::collections::{HashMap, VecDeque};

use anyhow::{anyhow, Result};
use slang_solidity::backend::binder::Definition;
use slang_solidity::cst::{NodeId, TextIndex};

use crate::ast_api::collect_definitions::collect_definitions;
use crate::ast_api::visit_definition::visit_definition;
use crate::ast_api::CompilationOutput;

pub fn find_unused_definitions<'a>(
    compilation_output: &'a CompilationOutput,
    starting_contract_name: &str,
) -> Vec<&'a Definition> {
    let all_definitions = compilation_output.binder.definitions();
    let mut unused_definitions: HashMap<NodeId, _> =
        all_definitions.iter().map(|(id, def)| (*id, def)).collect();

    let mut visit_queue = VecDeque::new();
    if let Ok(contract) = find_contract_by_name(all_definitions, starting_contract_name) {
        visit_queue.push_back(contract);
    }

    while let Some(to_visit) = visit_queue.pop_front() {
        let node_id = to_visit.node_id();
        if !unused_definitions.contains_key(&node_id) {
            continue;
        }
        unused_definitions.remove(&node_id);

        let definitions = visit_definition(compilation_output, to_visit);
        for def in definitions {
            visit_queue.push_back(def);
        }
    }

    // For remaining unused definitions, remove any nested definitions inside them
    // to prevent reporting eg. a function and all its parameters
    for def in unused_definitions.values() {
        visit_queue.push_back(def);
    }
    while let Some(to_visit) = visit_queue.pop_front() {
        if !unused_definitions.contains_key(&to_visit.node_id()) {
            continue;
        }

        let inner_definitions = collect_definitions(
            &compilation_output.binder,
            &to_visit.node().create_cursor(TextIndex::ZERO),
        );
        for inner in inner_definitions {
            if inner.node_id() == to_visit.node_id() {
                continue;
            }
            unused_definitions.remove(&inner.node_id());
        }
    }

    unused_definitions.values().map(|def| *def).collect()
}

pub(crate) fn find_contract_by_name<'a>(
    definitions: &'a HashMap<NodeId, Definition>,
    name: &str,
) -> Result<&'a Definition> {
    definitions
        .values()
        .find(
            |def| matches!(def, Definition::Contract(contract) if contract.identifier.unparse() == name),
        ).ok_or_else(|| anyhow!("Can't find a contract {name}"))
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
    assert_eq_defs!(result, ["test"]);
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
