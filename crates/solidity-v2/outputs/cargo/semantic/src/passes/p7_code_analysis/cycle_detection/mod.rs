use slang_solidity_v2_common::collections::{Set, SortedMap};
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::nodes::NodeId;

use crate::binder::Binder;
use crate::context::FileNodeMapper;
use crate::types::TypeRegistry;

mod constants;
mod structs;

pub(crate) fn run(
    binder: &Binder,
    types: &TypeRegistry,
    file_node_mapper: &FileNodeMapper,
    diagnostics: &mut DiagnosticCollection,
) {
    constants::detect_constant_value_dependency_cycles(binder, file_node_mapper, diagnostics);
    structs::detect_recursive_structs(binder, types, file_node_mapper, diagnostics);
}

struct DependencyGraph {
    // Outgoing edges per node. Keys are node-id sorted for deterministic,
    // declaration-order iteration. Successors keep caller-insertion order, so
    // each builder controls its own successor ordering.
    edges: SortedMap<NodeId, Vec<NodeId>>,
}

#[derive(Debug, PartialEq, Eq)]
enum CycleSearchResult {
    /// A cycle is reachable from the searched node. `via` is the searched
    /// node's first successor on the path to the cycle (or the node itself when
    /// it refers directly to itself).
    Cycle { via: NodeId },
    /// The search gave up on a path longer than [`DependencyGraph::MAX_DEPTH`]. `node` is
    /// the node at which the limit was hit.
    DepthExceeded { node: NodeId },
    /// No cycle is reachable from the searched node.
    None,
}

impl DependencyGraph {
    const MAX_DEPTH: usize = 256;

    fn new(edges: SortedMap<NodeId, Vec<NodeId>>) -> Self {
        Self { edges }
    }

    /// Runs [`Self::find_cycle`] on every node, dropping cycle-free results.
    /// Searches from different roots can give up at the same node, so
    /// depth-exceeded results are reported once per node.
    fn find_all_cycles(&self) -> impl Iterator<Item = (NodeId, CycleSearchResult)> + '_ {
        let mut reported_depth_exceeded = Set::default();

