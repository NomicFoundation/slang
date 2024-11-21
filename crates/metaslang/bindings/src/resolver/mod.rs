use std::collections::{HashMap, HashSet};
use std::iter::once;

use metaslang_cst::kinds::KindTypes;
use stack_graphs::graph::{Degree, Edge, StackGraph};
use stack_graphs::partial::{PartialPath, PartialPaths};
use stack_graphs::stitching::{
    ForwardCandidates, ForwardPartialPathStitcher, GraphEdgeCandidates, GraphEdges, StitcherConfig,
};
use stack_graphs::{CancellationError, CancellationFlag};

use crate::{Bindings, Definition, FileHandle, GraphHandle, Reference, ResolutionError, Tag};

mod c3;

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
/// The multiple definitions can be ranked by one or more secondary algorithms
/// to be applied by this resolver. For example, an alias import definition
/// would be downgraded, allowing the resolver to prefer the actual definition
/// if found (it may not be available yet). Or a topological ordering may be
/// applied to definitions pointing to virtual methods.
///
pub(crate) struct Resolver<'a, KT: KindTypes + 'static> {
    owner: &'a Bindings<KT>,
    reference: Reference<'a, KT>,
    partials: PartialPaths,
    results: Vec<ResolvedPath<'a, KT>>,
    options: ResolveOptions,
}

#[derive(Copy, Clone)]
pub(crate) struct ResolveOptions {
    pub rank_results: bool,
    pub apply_c3_linearisation: bool,
    pub use_extension_hooks: bool,
    pub recursive_extension_scopes: bool,
}

impl ResolveOptions {
    pub fn reentrant_safe() -> Self {
        Self {
            rank_results: true,
            apply_c3_linearisation: false,
            use_extension_hooks: false,
            recursive_extension_scopes: false,
        }
    }
}

impl Default for ResolveOptions {
    fn default() -> Self {
        Self {
            rank_results: true,
            apply_c3_linearisation: true,
            use_extension_hooks: true,
            recursive_extension_scopes: false,
        }
    }
}

struct ResolvedPath<'a, KT: KindTypes + 'static> {
    pub partial_path: PartialPath,
    pub definition: Definition<'a, KT>,
    pub score: f32,
}

impl<'a, KT: KindTypes + 'static> ResolvedPath<'a, KT> {
    pub fn len(&self) -> usize {
        self.partial_path.edges.len()
    }
}

pub struct NoCancellation;
impl CancellationFlag for NoCancellation {
    fn check(&self, _at: &'static str) -> Result<(), CancellationError> {
        Ok(())
    }
}

/// Acts as a database of the edges in the graph.
struct ResolverCandidates<'a, KT: KindTypes + 'static> {
    owner: &'a Bindings<KT>,
    partials: &'a mut PartialPaths,
    file: Option<FileHandle>,
    edges: GraphEdges,
    extensions: &'a [GraphHandle],
}

