use slang_solidity_v2_common::collections::{SortedMap, SortedSet};
use slang_solidity_v2_common::diagnostics::kinds::semantic::{
    CyclicConstantDependency, CyclicDependencyValidatorExhausted,
};
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_ir::ir::visitor::Visitor;

use super::{CycleSearchResult, DependencyGraph};
use crate::binder::{Binder, ConstantDefinition, Definition};
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
                let constant = constant_definition(binder, constant_id);
                let via = constant_definition(binder, via);
                diagnostics.push(
                    file_node_mapper.file_id_from_node_id(constant_id).clone(),
                    constant.ir_node.range.clone(),
                    CyclicConstantDependency {
                        name: constant.ir_node.name.unparse().to_owned(),
                        via: via.ir_node.name.unparse().to_owned(),
                    },
                );
            }
            CycleSearchResult::DepthExceeded { node } => {
                let constant = constant_definition(binder, node);
                diagnostics.push(
                    file_node_mapper.file_id_from_node_id(node).clone(),
                    constant.ir_node.range.clone(),
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
            let Definition::Constant(constant) = definition else {
                return None;
            };

            let dependencies: Vec<NodeId> = constant
                .ir_node
                .value
                .as_ref()
                .map_or_else(SortedSet::default, |value| {
                    collect_constant_dependencies(binder, value)
                })
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

fn constant_definition(binder: &Binder, constant_id: NodeId) -> &ConstantDefinition {
    match binder.find_definition_by_id(constant_id) {
        Some(Definition::Constant(constant)) => constant,
        _ => panic!("graph nodes should be constant definitions"),
    }
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
            .filter(|&id| {
                matches!(
                    self.binder.find_definition_by_id(id),
                    Some(Definition::Constant(_))
                )
            })
        {
            self.dependencies.insert(definition_id);
        }
    }
}