        self.edges
            .keys()
            .copied()
            .filter_map(move |node| match self.find_cycle(node) {
                CycleSearchResult::None => None,
                CycleSearchResult::DepthExceeded { node: capped } => reported_depth_exceeded
                    .insert(capped)
                    .then_some((node, CycleSearchResult::DepthExceeded { node: capped })),
                CycleSearchResult::Cycle { via } => Some((node, CycleSearchResult::Cycle { via })),
            })
    }

    fn find_cycle(&self, node: NodeId) -> CycleSearchResult {
        self.visit(node, &mut Vec::new(), &mut Set::default())
    }

    fn visit(
        &self,
        node: NodeId,
        stack: &mut Vec<NodeId>,
        processed: &mut Set<NodeId>,
    ) -> CycleSearchResult {
        // A node already proven cycle-free is not re-explored. This is
        // deliberately blind to depth: re-reaching it through a longer path
        // could pass the cap, but solc's `CycleDetector` also skips processed
        // vertices before its depth check, so tracking their depth would make
        // us report a depth error where solc reports none.
        if processed.contains(&node) {
            return CycleSearchResult::None;
        }

        stack.push(node);

        if stack.len() >= Self::MAX_DEPTH {
            stack.pop().expect("stack should not be empty");
            return CycleSearchResult::DepthExceeded { node };
        }

        let mut result = CycleSearchResult::None;
        if let Some(successors) = self.edges.get(&node) {
            for &successor in successors {
                if stack.contains(&successor) {
                    // Report the searched node's first successor on this path as
                    // `via`. That is the second stack entry. If the stack only
                    // holds the searched node, it refers directly to itself, so
                    // the `via` is the successor we just followed.
                    let via = stack.get(1).copied().unwrap_or(successor);
                    result = CycleSearchResult::Cycle { via };
                    break;
                }

                result = self.visit(successor, stack, processed);
                if !matches!(result, CycleSearchResult::None) {
                    break;
                }
            }
        }

        stack.pop().expect("stack should not be empty");

        // Cache cycle-free paths to avoid re-exploring them.
        if matches!(result, CycleSearchResult::None) {
            processed.insert(node);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn graph(edges: Vec<(usize, Vec<usize>)>) -> DependencyGraph {
        DependencyGraph::new(
            edges
                .into_iter()
                .map(|(id, successors)| {
                    (
                        NodeId::from(id),
                        successors.into_iter().map(NodeId::from).collect(),
                    )
                })
                .collect(),
        )
    }

    fn find_cycle(graph: &DependencyGraph, root: usize) -> CycleSearchResult {
        graph.find_cycle(NodeId::from(root))
    }

    /// An acyclic chain `0 -> 1 -> ... -> length - 1`.
    fn chain(length: usize) -> DependencyGraph {
        graph(
            (0..length)
                .map(|id| {
                    let successors = if id + 1 < length {
                        vec![id + 1]
                    } else {
                        vec![]
                    };
                    (id, successors)
                })
                .collect(),
        )
    }

    #[test]
    fn acyclic_graph_reports_nothing() {
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
    fn cycle_members_report_their_successor_as_via() {
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
    fn tail_reports_first_successor_on_path_to_cycle() {
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
    fn via_is_the_first_successor_that_reaches_the_cycle() {
        // Successor 1 is acyclic; the cycle is only reachable through 2.
        let graph = graph(vec![(0, vec![1, 2]), (1, vec![]), (2, vec![0])]);

        assert_eq!(
            find_cycle(&graph, 0),
            CycleSearchResult::Cycle {
                via: NodeId::from(2)
            }
        );
    }

    #[test]
    fn successors_are_visited_in_insertion_order_not_sorted() {
        // Both successors of 0 reach the self-cycle at 3. `via` is whichever is
        // listed first, so preserving insertion order `[2, 1]` reports 2, while
        // sorting successors by id would report 1. This pins that the graph does
        // not reorder successors and leaves ordering to the caller.
        let graph = graph(vec![
            (0, vec![2, 1]),
            (1, vec![3]),
            (2, vec![3]),
            (3, vec![3]),
        ]);

        assert_eq!(
            find_cycle(&graph, 0),
            CycleSearchResult::Cycle {
                via: NodeId::from(2)
            }
        );
    }

    #[test]
    fn deep_acyclic_chain_exceeds_depth_limit() {
        let graph = chain(DependencyGraph::MAX_DEPTH + 10);

        assert_eq!(
            find_cycle(&graph, 0),
            CycleSearchResult::DepthExceeded {
                node: NodeId::from(DependencyGraph::MAX_DEPTH - 1)
            }
        );
    }

    #[test]
    fn chain_reaching_max_depth_is_rejected() {
        let graph = chain(DependencyGraph::MAX_DEPTH);

        assert_eq!(
            find_cycle(&graph, 0),
            CycleSearchResult::DepthExceeded {
                node: NodeId::from(DependencyGraph::MAX_DEPTH - 1)
            }
        );
    }

    #[test]
    fn chain_below_max_depth_is_accepted() {
        let graph = chain(DependencyGraph::MAX_DEPTH - 1);

        assert_eq!(find_cycle(&graph, 0), CycleSearchResult::None);
    }

    #[test]
    fn find_all_cycles_reports_each_capped_node_once() {
        // Roots 0 and 1 enter the same chain at the same offset, so both
        // searches give up at the same node and only the first is kept. Root 2
        // sits one step deeper into the chain, gives up one node later, and is
        // still reported. Deeper roots stay under the cap and are dropped.
        let chain_end = DependencyGraph::MAX_DEPTH + 1;
        let mut edges = vec![(0, vec![2]), (1, vec![2])];
        edges.extend((2..=chain_end).map(|id| {
            let successors = if id < chain_end { vec![id + 1] } else { vec![] };
            (id, successors)
        }));
        let graph = graph(edges);

        assert_eq!(
            graph.find_all_cycles().collect::<Vec<_>>(),
            vec![
                (
                    NodeId::from(0),
                    CycleSearchResult::DepthExceeded {
                        node: NodeId::from(DependencyGraph::MAX_DEPTH)
                    }
                ),
                (
                    NodeId::from(2),
                    CycleSearchResult::DepthExceeded {
                        node: NodeId::from(chain_end)
                    }
                ),
            ]
        );
    }

    #[test]
    fn processed_node_is_skipped_when_rereached_at_the_depth_cap() {
        // Leaf node 1 is root 0's lowest-id successor, so it is visited and
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
        let chain_end = DependencyGraph::MAX_DEPTH - 1;
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
        // A diamond DAG: each layer's two nodes have edges to both nodes of the
        // next layer. Without memoizing fully-explored nodes the DFS enumerates
        // every path (2^layers), which hangs the search.
        let layers = 64;
        let mut edges = Vec::new();
        for layer in 0..layers {
            let successors = if layer + 1 < layers {
                vec![2 * (layer + 1), 2 * (layer + 1) + 1]
            } else {
                vec![]
            };
            edges.push((2 * layer, successors.clone()));
            edges.push((2 * layer + 1, successors));
        }
        let graph = graph(edges);

        for id in graph.edges.keys().copied() {
            assert_eq!(
                graph.find_cycle(id),
                CycleSearchResult::None,
                "node {id} should be cycle-free"
            );
        }
    }
}