impl<'a, KT: KindTypes + 'static> ResolverCandidates<'a, KT> {
    pub fn new(
        owner: &'a Bindings<KT>,
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
            // println!(
            //     "injecting edges to extensions at hook {node}",
            //     node = self.owner.stack_graph[node].id().local_id()
            // );
            let mut extension_edges = Vec::new();
            for extension in self.extensions {
                // println!(
                //     "- {extension}",
                //     extension = self.owner.stack_graph[*extension].id().local_id()
                // );
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

    #[allow(dead_code)]
    fn find_all_complete_partial_paths(&mut self) -> Result<Vec<PartialPath>, CancellationError> {
        let mut reference_paths = Vec::new();
        ForwardPartialPathStitcher::find_all_complete_partial_paths(
            &mut GraphEdgeCandidates::new(&self.owner.stack_graph, &mut self.partials, None),
            once(self.reference.handle),
            StitcherConfig::default(),
            &stack_graphs::NoCancellation,
            |_graph, _paths, path| {
                reference_paths.push(path.clone());
            },
        )?;
        Ok(reference_paths)
    }

    fn find_paths_with_extensions(
        &mut self,
        extensions: &[GraphHandle],
    ) -> Result<Vec<PartialPath>, CancellationError> {
        let mut reference_paths = Vec::new();
        let mut candidates =
            ResolverCandidates::new(&self.owner, &mut self.partials, None, extensions);
        let starting_nodes = once(self.reference.handle);
        let cancellation_flag = &NoCancellation;

        let (graph, partials, _) = candidates.get_graph_partials_and_db();
        let initial_paths = starting_nodes
            .into_iter()
            .filter(|n| graph[*n].is_reference())
            .map(|n| {
                let mut p = PartialPath::from_node(graph, partials, n);
                p.eliminate_precondition_stack_variables(partials);
                p
            })
            .collect::<Vec<_>>();

        let mut stitcher =
            ForwardPartialPathStitcher::from_partial_paths(graph, partials, initial_paths);

        stitcher.set_similar_path_detection(true);
        stitcher.set_collect_stats(false);
        stitcher.set_check_only_join_nodes(true);

        while !stitcher.is_complete() {
            cancellation_flag.check("finding complete partial paths")?;
            for path in stitcher.previous_phase_partial_paths() {
                candidates.load_forward_candidates(path, cancellation_flag)?;
            }
            stitcher.process_next_phase(&mut candidates, |_, _, _| true);
            let (graph, _, _) = candidates.get_graph_partials_and_db();
            for path in stitcher.previous_phase_partial_paths() {
                if path.is_complete(graph) {
                    //visit(graph, partials, path);
                    reference_paths.push(path.clone());
                }
            }
        }

        Ok(reference_paths)
    }

    fn resolve(&mut self) {
        let reference_paths = if self.options.use_extension_hooks {

            // let debug = self.reference.get_cursor().unwrap().node().unparse() == "addXXX";
            // if debug {
            //     println!(
            //         "Resolving {r} [{idx}] with extensions",
            //         r = self.reference,
            //         idx = self.owner.stack_graph[self.reference.handle]
            //             .id()
            //             .local_id()
            //             + 2
            //     );
            // }

            let ref_parents = self.reference.resolve_parents();
            let mut extensions = HashSet::new();
            for parent in &ref_parents {

                // if debug {
                //     println!("  Found parent {parent}");
                // }

                if let Some(extension_scope) = parent.get_extension_scope() {

                    // if debug {
                    //     println!(
                    //         "  - adding extension [{scope}]",
                    //         scope = self.owner.stack_graph[extension_scope].id().local_id() + 2
                    //     );
                    // }

                    extensions.insert(extension_scope);
                }

                if self.options.recursive_extension_scopes {
                    let grand_parents = Self::resolve_parents_all(parent.clone());
                    for grand_parent in grand_parents.values().flatten() {

                        // if debug {
                        //     println!("    Found grandparent {grand_parent}");
                        // }

                        if let Some(extension_scope) = grand_parent.get_extension_scope() {
                            extensions.insert(extension_scope);

                            // if debug {
                            //     println!(
                            //         "    - adding parent extension [{scope}]",
                            //         scope = self.owner.stack_graph[extension_scope].id().local_id() + 2
                            //     );
                            // }
                        }
                    }
                }
            }
            let extensions = extensions.drain().collect::<Vec<_>>();

            self.find_paths_with_extensions(&extensions)
                .expect("Should never be cancelled")
        } else {
            self.find_all_complete_partial_paths()
                .expect("Should never be cancelled")
        };

        let mut added_nodes = HashSet::new();
        for reference_path in &reference_paths {
            let end_node = reference_path.end_node;

            // Because of how we're using the scope stack to propagate dynamic
            // scopes, we may get multiple results with different scope stack
            // postconditions but reaching the exact same definition. We only
            // care about the definition, so we check for uniqueness.
            if !added_nodes.contains(&end_node)
                && reference_paths
                    .iter()
                    .all(|other| !other.shadows(&mut self.partials, reference_path))
            {
                self.results.push(ResolvedPath {
                    definition: self
                        .owner
                        .to_definition(end_node)
                        .expect("path to end in a definition node"),
                    partial_path: reference_path.clone(),
                    score: 0.0,
                });
                added_nodes.insert(end_node);
            }
        }
    }

    pub fn all(&self) -> Vec<Definition<'a, KT>> {
        self.results
            .iter()
            .map(|path| path.definition.clone())
            .collect()
    }

    pub fn first(&mut self) -> Result<Definition<'a, KT>, ResolutionError<'a, KT>> {
        if self.results.len() > 1 {
            if self.options.rank_results {
                self.rank_results();
            }

            let top_score = self.results[0].score;
            let mut results = self
                .results
                .iter()
                .take_while(|result| (result.score - top_score).abs() < f32::EPSILON)
                .map(|result| result.definition.clone())
                .collect::<Vec<_>>();
            if results.len() > 1 {
                Err(ResolutionError::AmbiguousDefinitions(results))
            } else {
                Ok(results.swap_remove(0))
            }
        } else {
            self.results
                .first()
                .map(|path| path.definition.clone())
                .ok_or(ResolutionError::Unresolved)
        }
    }

    fn rank_results(&mut self) {
        if self.results.is_empty() {
            return;
        }
        self.mark_down_aliases();
        self.mark_down_built_ins();
        if self.options.apply_c3_linearisation {
            self.rank_c3_methods();
        }
        self.results.sort_by(|a, b| b.score.total_cmp(&a.score));
    }

    fn mark_down_aliases(&mut self) {
        // compute min and max path lengths
        let (min_len, max_len) =
            self.results
                .iter()
                .fold((usize::MAX, usize::MIN), |(min_len, max_len), result| {
                    let len = result.len();
                    (min_len.min(len), max_len.max(len))
                });

        for result in &mut self.results {
            // mark down alias definitions
            #[allow(clippy::cast_precision_loss)]
            if result.definition.has_tag(Tag::Alias) {
                result.score -= 100.0;

                // but prioritize longer paths so that we can still return a
                // result if we only have multiple aliases as possible
                // definitions
                result.score += (result.len() - min_len) as f32 / (1 + max_len - min_len) as f32;
            }
        }
    }

    fn mark_down_built_ins(&mut self) {
        for result in &mut self.results {
            if result.definition.get_file().is_system() {
                result.score -= 200.0;
            }
        }
    }

    fn rank_c3_methods(&mut self) {
        // compute the linearisation to use for ranking
        let caller_parents = self.reference.resolve_parents();
        let Some(caller_context) = caller_parents.first() else {
            // the reference does not provide an enclosing definition, so nothing to do here
            return;
        };

        // if the bindings has some context set, use it instead of the caller's
        // to compute the full linearised ordering of methods
        let resolution_context = match self.owner.get_context() {
            Some(context) => context,
            None => caller_context.clone(),
        };

        #[allow(clippy::mutable_key_type)]
        let parents = Self::resolve_parents_all(resolution_context.clone());

        let Some(mro) = c3::linearise(&resolution_context, &parents) else {
            // linearisation failed
            eprintln!("Linearisation of {resolution_context} failed");
            return;
        };

        let caller_context_index = mro.iter().position(|x| x == caller_context);
        let super_call = self.reference.has_tag(Tag::Super);

        // Mark up user methods tagged C3 according to the computed linearisation.
        // Because only contract functions are marked with the C3 tag, this has
        // the added benefit of prioritizing them over globally defined
        // functions.
        for result in &mut self.results {
            if result.definition.has_tag(Tag::C3) && result.definition.get_file().is_user() {
                let definition_parents = result.definition.resolve_parents();
                let Some(definition_context) = definition_parents.first() else {
                    // this should not normally happen: the definition is tagged
                    // with the C3 selector but does not provide a resolvable
                    // enclosing definition
                    continue;
                };

                // find the definition context in the linearised result
                #[allow(clippy::cast_precision_loss)]
                if let Some(index) = mro.iter().position(|x| x == definition_context) {
                    // if this is a super call, ignore all implementations at or
                    // before (as in more derived) than the caller's
                    if !super_call
                        || (caller_context_index.is_none() || index > caller_context_index.unwrap())
                    {
                        result.score += 100.0 * (mro.len() - index) as f32;
                    }
                }
            }
        }
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
