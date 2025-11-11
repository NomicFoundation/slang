use std::collections::{HashMap, HashSet};

use slang_solidity::backend::binder::{Binder, Definition};
use slang_solidity::compilation::CompilationUnit;
use slang_solidity::cst::{Cursor, Node, NodeId, TerminalKindExtensions};

pub(crate) fn collect_all_definitions<'a>(
    compilation_unit: &'a CompilationUnit,
    binder: &'a Binder,
) -> Vec<&'a Definition> {
    let mut all_definitions = vec![];
    for file in compilation_unit.files() {
        let cursor = file.create_tree_cursor();
        all_definitions.append(&mut collect_definitions(binder, &cursor));
    }
    all_definitions
}

pub(crate) fn collect_definitions<'a>(binder: &'a Binder, root: &Cursor) -> Vec<&'a Definition> {
    let mut cursor = root.spawn();
    let mut definitions = Vec::new();
    while cursor.go_to_next_terminal() {
        let node = cursor.node();
        if node.is_nonterminal() || !node.as_terminal().unwrap().kind.is_identifier() {
            continue;
        }
        if let Some(definition) = binder.find_definition_by_identifier_node_id(node.id()) {
            definitions.push(definition);
        }
    }
    definitions
}

pub(crate) fn map_definitions_ids<'a>(
    compilation_unit: &'a CompilationUnit,
    binder: &'a Binder,
) -> HashMap<NodeId, Node> {
    let mut result = HashMap::new();

    let definitions = collect_all_definitions(compilation_unit, binder);
    let definitions_ids: HashSet<_> = definitions.iter().map(|def| def.node_id()).collect();

    for file in compilation_unit.files() {
        let mut cursor = file.create_tree_cursor();
        while cursor.go_to_next() {
            let node = cursor.node();
            if definitions_ids.contains(&node.id()) {
                result.insert(node.id(), node);
            }
        }
    }
    result
}
