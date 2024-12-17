use std::collections::{HashMap, HashSet};
use std::iter::once;

use metaslang_cst::kinds::KindTypes;
use stack_graphs::graph::{Degree, Edge, StackGraph};
use stack_graphs::partial::{PartialPath, PartialPaths};
use stack_graphs::stitching::{
    ForwardCandidates, ForwardPartialPathStitcher, GraphEdgeCandidates, GraphEdges, StitcherConfig,
};
use stack_graphs::CancellationError;

use crate::{BindingGraph, Definition, FileHandle, GraphHandle, Reference};

/// The resolver executes algorithms to resolve a reference to one or more
/// definitions. The reference may not be resolvable in the current state of the
/// bindings (eg. you may still need to add an imported file), so it may not be
/// able to find any definitions, or the definitions it finds may be an
/// incomplete set (eg. finding only an import alias, but not the actual
/// definition). The base algorithm will omit shadowed paths (ie. those
/// discarded by higher precedence edges) from the results, but there are other
/// circumstances when many definitions may be found:
///
/// 1. Destructuring imports (with or without aliases): these are
///    represented in the graph as intermediate definition nodes along the
///    path to the actual definition; hence why this function will return
///    the longest path available.
///
/// 2. Virtual methods: a reference should find valid paths to all available
///    definitions in a class hierarchy.
///
pub(crate) struct Resolver<'a, KT: KindTypes + 'static> {
    owner: &'a BindingGraph<KT>,
    reference: Reference<'a, KT>,
    partials: PartialPaths,
    results: Vec<Definition<'a, KT>>,
    options: ResolveOptions,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) enum ResolveOptions {
    Full,
    NonRecursive,
}

/// Candidates for the forward stitching resolution process. This will inject
/// edges to the the given extensions scopes at extension hook nodes when asked
/// for forward candidates (ie. `get_forward_candidates`) by the resolution
/// algorithm. Other than that, it's exactly the same as `GraphEdgeCandidates`.
struct ResolverCandidates<'a, KT: KindTypes + 'static> {
    owner: &'a BindingGraph<KT>,
    partials: &'a mut PartialPaths,
    file: Option<FileHandle>,
    edges: GraphEdges,
    extensions: &'a [GraphHandle],
}

impl<'a, KT: KindTypes + 'static> ResolverCandidates<'a, KT> {
    pub fn new(
        owner: &'a BindingGraph<KT>,
        partials: &'a mut PartialPaths,
        file: Option<FileHandle>,
        extensions: &'a [GraphHandle],
    ) -> Self {
        Self {
            owner,
            partials,
            file,
            edges: GraphEdges,
            extensions,
        }
    }
}

impl<KT: KindTypes + 'static> ForwardCandidates<Edge, Edge, GraphEdges, CancellationError>
    for ResolverCandidates<'_, KT>
{
    fn get_forward_candidates<R>(&mut self, path: &PartialPath, result: &mut R)
    where
        R: std::iter::Extend<Edge>,
    {
        let node = path.end_node;
        result.extend(self.owner.stack_graph.outgoing_edges(node).filter(|e| {
            self.file
                .map_or(true, |file| self.owner.stack_graph[e.sink].is_in_file(file))
        }));

        if self.owner.is_extension_hook(node) {
            // Inject edges from the extension hook node to each extension scope
            let mut extension_edges = Vec::new();
            for extension in self.extensions {
                extension_edges.push(Edge {
                    source: node,
                    sink: *extension,
                    precedence: 0,
                });
            }
            result.extend(extension_edges);
        }
    }

    fn get_joining_candidate_degree(&self, path: &PartialPath) -> Degree {
        self.owner.stack_graph.incoming_edge_degree(path.end_node)
    }

    fn get_graph_partials_and_db(&mut self) -> (&StackGraph, &mut PartialPaths, &GraphEdges) {
        (&self.owner.stack_graph, self.partials, &self.edges)
    }
}

impl<'a, KT: KindTypes + 'static> Resolver<'a, KT> {
    pub fn build_for(reference: &Reference<'a, KT>, options: ResolveOptions) -> Self {
        let mut resolver = Self {
            owner: reference.owner,
            reference: reference.clone(),
            partials: PartialPaths::new(),
            results: Vec::new(),
            options,
        };
        resolver.resolve();
        resolver
    }

    fn resolve(&mut self) {
        let mut reference_paths = Vec::new();
        if self.options == ResolveOptions::Full {
            let ref_parents = self.reference.resolve_parents();
            let mut extensions = HashSet::new();
            for parent in &ref_parents {
                if let Some(extension_scope) = parent.get_extension_scope() {
                    extensions.insert(extension_scope);
                }

                if parent.inherit_extensions() {
                    #[allow(clippy::mutable_key_type)]
                    let grand_parents = Self::resolve_parents_all(parent.clone());
                    for grand_parent in grand_parents.values().flatten() {
                        if let Some(extension_scope) = grand_parent.get_extension_scope() {
                            extensions.insert(extension_scope);
                        }
                    }
                }
            }
            let extensions = extensions.drain().collect::<Vec<_>>();

            ForwardPartialPathStitcher::find_all_complete_partial_paths(
                &mut ResolverCandidates::new(self.owner, &mut self.partials, None, &extensions),
                once(self.reference.handle),
                StitcherConfig::default(),
                &stack_graphs::NoCancellation,
                |_graph, _paths, path| {
                    reference_paths.push(path.clone());
                },
            )
            .expect("Should never be cancelled");
        } else {
            ForwardPartialPathStitcher::find_all_complete_partial_paths(
                &mut GraphEdgeCandidates::new(&self.owner.stack_graph, &mut self.partials, None),
                once(self.reference.handle),
                StitcherConfig::default(),
                &stack_graphs::NoCancellation,
                |_graph, _paths, path| {
                    reference_paths.push(path.clone());
                },
            )
            .expect("Should never be cancelled");
        };

        let mut added_nodes = HashSet::new();
        for reference_path in &reference_paths {
            let end_node = reference_path.end_node;

            // There may be duplicate ending nodes with different
            // post-conditions in the scope stack, but we only care about the
            // definition itself. Hence we need to check for uniqueness.
            if !added_nodes.contains(&end_node)
                && reference_paths
                    .iter()
                    .all(|other| !other.shadows(&mut self.partials, reference_path))
            {
                self.results.push(
                    self.owner
                        .to_definition(end_node)
                        .expect("path to end in a definition node"),
                );
                added_nodes.insert(end_node);
            }
        }
    }

    pub fn all(self) -> Vec<Definition<'a, KT>> {
        self.results
    }

    #[allow(clippy::mutable_key_type)]
    fn resolve_parents_all(
        context: Definition<'a, KT>,
    ) -> HashMap<Definition<'a, KT>, Vec<Definition<'a, KT>>> {
        let mut results = HashMap::new();
        let mut resolve_queue = Vec::new();
        resolve_queue.push(context);
        while let Some(current) = resolve_queue.pop() {
            let current_parents = current.resolve_parents();
            for current_parent in &current_parents {
                if !results.contains_key(current_parent) {
                    resolve_queue.push(current_parent.clone());
                }
            }
            results.insert(current, current_parents);
        }
        results
    }
}
