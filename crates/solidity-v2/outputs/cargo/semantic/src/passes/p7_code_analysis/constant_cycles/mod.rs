use slang_solidity_v2_common::collections::{Set, SortedMap, SortedSet};
use slang_solidity_v2_common::diagnostics::kinds::semantic::{
    CyclicConstantDependency, CyclicDependencyValidatorExhausted,
};
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_ir::ir::visitor::Visitor;

use crate::binder::{Binder, ConstantDefinition, Definition, Scope};

pub(super) fn detect_constant_value_dependency_cycles(
    binder: &Binder,
    diagnostics: &mut DiagnosticCollection,
) {
    let graph = ConstantDependencyGraph::build(binder);
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

struct ConstantDependencyGraph {
    // Maps each `ConstantDefinition` to the set of other constants that
    // the former's value depends on directly.
    dependencies: SortedMap<NodeId, SortedSet<NodeId>>,
}

#[derive(Debug, PartialEq, Eq)]
enum CycleSearchResult {
    Cycle { via: NodeId },
    DepthExceeded { constant: NodeId },
    None,
}

impl ConstantDependencyGraph {
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
                    .map_or_else(SortedSet::default, |value| {
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

    fn dependencies(&self, constant_id: NodeId) -> &SortedSet<NodeId> {
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
        // A constant already proven cycle-free is not re-explored. This is
        // deliberately blind to depth: re-reaching it through a longer path
        // could pass the cap, but solc's `CycleDetector` also skips processed
        // vertices before its depth check, so tracking their depth would make
        // us report a depth error where solc reports none. Dependencies are
        // visited in declaration order (see `collect_constant_dependencies`),
        // matching solc, so this does not depend on source order.
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

fn file_id(binder: &Binder, constant: &ConstantDefinition) -> FileId {
    match binder.get_scope_by_id(constant.enclosing_scope_id) {
        Scope::File(file_scope) => file_scope.file_id.clone(),
        Scope::Contract(contract_scope) => {
            let Scope::File(file_scope) = binder.get_scope_by_id(contract_scope.file_scope_id)
            else {
                unreachable!("contract file scope should point to a file scope");
            };
            file_scope.file_id.clone()
        }
        _ => {
            unreachable!("constant definitions can only be declared in a file or a contract scope")
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn graph(edges: Vec<(usize, Vec<usize>)>) -> ConstantDependencyGraph {
        ConstantDependencyGraph {
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

    fn find_cycle(graph: &ConstantDependencyGraph, root: usize) -> CycleSearchResult {
        graph.find_cycle(NodeId::from(root))
    }

    /// An acyclic chain `0 -> 1 -> ... -> length - 1`.
    fn chain(length: usize) -> ConstantDependencyGraph {
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
        let graph = chain(ConstantDependencyGraph::MAX_DEPENDENCY_DEPTH + 10);

        assert_eq!(
            find_cycle(&graph, 0),
            CycleSearchResult::DepthExceeded {
                constant: NodeId::from(ConstantDependencyGraph::MAX_DEPENDENCY_DEPTH - 1)
            }
        );
    }

    #[test]
    fn chain_reaching_max_depth_is_rejected() {
        let graph = chain(ConstantDependencyGraph::MAX_DEPENDENCY_DEPTH);

        assert_eq!(
            find_cycle(&graph, 0),
            CycleSearchResult::DepthExceeded {
                constant: NodeId::from(ConstantDependencyGraph::MAX_DEPENDENCY_DEPTH - 1)
            }
        );
    }

    #[test]
    fn chain_below_max_depth_is_accepted() {
        let graph = chain(ConstantDependencyGraph::MAX_DEPENDENCY_DEPTH - 1);

        assert_eq!(find_cycle(&graph, 0), CycleSearchResult::None);
    }

    #[test]
    fn processed_constant_is_skipped_when_rereached_at_the_depth_cap() {
        // Leaf constant 1 is root 0's lowest-id dependency, so it is visited and
        // cached cycle-free first (at shallow depth). The chain 2 -> 3 -> ... ->
        // 255 -> 1 then re-reaches it when the stack is one short of the cap, so
        // pushing it would hit the cap. The cached result wins: the search
        // returns `None`, not a depth error.
        //
        // This pins that the processed check runs *before* the depth check, and
        // documents the intended depth-blindness of the cache. solc behaves the
        // same way (its `CycleDetector` skips processed vertices before its own
        // depth check), so re-checking depth here would diverge from solc.
        let target = 1;
        let chain_end = ConstantDependencyGraph::MAX_DEPENDENCY_DEPTH - 1;
        let mut edges = vec![(0, vec![target, 2]), (target, vec![])];
        edges.extend((2..=chain_end).map(|id| {
            let next = if id < chain_end { id + 1 } else { target };
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
