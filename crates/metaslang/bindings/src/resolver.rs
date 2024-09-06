use std::collections::{HashMap, VecDeque};
use std::iter::once;

use metaslang_cst::KindTypes;
use stack_graphs::graph::Node;
use stack_graphs::partial::{PartialPath, PartialPaths};
use stack_graphs::stitching::{ForwardPartialPathStitcher, GraphEdgeCandidates, StitcherConfig};

use crate::builder::Selector;
use crate::{Bindings, Definition, GraphHandle, Reference};

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
pub struct Resolver<'a, KT: KindTypes + 'static> {
    owner: &'a Bindings<KT>,
    reference: Reference<'a, KT>,
    partials: PartialPaths,
    results: Vec<ResolvedPath<'a, KT>>,
}

struct ResolvedPath<'a, KT: KindTypes + 'static> {
    pub partial_path: PartialPath,
    pub definition: Definition<'a, KT>,
    pub score: f32,
    //pub pushes_super: bool,
}

impl<'a, KT: KindTypes + 'static> ResolvedPath<'a, KT> {
    pub fn end_node(&self) -> GraphHandle {
        self.partial_path.end_node
    }

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
                // TODO: we'll need this later when resolving virtual methods
                let _pushes_super = self.path_pushes_super(reference_path);
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

    // TODO: this is very specific to Solidity; find a way to generalize the concept
    fn path_pushes_super(&mut self, path: &PartialPath) -> bool {
        for edge in path.edges.iter(&mut self.partials) {
            let source_node_handle = self
                .owner
                .stack_graph
                .node_for_id(edge.source_node_id)
                .unwrap();
            let source_node = &self.owner.stack_graph[source_node_handle];
            if matches!(source_node, Node::PushScopedSymbol(_) | Node::PushSymbol(_)) {
                let symbol = &self.owner.stack_graph[source_node.symbol().unwrap()];
                if "super" == symbol {
                    return true;
                }
            }
        }
        false
    }

    pub fn all(&self) -> Vec<Definition<'a, KT>> {
        self.results
            .iter()
            .map(|path| path.definition.clone())
            .collect()
    }

    pub fn first(&mut self) -> Option<Definition<'a, KT>> {
        if self.results.len() > 1 {
            self.rank_results();

            if (self.results[1].score - self.results[0].score).abs() < f32::EPSILON {
                self.inspect_results();
                panic!(
                    "Reference {reference} resolved to multiple definitions that cannot be disambiguated",
                    reference = self.reference,
                );
            }

            Some(self.results[0].definition.clone())
        } else {
            self.results.first().map(|path| path.definition.clone())
        }
    }

    fn rank_results(&mut self) {
        if self.results.is_empty() {
            return;
        }
        self.mark_down_aliases();
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
            if result.definition.has_selector(Selector::Alias) {
                result.score -= 100.0;

                // but prioritize longer paths so that we can still return a
                // result if we only have multiple aliases as possible
                // definitions
                result.score += (result.len() - min_len) as f32 / (1 + max_len - min_len) as f32;
            }
        }
    }

    fn rank_c3_methods(&mut self) {
        // compute the linearisation to use for ranking
        let parents = self.reference.resolve_parents();
        let Some(context) = parents.first() else {
            // the reference does not provide an enclosing definition, so nothing to do here
            return;
        };
        let mro = Self::linearise(context);

        // mark up methods tagged with the C3 selector according to the computed linearisation
        for result in &mut self.results {
            if result.definition.has_selector(Selector::C3) {
                let definition_parents = result.definition.resolve_parents();
                let Some(definition_context) = definition_parents.first() else {
                    // this should not normally happen, but the definition is
                    // marked with C3 but does not provide a resolvable
                    // enclosing definition
                    continue;
                };
                #[allow(clippy::cast_precision_loss)]
                if let Some(order) = mro.iter().position(|x| x == definition_context) {
                    result.score += 100.0 * (mro.len() - order) as f32;
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

    fn linearise(context: &Definition<'a, KT>) -> Vec<Definition<'a, KT>> {
        let parents = Self::resolve_parents_all(context.clone());
        let mut linearisations: HashMap<Definition<'a, KT>, Vec<Definition<'a, KT>>> =
            HashMap::new();

        println!("Linearising {context} with {parents:#?}");
        let mut queue: VecDeque<Definition<'a, KT>> = VecDeque::new();
        queue.push_back(context.clone());

        while let Some(current) = queue.pop_front() {
            if linearisations.contains_key(&current) {
                continue;
            }
            let current_parents = &parents[&current];
            let mut merge_set = Vec::new();
            for parent in current_parents {
                if let Some(parent_linearisation) = linearisations.get(parent) {
                    merge_set.push(parent_linearisation.clone());
                } else {
                    // queue the parent at the front to resolve it first
                    queue.push_front(parent.clone());
                }
            }
            if merge_set.len() == current_parents.len() {
                merge_set.push(current_parents.clone());
                let mut result = Vec::new();
                result.push(current.clone());
                result.extend(Self::merge(merge_set));
                linearisations.insert(current, result);
            } else {
                // we're missing some linearisations, so re-enqueue current at the end and try again later
                queue.push_back(current);
            }
        }

        let result = linearisations.remove(context).unwrap_or_default();
        println!("Linearisation result: {result:#?}");
        result
    }

    fn merge(mut set: Vec<Vec<Definition<'a, KT>>>) -> Vec<Definition<'a, KT>> {
        // reverse all the vector in the set
        for subset in &mut set {
            subset.reverse();
        }

        // find a candidate
        let find_candidate = |set: &Vec<Vec<Definition<'a, KT>>>| -> Option<Definition<'a, KT>> {
            for subset in set {
                let Some(candidate) = subset.last() else {
                    continue;
                };
                if set.iter().all(|subset| {
                    let subset_len = subset.len();
                    subset
                        .iter()
                        .position(|elt| elt == candidate)
                        .unwrap_or(subset_len)
                        + 1
                        >= subset_len
                }) {
                    return Some(candidate.clone());
                }
            }
            None
        };

        // remove an element from the set
        let remove_from_set = |set: &mut Vec<Vec<Definition<'a, KT>>>,
                               element: &Definition<'a, KT>| {
            for subset in set {
                if let Some(last) = subset.last() {
                    if last == element {
                        subset.pop();
                    }
                }
            }
        };

        let mut result = Vec::new();
        loop {
            let found = find_candidate(&set);
            if let Some(found) = found {
                remove_from_set(&mut set, &found);
                result.push(found);
            } else {
                break;
            }
        }
        // if set is empty, we successfully merged the set
        // otherwise, linearisation is not possible
        if set.iter().all(|subset| subset.is_empty()) {
            result
        } else {
            Vec::new()
        }
    }

    fn inspect_results(&self) {
        let reference = &self.reference;
        println!("Reference {reference} has parents:");
        for parent in reference.resolve_parents() {
            println!("  -> {parent}");
            resolve_parents(&parent, 1);
        }

        println!("... and resolved to definitions:");
        for (index, result) in self.results.iter().enumerate() {
            let selector = self.owner.selectors.get(&result.end_node());
            println!(
                "  {index}. {definition} (score {score}, length {length}, selector {selector})",
                index = index + 1,
                definition = result.definition,
                score = result.score,
                length = result.len(),
                selector = selector.map_or(String::from("<none>"), |s| format!("{s:?}")),
            );

            resolve_parents(&result.definition, 0);
        }
    }
}

fn resolve_parents<KT: KindTypes + 'static>(definition: &Definition<'_, KT>, level: usize) {
    // FIXME: this cannot handle recursive definitions
    for parent in definition.resolve_parents() {
        println!(
            "  {indentation}-> {parent}",
            indentation = "  ".repeat(level)
        );
        resolve_parents(&parent, level + 1);
    }
}
