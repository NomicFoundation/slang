use slang_solidity_v2_common::collections::SortedMap;
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::diagnostics::kinds::semantic::{
    RecursiveStruct, RecursiveStructValidatorExhausted,
};
use slang_solidity_v2_common::nodes::NodeId;

use super::{CycleSearchResult, DependencyGraph};
use crate::binder::{Binder, Definition, StructDefinition};
use crate::context::FileNodeMapper;
use crate::types::{
    ArrayType, FixedSizeArrayType, FunctionType, MappingType, StructType, TupleType, Type, TypeId,
    TypeRegistry,
};

/// The kinds of member type through which one struct reaches another, in
/// inclusion order: each holds every kind before it.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Reach {
    /// The reached struct is laid out inline, so its size is part of the
    /// reaching one's.
    ByValue,
    /// The reached struct lies behind one slot of the reaching one, whose
    /// layout closes without it: a dynamic array's element or a mapping's
    /// value.
    Indirect,
    /// No storage of the reached struct is laid out at all.
    Function,
}

/// The struct-to-struct edges of the unit, one per member occurrence of the
/// reached struct, each labelled with the least reach that follows it, so the
/// graph at a reach keeps an edge when any occurrence lies within it.
/// Successors keep member declaration order, matching solc's member iteration.
type Dependencies = SortedMap<NodeId, Vec<(NodeId, Reach)>>;

pub(super) fn run(
    binder: &mut Binder,
    types: &TypeRegistry,
    file_node_mapper: &FileNodeMapper,
    diagnostics: &mut DiagnosticCollection,
) {
    let dependencies = build_dependencies(binder, types);
    detect_recursive_structs(binder, &dependencies, file_node_mapper, diagnostics);
    mark_recursive_structs(binder, &dependencies);
}

/// A struct that contains itself by value, directly or through other structs,
/// is rejected.
fn detect_recursive_structs(
    binder: &Binder,
    dependencies: &Dependencies,
    file_node_mapper: &FileNodeMapper,
    diagnostics: &mut DiagnosticCollection,
) {
    let graph = dependency_graph(dependencies, Reach::ByValue);
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

/// Marks the structs from which a cycle is reachable through dynamic arrays,
/// mappings, fixed-size arrays or nested structs, which solc's `recursive`
/// annotation marks, and those on a cycle closed by a function type that
/// passes through none of them.
fn mark_recursive_structs(binder: &mut Binder, dependencies: &Dependencies) {
    let mut recursive = dependency_graph(dependencies, Reach::Indirect).nodes_reaching_cycles();
    let remaining = dependency_graph(dependencies, Reach::Function).excluding(&recursive);
    recursive.extend(remaining.nodes_on_cycles());
    for struct_id in recursive {
        let Definition::Struct(definition) = binder.get_definition_mut(struct_id) else {
            unreachable!("graph nodes should be struct definitions");
        };
        definition.is_recursive = true;
    }
}

fn build_dependencies(binder: &Binder, types: &TypeRegistry) -> Dependencies {
    binder
        .definitions()
        .iter()
        .filter_map(|(definition_id, definition)| {
            let Definition::Struct(struct_definition) = definition else {
                return None;
            };
            let dependencies: Vec<(NodeId, Reach)> = struct_definition
                .ir_node
                .members
                .iter()
                .filter_map(|member| binder.node_typing(member.id()).as_type_id())
                .flat_map(|type_id| struct_dependencies(types, type_id, Reach::ByValue))
                .collect();
            Some((*definition_id, dependencies))
        })
        .collect()
}

/// The graph of the edges followed at `reach`.
fn dependency_graph(dependencies: &Dependencies, reach: Reach) -> DependencyGraph {
    DependencyGraph::new(
        dependencies
            .iter()
            .map(|(struct_id, edges)| {
                (
                    *struct_id,
                    edges
                        .iter()
                        .filter_map(|&(dependency, least)| (least <= reach).then_some(dependency))
                        .collect(),
                )
            })
            .collect(),
    )
}

/// The structs `type_id` reaches, each with the least reach that follows the
/// whole path to it, of which `reach` is the part already followed.
fn struct_dependencies(
    types: &TypeRegistry,
    type_id: TypeId,
    reach: Reach,
) -> Vec<(NodeId, Reach)> {
    match types.get_type_by_id(type_id) {
        Type::Struct(StructType { definition_id, .. }) => vec![(*definition_id, reach)],
        Type::FixedSizeArray(FixedSizeArrayType { element_type, .. }) => {
            struct_dependencies(types, *element_type, reach)
        }
        Type::Array(ArrayType { element_type, .. }) => {
            struct_dependencies(types, *element_type, reach.max(Reach::Indirect))
        }
        Type::Mapping(MappingType { value_type_id, .. }) => {
            struct_dependencies(types, *value_type_id, reach.max(Reach::Indirect))
        }
        Type::Function(FunctionType {
            parameter_types,
            return_type,
            ..
        }) => parameter_types
            .iter()
            .chain(std::iter::once(return_type))
            .flat_map(|type_id| struct_dependencies(types, *type_id, Reach::Function))
            .collect(),
        // A tuple is only ever a function's multi-value return type.
        Type::Tuple(TupleType {
            types: element_types,
        }) => element_types
            .iter()
            .flat_map(|type_id| struct_dependencies(types, *type_id, reach))
            .collect(),
        Type::Address(_)
        | Type::ArraySlice(_)
        | Type::Boolean
        | Type::ByteArray(_)
        | Type::Bytes(_)
        | Type::Contract(_)
        | Type::Enum(_)
        | Type::Error(_)
        | Type::Event(_)
        | Type::FixedPointNumber(_)
        | Type::Integer(_)
        | Type::Interface(_)
        | Type::Library(_)
        | Type::Literal(_)
        | Type::MetaType(_)
        | Type::String(_)
        | Type::UserDefinedValue(_)
        | Type::UserMetaType(_)
        | Type::Void => Vec::new(),
    }
}

fn struct_definition(binder: &Binder, struct_id: NodeId) -> &StructDefinition {
    match binder.find_definition_by_id(struct_id) {
        Some(Definition::Struct(definition)) => definition,
        _ => unreachable!("graph nodes should be struct definitions"),
    }
}
