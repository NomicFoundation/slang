use slang_solidity_v2_common::collections::{Set, SortedMap};
use slang_solidity_v2_common::diagnostics::kinds::semantic::{
    CyclicConstantDependency, CyclicDependencyValidatorExhausted,
};
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_ir::ir::visitor::Visitor;

use crate::binder::{Binder, ConstantDefinition, Definition};

pub(super) fn detect_constant_cycles(binder: &Binder, diagnostics: &mut DiagnosticCollection) {
    let graph = ConstantGraph::build(binder);
    let mut reported_depth_exceeded = Set::default();

    for constant_id in graph.constant_ids() {
        match graph.find_cycle(constant_id) {
            CycleSearchResult::Cycle { via } => {
                let constant = constant_definition(binder, constant_id);
                let via = constant_definition(binder, via);
                diagnostics.push(
                    file_id(binder, constant),
                    constant.ir_node.range.clone(),
                    CyclicConstantDependency {
                        name: constant.ir_node.name.unparse().to_owned(),
                        via: via.ir_node.name.unparse().to_owned(),
                    },
                );
            }
            CycleSearchResult::DepthExceeded { constant } => {
                if reported_depth_exceeded.insert(constant) {
                    let constant = constant_definition(binder, constant);
                    diagnostics.push(
                        file_id(binder, constant),
                        constant.ir_node.range.clone(),
                        CyclicDependencyValidatorExhausted,
                    );
                }
            }
            CycleSearchResult::None => {}
        }
    }
}

struct ConstantGraph {
    dependencies: SortedMap<NodeId, Vec<NodeId>>,
}

#[derive(Debug, PartialEq, Eq)]
enum CycleSearchResult {
    Cycle { via: NodeId },
    DepthExceeded { constant: NodeId },
    None,
}

impl ConstantGraph {
    const MAX_DEPENDENCY_DEPTH: usize = 256;

    fn build(binder: &Binder) -> Self {
        let dependencies = binder
            .definitions()
            .iter()
            .filter_map(|(definition_id, definition)| {
                let Definition::Constant(constant) = definition else {
                    return None;
                };

                let dependencies = constant
                    .ir_node
                    .value
                    .as_ref()
                    .map_or_else(Vec::new, |value| {
                        collect_constant_dependencies(binder, value)
                    });

                Some((*definition_id, dependencies))
            })
            .collect();

        Self { dependencies }
    }

    fn constant_ids(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.dependencies.keys().copied()
    }

    fn dependencies(&self, constant_id: NodeId) -> &[NodeId] {
        self.dependencies
            .get(&constant_id)
            .expect("constant should exist in graph")
    }

    fn find_cycle(&self, constant_id: NodeId) -> CycleSearchResult {
        self.visit(constant_id, &mut Vec::new(), &mut Set::default())
    }

    fn visit(
        &self,
        constant_id: NodeId,
        stack: &mut Vec<NodeId>,
        processed: &mut Set<NodeId>,
    ) -> CycleSearchResult {
        // If we have already fully explored this constant's subtree, we know it's
        // cycle-free without re-exploring it.
        if processed.contains(&constant_id) {
            return CycleSearchResult::None;
        }

        stack.push(constant_id);

        if stack.len() >= Self::MAX_DEPENDENCY_DEPTH {
            stack.pop().expect("stack should not be empty");
            return CycleSearchResult::DepthExceeded {
                constant: constant_id,
            };
        }

        let mut result = CycleSearchResult::None;
        for dependency_id in self.dependencies(constant_id) {
            if stack.contains(dependency_id) {
                // Report the root's first dependency on this path as `via`.
                // That is the second stack entry. If the stack only holds the
                // root, the constant refers directly to itself, so the `via`
                // is the dependency we just followed.
                let via = stack.get(1).copied().unwrap_or(*dependency_id);
                result = CycleSearchResult::Cycle { via };
                break;
            }

            result = self.visit(*dependency_id, stack, processed);
            if !matches!(result, CycleSearchResult::None) {
                break;
            }
        }

        stack.pop().expect("stack should not be empty");

        // Cache cycle-free paths to avoid re-exploring them.
        if matches!(result, CycleSearchResult::None) {
            processed.insert(constant_id);
        }

        result
    }
}

fn constant_definition(binder: &Binder, constant_id: NodeId) -> &ConstantDefinition {
    match binder.find_definition_by_id(constant_id) {
        Some(Definition::Constant(constant)) => constant,
        _ => panic!("graph nodes should be constant definitions"),
    }
}

fn file_id(binder: &Binder, constant: &ConstantDefinition) -> String {
    binder
        .file_id_for_scope_id(constant.enclosing_scope_id)
        .unwrap_or_default()
        .to_owned()
}

fn collect_constant_dependencies(binder: &Binder, expression: &ir::Expression) -> Vec<NodeId> {
    let mut collector = DependencyCollector {
        binder,
        seen: Set::default(),
        dependencies: Vec::new(),
    };
    ir::visitor::accept_expression(expression, &mut collector);
    collector.dependencies
}

struct DependencyCollector<'a> {
    binder: &'a Binder,
    // Dedups repeated references, so `A = B + B` yields a single `B`
    // dependency.
    seen: Set<NodeId>,
    dependencies: Vec<NodeId>,
}

