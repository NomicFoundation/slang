use slang_solidity_v2_common::collections::SortedMap;
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::diagnostics::kinds::semantic::{
    RecursiveStruct, RecursiveStructValidatorExhausted,
};
use slang_solidity_v2_common::nodes::NodeId;

use super::{CycleSearchResult, DependencyGraph};
use crate::binder::{Binder, Definition, StructDefinition};
use crate::context::FileNodeMapper;
use crate::types::{FixedSizeArrayType, StructType, Type, TypeId, TypeRegistry};

// A struct is recursive if it contains itself by value, directly or through
// other structs.
pub(super) fn detect_recursive_structs(
    binder: &Binder,
    types: &TypeRegistry,
    file_node_mapper: &FileNodeMapper,
    diagnostics: &mut DiagnosticCollection,
) {
    let graph = DependencyGraph::new(build_dependencies(binder, types));
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

fn build_dependencies(binder: &Binder, types: &TypeRegistry) -> SortedMap<NodeId, Vec<NodeId>> {
    binder
        .definitions()
        .iter()
        .filter_map(|(definition_id, definition)| {
            let Definition::Struct(struct_definition) = definition else {
                return None;
            };

            // Successors keep member declaration order, matching solc's member
            // iteration. Duplicate edges from members of the same struct type
            // are harmless to the search.
            let dependencies: Vec<NodeId> = struct_definition
                .ir_node
                .members
                .iter()
                .filter_map(|member| binder.node_typing(member.id()).as_type_id())
                .filter_map(|type_id| struct_dependency(types, type_id))
                .collect();

            // A struct with no by-value struct members cannot be on a cycle, so
            // drop it rather than keep an empty entry.
            if dependencies.is_empty() {
                return None;
            }

            Some((*definition_id, dependencies))
        })
        .collect()
}

// Returns the struct held by value at `type_id`, peeling through fixed-size
// arrays. Stops at a dynamic array, mapping or other type, as these do not
// hold a struct by value.
fn struct_dependency(types: &TypeRegistry, type_id: TypeId) -> Option<NodeId> {
    let mut current = type_id;
    loop {
        match types.get_type_by_id(current) {
            Type::Struct(StructType { definition_id, .. }) => return Some(*definition_id),
            Type::FixedSizeArray(FixedSizeArrayType { element_type, .. }) => {
                current = *element_type;
            }
            _ => return None,
        }
    }
}

fn struct_definition(binder: &Binder, struct_id: NodeId) -> &StructDefinition {
    match binder.find_definition_by_id(struct_id) {
        Some(Definition::Struct(definition)) => definition,
        _ => panic!("graph nodes should be struct definitions"),
    }
}
