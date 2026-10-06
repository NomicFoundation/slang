use slang_solidity_v2_common::collections::SortedMap;
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::diagnostics::kinds::semantic::{
    RecursiveStruct, RecursiveStructValidatorExhausted,
};
use slang_solidity_v2_common::nodes::NodeId;

use super::{CycleSearchResult, DependencyGraph};
use crate::binder::{Binder, Definition, StructDefinition};
use crate::context::FileNodeMapper;

// A struct is infinitely sized if it contains itself by value, directly or
// through other structs.
pub(super) fn detect_recursive_structs(
    binder: &Binder,
    file_node_mapper: &FileNodeMapper,
    diagnostics: &mut DiagnosticCollection,
) {
    let graph = DependencyGraph::new(build_dependencies(binder));
    for (struct_id, result) in graph.find_all_cycles() {
        match result {
            CycleSearchResult::Cycle { .. } => {
                let definition = struct_definition(binder, struct_id);
                diagnostics.push(
                    file_node_mapper.file_id_from_node_id(struct_id).clone(),
                    definition.ir_node.range.clone(),
                    RecursiveStruct,
                );
            }
            CycleSearchResult::DepthExceeded { node } => {
                let definition = struct_definition(binder, node);
                diagnostics.push(
                    file_node_mapper.file_id_from_node_id(node).clone(),
                    definition.ir_node.range.clone(),
                    RecursiveStructValidatorExhausted,
                );
            }
            CycleSearchResult::None => unreachable!("cycle-free nodes are not returned"),
        }
    }
}

// Member order matches solc's member iteration. A struct holding no struct by
// value cannot be on a cycle, so it gets no entry.
fn build_dependencies(binder: &Binder) -> SortedMap<NodeId, Vec<NodeId>> {
    binder
        .definitions()
        .iter()
        .filter_map(|(definition_id, definition)| match definition {
            Definition::Struct(definition) if !definition.by_value_dependencies.is_empty() => {
                Some((*definition_id, definition.by_value_dependencies.clone()))
            }
            _ => None,
        })
        .collect()
}

fn struct_definition(binder: &Binder, struct_id: NodeId) -> &StructDefinition {
    match binder.find_definition_by_id(struct_id) {
        Some(Definition::Struct(definition)) => definition,
        _ => panic!("graph nodes should be struct definitions"),
    }
}