impl Visitor for DependencyCollector<'_> {
    fn visit_identifier(&mut self, node: &ir::Identifier) {
        for definition_id in self
            .binder
            .find_reference_by_identifier_node_id(node.id())
            .map(|reference| self.binder.follow_symbol_aliases(&reference.resolution))
            .into_iter()
            .flat_map(|resolution| resolution.get_definition_ids())
        {
            if matches!(
                self.binder.find_definition_by_id(definition_id),
                Some(Definition::Constant(_))
            ) && self.seen.insert(definition_id)
            {
                self.dependencies.push(definition_id);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn graph(edges: Vec<(usize, Vec<usize>)>) -> ConstantGraph {
        ConstantGraph {
            dependencies: edges
                .into_iter()
                .map(|(id, dependencies)| {
                    (
                        NodeId::from(id),
                        dependencies.into_iter().map(NodeId::from).collect(),
                    )
                })
                .collect(),
        }
    }

    fn find_cycle(graph: &ConstantGraph, root: usize) -> CycleSearchResult {
        graph.find_cycle(NodeId::from(root))
    }

    /// An acyclic chain `0 -> 1 -> ... -> length - 1`.
    fn chain(length: usize) -> ConstantGraph {
        graph(
            (0..length)
                .map(|id| {
                    let dependencies = if id + 1 < length {
                        vec![id + 1]
                    } else {
                        vec![]
                    };
                    (id, dependencies)
                })
                .collect(),
        )
    }

    #[test]
    fn acyclic_constants_report_nothing() {
        let graph = graph(vec![(0, vec![1, 2]), (1, vec![2]), (2, vec![])]);

        for root in 0..3 {
            assert_eq!(find_cycle(&graph, root), CycleSearchResult::None);
        }
    }

    #[test]
    fn self_cycle_reports_itself_as_via() {
        let graph = graph(vec![(0, vec![0])]);

        assert_eq!(
            find_cycle(&graph, 0),
            CycleSearchResult::Cycle {
                via: NodeId::from(0)
            }
        );
    }

    #[test]
    fn cycle_members_report_their_dependency_as_via() {
        let graph = graph(vec![(0, vec![1]), (1, vec![0])]);

        assert_eq!(
            find_cycle(&graph, 0),
            CycleSearchResult::Cycle {
                via: NodeId::from(1)
            }
        );
        assert_eq!(
            find_cycle(&graph, 1),
            CycleSearchResult::Cycle {
                via: NodeId::from(0)
            }
        );
    }

    #[test]
    fn tail_reports_first_dependency_on_path_to_cycle() {
        // 0 is not part of the cycle (1 -> 2 -> 1), but reaches it via 1.
        let graph = graph(vec![(0, vec![1]), (1, vec![2]), (2, vec![1])]);

        assert_eq!(
            find_cycle(&graph, 0),
            CycleSearchResult::Cycle {
                via: NodeId::from(1)
            }
        );
    }

    #[test]
    fn via_is_the_first_dependency_that_reaches_the_cycle() {
        // Dependency 1 is acyclic; the cycle is only reachable through 2.
        let graph = graph(vec![(0, vec![1, 2]), (1, vec![]), (2, vec![0])]);

        assert_eq!(
            find_cycle(&graph, 0),
            CycleSearchResult::Cycle {
                via: NodeId::from(2)
            }
        );
    }

    #[test]
    fn deep_acyclic_chain_exceeds_depth_limit() {
        let graph = chain(ConstantGraph::MAX_DEPENDENCY_DEPTH + 10);

        assert_eq!(
            find_cycle(&graph, 0),
            CycleSearchResult::DepthExceeded {
                constant: NodeId::from(ConstantGraph::MAX_DEPENDENCY_DEPTH - 1)
            }
        );
    }

    #[test]
    fn chain_reaching_max_depth_is_rejected() {
        let graph = chain(ConstantGraph::MAX_DEPENDENCY_DEPTH);

        assert_eq!(
            find_cycle(&graph, 0),
            CycleSearchResult::DepthExceeded {
                constant: NodeId::from(ConstantGraph::MAX_DEPENDENCY_DEPTH - 1)
            }
        );
    }

    #[test]
    fn chain_below_max_depth_is_accepted() {
        let graph = chain(ConstantGraph::MAX_DEPENDENCY_DEPTH - 1);

        assert_eq!(find_cycle(&graph, 0), CycleSearchResult::None);
    }

    #[test]
    fn processed_constant_is_skipped_when_rereached_at_the_depth_cap() {
        // Root 0 explores leaf 1000 first (a short path), then a chain
        // 1 -> 2 -> ... -> 254 -> 1000 that re-reaches it exactly at the cap.
        // The processed check must win over the depth cap.
        let target = 1000;
        let chain_end = ConstantGraph::MAX_DEPENDENCY_DEPTH - 1;
        let mut edges = vec![(0, vec![target, 1]), (target, vec![])];
        edges.extend((1..chain_end).map(|id| {
            let next = if id + 1 < chain_end { id + 1 } else { target };
            (id, vec![next])
        }));
        let graph = graph(edges);

        assert_eq!(find_cycle(&graph, 0), CycleSearchResult::None);
    }

    #[test]
    fn shared_acyclic_subgraphs_are_explored_once() {
        // A diamond DAG: each layer's two constants depend on both constants
        // of the next layer. Without memoizing fully-explored constants the
        // DFS enumerates every path (2^layers), which hangs the pass.
        let layers = 64;
        let mut edges = Vec::new();
        for layer in 0..layers {
            let dependencies = if layer + 1 < layers {
                vec![2 * (layer + 1), 2 * (layer + 1) + 1]
            } else {
                vec![]
            };
            edges.push((2 * layer, dependencies.clone()));
            edges.push((2 * layer + 1, dependencies));
        }
        let graph = graph(edges);

        for id in graph.constant_ids() {
            assert_eq!(
                graph.find_cycle(id),
                CycleSearchResult::None,
                "constant {id} should be cycle-free"
            );
        }
    }
}
