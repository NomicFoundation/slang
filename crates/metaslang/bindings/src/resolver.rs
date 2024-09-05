use std::iter::once;

use metaslang_cst::KindTypes;
use stack_graphs::{
    graph::Node,
    partial::{PartialPath, PartialPaths},
    stitching::{ForwardPartialPathStitcher, GraphEdgeCandidates, StitcherConfig},
};

use crate::{builder::Selector, Bindings, Definition, GraphHandle, Reference};

pub struct Resolver<'a, KT: KindTypes + 'static> {
    owner: &'a Bindings<KT>,
    reference_handle: GraphHandle,
    partials: PartialPaths,
    pub results: Vec<PartialPath>,
}

impl<'a, KT: KindTypes + 'static> Resolver<'a, KT> {
    pub fn build_for(reference: &Reference<'a, KT>) -> Self {
        let mut resolver = Self {
            owner: reference.owner,
            reference_handle: reference.handle,
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
            once(self.reference_handle),
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
                self.results.push(reference_path.clone());

                if self.path_pushes_super(reference_path) {
                    println!(
                        "Found push `super` in path ending at {}",
                        self.owner.to_definition(reference_path.end_node).unwrap()
                    );
                }
            }
        }
    }

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

    pub fn all(self) -> Vec<Definition<'a, KT>> {
        self.results
            .into_iter()
            .filter_map(|path| self.owner.to_definition(path.end_node))
            .collect()
    }

    /// Attempt to resolve the reference and find its definition. If the
    /// reference cannot be resolved in the current state of the bindings (eg.
    /// you may still need to add an imported file), this function will return
    /// `None`. Otherwise, it will always return the definition with the
    /// longest, not shadowed, path in the underlying stack graph. This is to
    /// ensure that we always get the actual definition of some identifier
    /// reference, and not an intermediate result such as an import alias.
    ///
    /// There are multiple reasons why a reference may resolve to more than one
    /// definition in well formed and valid code. For example:
    ///
    /// 1. Variable shadowing: this should be resolved in the rules file by
    ///    setting the precedence attribute to the appropriate edges.
    ///
    /// 2. Destructuring imports (with or without aliases): these are
    ///    represented in the graph as intermediate definition nodes along the
    ///    path to the actual definition; hence why this function will return
    ///    the longest path available.
    ///
    /// 3. Overriden virtual methods: a reference will find valid paths to all
    ///    available definitions in the class hierarchy.
    ///
    pub fn first(&self) -> Option<Definition<'a, KT>> {
        if self.results.len() <= 1 {
            self.results
                .first()
                .and_then(|path| self.owner.to_definition(path.end_node))
        } else {
            // attempt to disambiguate from all found alternatives

            // remove aliases
            let results = self
                .results
                .iter()
                .filter(|path| {
                    self.owner
                        .selectors
                        .get(&path.end_node)
                        .map_or(true, |s| !matches!(s, Selector::Alias))
                })
                .collect::<Vec<_>>();

            match results.len() {
                0 => {
                    // TODO: this is an error because the actual definition
                    // is not found, but maybe we can revert back to
                    // returning the longest path?
                    panic!("More than one definition found but they are all aliases")
                }
                1 => results.first().map(|path| Definition {
                    owner: self.owner,
                    handle: path.end_node,
                }),
                _ => {
                    for (index, result) in results.iter().enumerate() {
                        let definition = self.owner.to_definition(result.end_node).unwrap();
                        let selector = self.owner.selectors.get(&result.end_node);
                        println!(
                            "  {index}. {definition} (length {length}) (selector = {selector})",
                            index = index + 1,
                            length = result.edges.len(),
                            selector =
                                selector.map_or(String::from("<none>"), |s| format!("{s:?}")),
                        );
                        let Some(selector) = selector else {
                            continue;
                        };

                        if let Selector::ParentDefinitions(related) = selector {
                            let enclosing_node = related.first().expect("at least one parent");
                            if self.owner.stack_graph[*enclosing_node].is_definition() {
                                let enclosing_def =
                                    self.owner.to_definition(*enclosing_node).unwrap();
                                println!("   Selector points to {enclosing_def}");
                                let related_sel = self.owner.selectors.get(enclosing_node);
                                if let Some(Selector::ParentReferences(parents)) = related_sel {
                                    println!("      with parents:");
                                    for parent in parents {
                                        let parent_reference =
                                            self.owner.to_reference(*parent).unwrap();
                                        let parent_definition =
                                            parent_reference.jump_to_definition().unwrap();
                                        println!(
                                            "        {parent_reference} -> {parent_definition}"
                                        );
                                    }
                                }
                            } else {
                                println!("   Virtual selector doesn't point to a definition");
                            }
                        }
                    }

                    panic!(concat!(
                        "More than one non-alias definitions found and ",
                        "disambiguation not implemented yet"
                    ));
                }
            }
        }
    }
}
