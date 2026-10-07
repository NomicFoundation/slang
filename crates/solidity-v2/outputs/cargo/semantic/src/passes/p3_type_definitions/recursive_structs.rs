//! One traversal of the struct members computes three results, which differ in
//! the references they follow from a member's type to another struct:
//!
//! | Property                     | Edges followed                          |
//! |------------------------------|-----------------------------------------|
//! | by-value dependencies (p8)   | struct members, fixed-size arrays       |
//! | `ByReference`                | + dynamic arrays, mapping values        |
//! | `ByFunctionType`             | + function parameter and return types   |
//!
//! For example, `struct S { S[] children; }` is valid and is `ByReference`,
//! but has no by-value dependency. A function type stores no value of its
//! parameter or return types, so a struct named only in a signature is not
//! recursive in Solidity's sense; such a cycle makes a struct `ByFunctionType`,
//! which tells whether expanding the struct's type description terminates.
//! Every kind includes the structs that reach a cycle without belonging to it.
//!
//! By-value cycles make a struct `ByValue` and infinitely sized, which is an
//! error. p8 reports it with its own ordered, depth-limited search matching
//! solc's diagnostics, so only the dependencies are recorded here.

use slang_solidity_v2_common::collections::{Map, Set};
use slang_solidity_v2_common::nodes::NodeId;

use crate::binder::{Binder, Definition, RecursionKind};
use crate::types::{
    ArrayType, FixedSizeArrayType, FunctionType, MappingType, StructType, TupleType, Type, TypeId,
    TypeRegistry,
};

