use std::collections::HashMap;
use std::iter::once;

use metaslang_cst::kinds::KindTypes;
use stack_graphs::partial::{PartialPath, PartialPaths};
use stack_graphs::stitching::{ForwardPartialPathStitcher, GraphEdgeCandidates, StitcherConfig};

use crate::{Bindings, Definition, Reference, ResolutionError, Tag};

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

impl<'a, KT: KindTypes + 'static> Resolver<'a, KT> {
    pub fn build_for(reference: &Reference<'a, KT>) -> Self {
        let mut resolver = Self {
            owner: reference.owner,
            reference: reference.clone(),
            partials: PartialPaths::new(),
            results: Vec::new(),
        };
        resolver.resolve();
        resolver
    }

    fn resolve(&mut self) {
        let mut reference_paths = Vec::new();
        ForwardPartialPathStitcher::find_all_complete_partial_paths(
            &mut GraphEdgeCandidates::new(&self.owner.stack_graph, &mut self.partials, None),
            once(self.reference.handle),
            StitcherConfig::default(),
            &stack_graphs::NoCancellation,
            |_graph, _paths, path| {
                reference_paths.push(path.clone());
            },
        )
        .expect("should never be cancelled");

        for reference_path in &reference_paths {
            if reference_paths
                .iter()
                .all(|other| !other.shadows(&mut self.partials, reference_path))
            {
                self.results.push(ResolvedPath {
                    definition: self
                        .owner
                        .to_definition(reference_path.end_node)
                        .expect("path to end in a definition node"),
                    partial_path: reference_path.clone(),
                    score: 0.0,
                });
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
            self.rank_results();

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
        self.rank_c3_methods();
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
