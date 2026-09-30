use slang_solidity_v2_common::collections::{Map, Set};
use slang_solidity_v2_common::nodes::NodeId;

use crate::binder::{Binder, Definition};
use crate::types::{
    ArrayType, FixedSizeArrayType, FunctionType, MappingType, StructType, TupleType, Type, TypeId,
    TypeRegistry,
};

/// Marks the structs from which a cycle of members is reachable through nested
/// structs, arrays and mappings, and those on a cycle closed by a function
/// type among the rest.
pub(super) fn mark_recursive_structs(binder: &mut Binder, types: &TypeRegistry) {
    let held = build_dependencies(binder, types, false);
    let on_held_cycles = nodes_on_cycles(&held);
    let recursive: Set<NodeId> = held
        .keys()
        .copied()
        .filter(|&struct_id| !reachable(&held, struct_id).is_disjoint(&on_held_cycles))
        .collect();
    let mut named = build_dependencies(binder, types, true);
    for dependencies in named.values_mut() {
        dependencies.retain(|struct_id| !recursive.contains(struct_id));
    }
    for struct_id in recursive.union(&nodes_on_cycles(&named)) {
        let Definition::Struct(definition) = binder.get_definition_mut(*struct_id) else {
            unreachable!("definition is not a struct");
        };
        definition.is_recursive = true;
    }
}

fn build_dependencies(
    binder: &Binder,
    types: &TypeRegistry,
    through_function_types: bool,
) -> Map<NodeId, Vec<NodeId>> {
    binder
        .definitions()
        .iter()
        .filter_map(|(definition_id, definition)| {
            let Definition::Struct(struct_definition) = definition else {
                return None;
            };
            let dependencies: Vec<NodeId> = struct_definition
                .ir_node
                .members
                .iter()
                .filter_map(|member| binder.node_typing(member.id()).as_type_id())
                .flat_map(|type_id| named_structs(types, type_id, through_function_types))
                .collect();
            Some((*definition_id, dependencies))
        })
        .collect()
}

// Returns the structs `type_id` names through arrays, mappings, tuples and,
// when `through_function_types`, the parameters and results of function types.
fn named_structs(
    types: &TypeRegistry,
    type_id: TypeId,
    through_function_types: bool,
) -> Vec<NodeId> {
    match types.get_type_by_id(type_id) {
        Type::Struct(StructType { definition_id, .. }) => vec![*definition_id],
        Type::Array(ArrayType { element_type, .. })
        | Type::FixedSizeArray(FixedSizeArrayType { element_type, .. }) => {
            named_structs(types, *element_type, through_function_types)
        }
        Type::Mapping(MappingType { value_type_id, .. }) => {
            named_structs(types, *value_type_id, through_function_types)
        }
        Type::Function(FunctionType {
            parameter_types,
            return_type,
            ..
        }) if through_function_types => parameter_types
            .iter()
            .chain(std::iter::once(return_type))
            .flat_map(|type_id| named_structs(types, *type_id, through_function_types))
            .collect(),
        Type::Tuple(TupleType {
            types: element_types,
        }) => element_types
            .iter()
            .flat_map(|type_id| named_structs(types, *type_id, through_function_types))
            .collect(),
        _ => Vec::new(),
    }
}

/// The nodes on a cycle: those reaching themselves.
fn nodes_on_cycles(graph: &Map<NodeId, Vec<NodeId>>) -> Set<NodeId> {
    graph
        .keys()
        .copied()
        .filter(|&node| reachable(graph, node).contains(&node))
        .collect()
}

/// The nodes reachable from `node`; `node` itself only when it lies on a cycle.
fn reachable(graph: &Map<NodeId, Vec<NodeId>>, node: NodeId) -> Set<NodeId> {
    let mut reached = Set::default();
    let mut pending = graph[&node].clone();
    while let Some(next) = pending.pop() {
        if reached.insert(next) {
            pending.extend(graph[&next].iter().copied());
        }
    }
    reached
}