pub(super) fn mark_recursive_structs(binder: &mut Binder, types: &TypeRegistry) {
    let graph = StructDependencyGraph::build(binder, types);
    let by_value = graph.reaching_cycles(|kind| kind == DependencyKind::ByValue);
    let by_reference = graph.reaching_cycles(|kind| kind != DependencyKind::FunctionSignature);
    let by_function_type = graph.reaching_cycles(|_| true);

    for (struct_id, node) in graph.nodes {
        let Definition::Struct(definition) = binder.get_definition_mut(struct_id) else {
            unreachable!("definition is not a struct");
        };
        definition.recursive = Some(if by_value.contains(&struct_id) {
            RecursionKind::ByValue
        } else if by_reference.contains(&struct_id) {
            RecursionKind::ByReference
        } else if by_function_type.contains(&struct_id) {
            RecursionKind::ByFunctionType
        } else {
            RecursionKind::NonRecursive
        });
        definition.by_value_dependencies = node.by_value.into_boxed_slice();
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum DependencyKind {
    ByValue,
    IndirectMember,
    FunctionSignature,
}

#[derive(Default)]
struct StructDependencies {
    // Reverse edges: the structs referring to this one.
    predecessors: Vec<(NodeId, DependencyKind)>,
    by_value: Vec<NodeId>,
}

/// Holds every struct.
#[derive(Default)]
struct StructDependencyGraph {
    nodes: Map<NodeId, StructDependencies>,
}

impl StructDependencyGraph {
    fn build(binder: &Binder, types: &TypeRegistry) -> Self {
        let mut graph = Self::default();
        for (struct_id, definition) in binder.definitions() {
            let Definition::Struct(definition) = definition else {
                continue;
            };
            graph.nodes.entry(*struct_id).or_default();
            for member in definition.ir_node.members.iter() {
                if let Some(type_id) = binder.node_typing(member.id()).as_type_id() {
                    graph.add_type_dependencies(types, *struct_id, type_id);
                }
            }
        }
        graph
    }

    fn add_dependency(&mut self, from: NodeId, to: NodeId, kind: DependencyKind) {
        let source = self.nodes.entry(from).or_default();
        if kind == DependencyKind::ByValue {
            source.by_value.push(to);
        }
        self.nodes
            .entry(to)
            .or_default()
            .predecessors
            .push((from, kind));
    }

    fn add_type_dependencies(&mut self, types: &TypeRegistry, from: NodeId, type_id: TypeId) {
        let mut pending = vec![(type_id, DependencyKind::ByValue)];
        while let Some((type_id, kind)) = pending.pop() {
            match types.get_type_by_id(type_id) {
                // Stop at structs; their members are visited separately.
                Type::Struct(StructType { definition_id, .. }) => {
                    self.add_dependency(from, *definition_id, kind);
                }
                Type::FixedSizeArray(FixedSizeArrayType { element_type, .. }) => {
                    pending.push((*element_type, kind));
                }
                Type::Array(ArrayType { element_type, .. })
                | Type::Mapping(MappingType {
                    value_type_id: element_type,
                    ..
                }) => {
                    let kind = if kind == DependencyKind::FunctionSignature {
                        kind
                    } else {
                        DependencyKind::IndirectMember
                    };
                    pending.push((*element_type, kind));
                }
                Type::Function(FunctionType {
                    parameter_types,
                    return_type,
                    ..
                }) => {
                    // Once inside a signature, nested containers and functions
                    // remain signature dependencies.
                    pending.extend(
                        parameter_types
                            .iter()
                            .chain(std::iter::once(return_type))
                            .map(|id| (*id, DependencyKind::FunctionSignature)),
                    );
                }
                Type::Tuple(TupleType { types }) => {
                    pending.extend(types.iter().map(|id| (*id, kind)));
                }
                _ => {}
            }
        }
    }

    /// Repeatedly removes leaves and their incoming edges. Exactly the nodes
    /// reaching a cycle retain outgoing edges. Each call runs in O(nodes + edges)
    /// time.
    fn reaching_cycles(&self, follow: impl Fn(DependencyKind) -> bool) -> Set<NodeId> {
        let predecessors = |id: &NodeId| {
            self.nodes[id]
                .predecessors
                .iter()
                .filter(|(_, kind)| follow(*kind))
                .map(|(predecessor, _)| *predecessor)
        };

        // The number of outgoing edges each struct still has.
        let mut counts: Map<NodeId, usize> = self.nodes.keys().map(|id| (*id, 0)).collect();
        for predecessor in self.nodes.keys().flat_map(predecessors) {
            *counts.get_mut(&predecessor).expect("predecessor exists") += 1;
        }
        let mut pending: Vec<NodeId> = counts
            .iter()
            .filter(|(_, count)| **count == 0)
            .map(|(id, _)| *id)
            .collect();

        while let Some(id) = pending.pop() {
            for predecessor in predecessors(&id) {
                let count = counts.get_mut(&predecessor).expect("predecessor exists");
                *count -= 1;
                if *count == 0 {
                    pending.push(predecessor);
                }
            }
        }

        counts
            .into_iter()
            .filter(|(_, count)| *count > 0)
            .map(|(id, _)| id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature_edges_are_skipped_not_stopped_at() {
        // 0 names 1 in a function signature, then 2 holds 1 by value: nothing
        // is on a cycle. 3 names itself in a signature, then through an array:
        // 3 is on a cycle for both filters. In each case the signature edge
        // comes first, so skipping it must not end the predecessor walk.
        let mut graph = StructDependencyGraph::default();
        for (from, to, kind) in [
            (0, 1, DependencyKind::FunctionSignature),
            (2, 1, DependencyKind::ByValue),
            (3, 3, DependencyKind::FunctionSignature),
            (3, 3, DependencyKind::IndirectMember),
        ] {
            graph.add_dependency(NodeId::from(from), NodeId::from(to), kind);
        }

        let expected = Set::from_iter([NodeId::from(3)]);
        assert_eq!(
            graph.reaching_cycles(|kind| kind != DependencyKind::FunctionSignature),
            expected
        );
        assert_eq!(graph.reaching_cycles(|_| true), expected);
    }
}
