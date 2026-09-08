use slang_solidity_v2_common::collections::{Map, Set, SortedMap};
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::nodes::NodeId;

use crate::binder::Binder;
use crate::context::{ContractData, FileNodeMapper};
use crate::types::TypeRegistry;

mod bytecode;
mod constants;
mod structs;

pub(crate) fn run(
    binder: &mut Binder,
    contract_data: &ContractData,
    types: &TypeRegistry,
    file_node_mapper: &FileNodeMapper,
    diagnostics: &mut DiagnosticCollection,
) {
    constants::detect_constant_value_dependency_cycles(binder, file_node_mapper, diagnostics);
    structs::run(binder, types, file_node_mapper, diagnostics);
    bytecode::detect_bytecode_dependency_cycles(
        binder,
        contract_data,
        file_node_mapper,
        diagnostics,
    );
}

/// The path length at which solc's cycle checks give up, imposed on its
/// `CycleDetector` by `DeclarationTypeChecker`, `PostTypeChecker` and
/// `CompilerStack` alike, which the searches mirroring them take as their
/// depth limit.
pub(crate) const MAX_DEPTH: usize = 256;

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
    /// The search gave up on a path longer than [`MAX_DEPTH`]. `node` is the
    /// node at which the limit was hit.
    DepthExceeded { node: NodeId },
    /// No cycle is reachable from the searched node.
    None,
}

impl DependencyGraph {
    fn new(edges: SortedMap<NodeId, Vec<NodeId>>) -> Self {
        Self { edges }
    }

    /// The graph without `nodes`: an edge into one then leads nowhere, as any
    /// edge to a node without an entry.
    fn excluding(mut self, nodes: &Set<NodeId>) -> Self {
        self.edges.retain(|node, _| !nodes.contains(node));
        self
    }

    /// Runs [`Self::find_cycle`] on every node and drops the cycle-free results.
    /// Returns every cycle, but only the first depth-exceeded result. A graph
    /// past the depth limit exceeds it from many nodes, so keeping them all
    /// would mean hundreds of identical errors.
    fn find_all_cycles(&self) -> impl Iterator<Item = (NodeId, CycleSearchResult)> + '_ {
        let mut reported_exhaustion = false;

