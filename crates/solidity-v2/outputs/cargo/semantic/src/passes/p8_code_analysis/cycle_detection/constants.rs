use slang_solidity_v2_common::collections::{SortedMap, SortedSet};
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::diagnostics::kinds::semantic::{
    CyclicConstantDependency, CyclicDependencyValidatorExhausted,
};
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_ir::ir::visitor::Visitor;

use super::{CycleSearchResult, DependencyGraph};
use crate::binder::Binder;
use crate::context::FileNodeMapper;

pub(super) fn detect_constant_value_dependency_cycles(
    binder: &Binder,
    file_node_mapper: &FileNodeMapper,
    diagnostics: &mut DiagnosticCollection,
) {
    let graph = DependencyGraph::new(build_dependencies(binder));
    for (constant_id, result) in graph.find_all_cycles() {
        match result {
            CycleSearchResult::Cycle { via } => {
                let name = binder
                    .find_definition_by_id(constant_id)
                    .unwrap()
                    .identifier();

                let via_name = binder.find_definition_by_id(via).unwrap().identifier();

                diagnostics.push(
                    file_node_mapper.file_id_from_node_id(constant_id).clone(),
                    name.range.clone(),
                    CyclicConstantDependency {
                        name: name.unparse().to_owned(),
                        via: via_name.unparse().to_owned(),
                    },
                );
            }
            CycleSearchResult::DepthExceeded { node } => {
                let name = binder.find_definition_by_id(node).unwrap().identifier();

                diagnostics.push(
                    file_node_mapper.file_id_from_node_id(node).clone(),
                    name.range.clone(),
                    CyclicDependencyValidatorExhausted,
                );
            }
            CycleSearchResult::None => unreachable!("cycle-free nodes are not returned"),
        }
    }
}

fn build_dependencies(binder: &Binder) -> SortedMap<NodeId, Vec<NodeId>> {
    binder
        .definitions()
        .iter()
        .filter_map(|(definition_id, definition)| {
            let value = definition.as_constant_value()?;

            let dependencies: Vec<NodeId> = collect_constant_dependencies(binder, value)
                .into_iter()
                .collect();

            // A constant with no dependencies cannot be on a cycle, so drop it
            // rather than keep an empty entry.
            if dependencies.is_empty() {
                return None;
            }

            Some((*definition_id, dependencies))
        })
        .collect()
}

fn collect_constant_dependencies(
    binder: &Binder,
    expression: &ir::Expression,
) -> SortedSet<NodeId> {
    let mut collector = DependencyCollector {
        binder,
        dependencies: SortedSet::default(),
    };
    ir::visitor::accept_expression(expression, &mut collector);
    collector.dependencies
}

struct DependencyCollector<'a> {
    binder: &'a Binder,
    // Dedups dependencies and visits them in `NodeId` (ie. declaration) order,
    // matching solc, whose cycle detector sorts each constant's dependencies by
    // declaration id. The order is observable: it selects which path to a cycle
    // is explored first (the reported `via`) and, at the depth backstop, whether
    // a shared constant is cached before a longer path reaches it.
    dependencies: SortedSet<NodeId>,
}

impl Visitor for DependencyCollector<'_> {
    fn visit_identifier(&mut self, node: &ir::Identifier) {
        if let Some(definition_id) = self
            .binder
            .find_reference_by_identifier_node_id(node.id())
            .map(|reference| {
                self.binder
                    .follow_symbol_aliases(reference.resolution.clone())
            })
            .and_then(|resolution| resolution.as_definition_id())
            // A constant declared without a value has no outgoing edges, so it
            // is not a graph node either. Skipping it here keeps every edge
            // pointing at a node the search can reach.
            .filter(|&id| {
                self.binder
                    .find_definition_by_id(id)
                    .is_some_and(|definition| definition.as_constant_value().is_some())
            })
        {
            self.dependencies.insert(definition_id);
        }
    }
}