        self.edges
            .keys()
            .copied()
            .filter_map(move |node| match self.find_cycle(node) {
                CycleSearchResult::None => None,
                CycleSearchResult::DepthExceeded { node: capped } => {
                    (!reported_exhaustion).then(|| {
                        reported_exhaustion = true;
                        (node, CycleSearchResult::DepthExceeded { node: capped })
                    })
                }
                CycleSearchResult::Cycle { via } => Some((node, CycleSearchResult::Cycle { via })),
            })
    }

    fn find_cycle(&self, node: NodeId) -> CycleSearchResult {
        self.visit(node, &mut Vec::new(), &mut Set::default())
    }

    /// The nodes that reach themselves, keeping no path for [`MAX_DEPTH`] to
    /// bound: this and [`Self::nodes_reaching_cycles`] stand for solc's
    /// `recursive` annotation, which walks without a limit.
    fn nodes_on_cycles(&self) -> Set<NodeId> {
        self.edges
            .keys()
            .copied()
            .filter(|&node| self.reaches_itself(node))
            .collect()
    }

    /// The nodes from which a cycle is reachable: those on one and every node
    /// reaching them.
    fn nodes_reaching_cycles(&self) -> Set<NodeId> {
        let mut predecessors: Map<NodeId, Vec<NodeId>> = Map::default();
        for (node, successors) in &self.edges {
            for successor in successors {
                predecessors.entry(*successor).or_default().push(*node);
            }
        }
        let mut reaching = self.nodes_on_cycles();
        let mut pending: Vec<NodeId> = reaching.iter().copied().collect();
        while let Some(node) = pending.pop() {
            for &predecessor in predecessors.get(&node).into_iter().flatten() {
                if reaching.insert(predecessor) {
                    pending.push(predecessor);
                }
            }
        }
        reaching
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

        if stack.len() >= MAX_DEPTH {
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

    /// Whether a walk from `node` returns to it.
    fn reaches_itself(&self, node: NodeId) -> bool {
        let mut visited = Set::default();
        let mut pending = vec![node];
        while let Some(current) = pending.pop() {
            for &successor in self.edges.get(&current).into_iter().flatten() {
                if successor == node {
                    return true;
                }
                if visited.insert(successor) {
                    pending.push(successor);
                }
            }
        }
        false
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
        let graph = chain(MAX_DEPTH + 10);

        assert_eq!(
            find_cycle(&graph, 0),
            CycleSearchResult::DepthExceeded {
                node: NodeId::from(MAX_DEPTH - 1)
            }
        );
    }

    #[test]
    fn chain_reaching_max_depth_is_rejected() {
        let graph = chain(MAX_DEPTH);

        assert_eq!(
            find_cycle(&graph, 0),
            CycleSearchResult::DepthExceeded {
                node: NodeId::from(MAX_DEPTH - 1)
            }
        );
    }

    #[test]
    fn chain_below_max_depth_is_accepted() {
        let graph = chain(MAX_DEPTH - 1);

        assert_eq!(find_cycle(&graph, 0), CycleSearchResult::None);
    }

    /// An oversized chain `2 -> 3 -> ... -> end`, entered by roots 0 and 1.
    /// Searches from 0, 1 and 2 all give up, each at a different node.
    fn oversized_chain(extra_edges: Vec<(usize, Vec<usize>)>) -> DependencyGraph {
        let chain_end = MAX_DEPTH + 1;
        let mut edges = vec![(0, vec![2]), (1, vec![2])];
        edges.extend((2..=chain_end).map(|id| {
            let successors = if id < chain_end { vec![id + 1] } else { vec![] };
            (id, successors)
        }));
        edges.extend(extra_edges);
        graph(edges)
    }

    #[test]
    fn find_all_cycles_reports_exhaustion_once() {
        // Three roots give up on the same oversized chain, at three different
        // nodes, and only the first is kept. Deeper roots stay under the cap and
        // are dropped.
        let graph = oversized_chain(vec![]);

        assert_eq!(
            graph.find_all_cycles().collect::<Vec<_>>(),
            vec![(
                NodeId::from(0),
                CycleSearchResult::DepthExceeded {
                    node: NodeId::from(MAX_DEPTH)
                }
            )]
        );
    }

    #[test]
    fn cycles_are_still_reported_after_an_exhaustion() {
        // A cycle declared after the oversized chain. Dropping the repeated
        // give-ups must not drop the cycle behind them.
        let first = MAX_DEPTH + 2;
        let second = first + 1;
        let graph = oversized_chain(vec![(first, vec![second]), (second, vec![first])]);

        assert_eq!(
            graph.find_all_cycles().collect::<Vec<_>>(),
            vec![
                (
                    NodeId::from(0),
                    CycleSearchResult::DepthExceeded {
                        node: NodeId::from(MAX_DEPTH)
                    }
                ),
                (
                    NodeId::from(first),
                    CycleSearchResult::Cycle {
                        via: NodeId::from(second)
                    }
                ),
                (
                    NodeId::from(second),
                    CycleSearchResult::Cycle {
                        via: NodeId::from(first)
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
        let chain_end = MAX_DEPTH - 1;
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

    #[test]
    fn nodes_on_cycles_omits_a_node_that_only_reaches_one() {
        let graph = graph(vec![(0, vec![1]), (1, vec![2]), (2, vec![1])]);

        assert_eq!(
            graph.nodes_on_cycles(),
            [NodeId::from(1), NodeId::from(2)].into_iter().collect()
        );
    }

    #[test]
    fn nodes_reaching_cycles_keeps_a_node_that_only_reaches_one() {
        let graph = graph(vec![(0, vec![1]), (1, vec![2]), (2, vec![1])]);

        assert_eq!(
            graph.nodes_reaching_cycles(),
            [NodeId::from(0), NodeId::from(1), NodeId::from(2)]
                .into_iter()
                .collect()
        );
    }

    #[test]
    fn a_ring_past_the_depth_limit_puts_every_node_on_a_cycle() {
        let length = MAX_DEPTH + 4;
        let ring = graph(
            (0..length)
                .map(|id| (id, vec![(id + 1) % length]))
                .collect(),
        );

        assert_eq!(ring.nodes_on_cycles().len(), length);
    }

    #[test]
    fn excluding_takes_a_node_off_its_cycles() {
        let graph = graph(vec![(0, vec![1]), (1, vec![2]), (2, vec![0])]);

        assert!(
            graph
                .excluding(&[NodeId::from(2)].into_iter().collect())
                .nodes_on_cycles()
                .is_empty()
        );
    }
}
