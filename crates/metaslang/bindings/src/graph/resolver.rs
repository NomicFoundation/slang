use std::collections::{HashMap, HashSet};
use std::iter::once;

use metaslang_cst::kinds::KindTypes;
use stack_graphs::arena::Handle;
use stack_graphs::graph::{Degree, Edge, StackGraph};
use stack_graphs::partial::{PartialPath, PartialPaths};
use stack_graphs::stitching::{
    Database, ForwardCandidates, ForwardPartialPathStitcher, StitcherConfig, ToAppendable,
};
use stack_graphs::{CancellationError, NoCancellation};

use crate::builder::{BindingInfo, GraphHandle};

pub(crate) struct Resolver {
    partials: PartialPaths,
    database: Database,
    pub(crate) references: HashMap<GraphHandle, Vec<GraphHandle>>,
}

impl Resolver {
    pub fn new<KT: KindTypes + 'static>(owner: &BindingInfo<KT>) -> Self {
        let database = Database::new();
        let partials = PartialPaths::new();

        let mut resolver = Self {
            partials,
            database,
            references: HashMap::new(),
        };
        resolver.build(owner);
        resolver
    }

    fn build<KT: KindTypes + 'static>(&mut self, owner: &BindingInfo<KT>) {
        for file in owner.stack_graph.iter_files() {
            ForwardPartialPathStitcher::find_minimal_partial_path_set_in_file(
                &owner.stack_graph,
                &mut self.partials,
                file,
                StitcherConfig::default(),
                &NoCancellation,
                |stack_graph, partials, path| {
                    self.database
                        .add_partial_path(stack_graph, partials, path.clone());
                },
            )
            .expect("Should never be cancelled");

            self.database.ensure_both_directions(&mut self.partials);
        }
    }

    fn resolve_parents<KT: KindTypes + 'static>(
        &mut self,
        owner: &BindingInfo<KT>,
        reference: GraphHandle,
    ) -> Vec<GraphHandle> {
        owner
            .get_parents(reference)
            .iter()
            .flat_map(|handle| {
                if owner.is_definition(*handle) {
                    vec![*handle]
                } else {
                    self.resolve_internal(owner, *handle, false)
                }
            })
            .collect()
    }

    fn resolve_parents_recursively<KT: KindTypes + 'static>(
        &mut self,
        owner: &BindingInfo<KT>,
        parent: GraphHandle,
    ) -> Vec<GraphHandle> {
        let mut results = HashMap::new();
        let mut resolve_queue = Vec::new();
        resolve_queue.push(parent);
        while let Some(current) = resolve_queue.pop() {
            let current_parents = self.resolve_parents(owner, current);
            for current_parent in &current_parents {
                if !results.contains_key(current_parent) {
                    resolve_queue.push(*current_parent);
                }
            }
            results.insert(current, current_parents);
        }
        results.into_values().flatten().collect()
    }

    fn resolve_internal<KT: KindTypes + 'static>(
        &mut self,
        owner: &BindingInfo<KT>,
        reference: GraphHandle,
        allow_recursion: bool,
    ) -> Vec<GraphHandle> {
        if let Some(definitions) = self.references.get(&reference) {
            return definitions.clone();
        }

        // Save `PartialPaths` state to restore allocations after the resolution
        // is complete
        let checkpoint = self.partials.save_checkpoint();
        let mut reference_paths = Vec::new();

        let extensions = if allow_recursion {
            // look for extension scopes to apply to the reference
            let ref_parents = self.resolve_parents(owner, reference);
            let mut extensions = HashSet::new();
            for parent in &ref_parents {
                if let Some(extension_scope) = owner.get_extension_scope(*parent) {
                    extensions.insert(extension_scope);
                }

                if owner.inherits_extensions(*parent) {
                    let grand_parents = self.resolve_parents_recursively(owner, *parent);
                    for grand_parent in &grand_parents {
                        if let Some(extension_scope) = owner.get_extension_scope(*grand_parent) {
                            extensions.insert(extension_scope);
                        }
                    }
                }
            }
            extensions.drain().collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        let mut database = ExtendedDatabase::new(&mut self.database);

        ForwardPartialPathStitcher::find_all_complete_partial_paths(
            &mut DatabaseCandidatesExtended::new(
                owner,
                &mut self.partials,
                &mut database,
                extensions,
            ),
            once(reference),
            StitcherConfig::default(),
            &NoCancellation,
            |_graph, _partials, path| {
                reference_paths.push(path.clone());
            },
        )
        .expect("not cancelled");

        let mut results = Vec::new();
        for reference_path in &reference_paths {
            let end_node = reference_path.end_node;

            if reference_paths
                .iter()
                .all(|other| !other.shadows(&mut self.partials, reference_path))
            {
                results.push(end_node);
            }
        }

        // Reclaim arena memory used for this resolution
        self.partials.restore_checkpoint(checkpoint);
        results
    }

    #[allow(dead_code)]
    pub(crate) fn resolve_all<KT: KindTypes + 'static>(&mut self, owner: &BindingInfo<KT>) {
        for handle in owner.stack_graph.iter_nodes() {
            if owner.is_reference(handle)
                && owner.get_file(handle).is_some_and(|file| file.is_user())
            {
                let definition_handles = self.resolve_internal(owner, handle, true);
                self.references.insert(handle, definition_handles);
            }
        }
    }

    pub(crate) fn resolve_single<KT: KindTypes + 'static>(
        &mut self,
        owner: &BindingInfo<KT>,
        handle: GraphHandle,
    ) -> Vec<GraphHandle> {
        if owner.is_reference(handle) && owner.get_file(handle).is_some_and(|file| file.is_user()) {
            if let Some(definitions) = self.references.get(&handle) {
                definitions.clone()
            } else {
                let definition_handles = self.resolve_internal(owner, handle, true);
                self.references.insert(handle, definition_handles.clone());
                definition_handles
            }
        } else {
            Vec::new()
        }
    }
}

// This is a partial paths database, but we also need to keep track of edges
// added to connect to extension scopes
struct ExtendedDatabase<'a> {
    pub database: &'a mut Database,
    pub edges: Vec<PartialPath>,
}

impl<'a> ExtendedDatabase<'a> {
    fn new(database: &'a mut Database) -> Self {
        Self {
            database,
            edges: Vec::new(),
        }
    }
}

// These are handles to partial paths or edges in `ExtendedDatabase`
#[derive(Clone, Debug)]
enum ExtendedHandle {
    Handle(Handle<PartialPath>),
    Edge(usize),
}

impl ToAppendable<ExtendedHandle, PartialPath> for ExtendedDatabase<'_> {
    fn get_appendable<'a>(&'a self, handle: &'a ExtendedHandle) -> &'a PartialPath {
        match handle {
            ExtendedHandle::Handle(handle) => self.database.get_appendable(handle),
            ExtendedHandle::Edge(edge) => &self.edges[*edge],
        }
    }
}

struct DatabaseCandidatesExtended<'a, KT: KindTypes + 'static> {
    owner: &'a BindingInfo<KT>,
    partials: &'a mut PartialPaths,
    database: &'a mut ExtendedDatabase<'a>,
    extensions: Vec<GraphHandle>,
}

impl<'a, KT: KindTypes + 'static> DatabaseCandidatesExtended<'a, KT> {
    fn new(
        owner: &'a BindingInfo<KT>,
        partials: &'a mut PartialPaths,
        database: &'a mut ExtendedDatabase<'a>,
        extensions: Vec<GraphHandle>,
    ) -> Self {
        Self {
            owner,
            partials,
            database,
            extensions,
        }
    }
}

impl<'a, KT: KindTypes + 'static>
    ForwardCandidates<ExtendedHandle, PartialPath, ExtendedDatabase<'a>, CancellationError>
    for DatabaseCandidatesExtended<'a, KT>
{
    // Return the forward candidates from the encapsulated `Database` and inject
    // the extension edges if the given path's end is an extension hook
    fn get_forward_candidates<R>(&mut self, path: &PartialPath, result: &mut R)
    where
        R: std::iter::Extend<ExtendedHandle>,
    {
        let node = path.end_node;
        let is_complete = path.is_complete(&self.owner.stack_graph);

        let mut db_candidates = Vec::new();
        self.database.database.find_candidate_partial_paths(
            &self.owner.stack_graph,
            self.partials,
            path,
            &mut db_candidates,
        );
        result.extend(
            db_candidates
                .iter()
                .filter(|candidate| {
                    if is_complete {
                        let candidate = &self.database.database[**candidate];
                        if let Some(last_symbol) = candidate
                            .symbol_stack_postcondition
                            .iter(self.partials)
                            .last()
                        {
                            // Special case: when the current partial path is
                            // already complete, don't extend through a @typeof
                            // symbol since that will not resolve to anything
                            if &self.owner.stack_graph[last_symbol.symbol] == "@typeof" {
                                return false;
                            }
                        }
                    }
                    true
                })
                .map(|candidate| ExtendedHandle::Handle(*candidate)),
        );

        if self.owner.is_extension_hook(node) {
            for extension in &self.extensions {
                let edge = Edge {
                    source: node,
                    sink: *extension,
                    precedence: 0,
                };
                let mut partial_path =
                    PartialPath::from_node(&self.owner.stack_graph, self.partials, node);
                partial_path
                    .append(&self.owner.stack_graph, self.partials, edge)
                    .expect("path can be extended");
                let edge_handle = self.database.edges.len();
                self.database.edges.push(partial_path);
                result.extend(once(ExtendedHandle::Edge(edge_handle)));
            }
        }
    }

    fn get_joining_candidate_degree(&self, path: &PartialPath) -> Degree {
        // TODO: this may not be correct for extension scopes, but it's only
        // used for cycle detection
        self.database
            .database
            .get_incoming_path_degree(path.end_node)
    }

    fn get_graph_partials_and_db(
        &mut self,
    ) -> (&StackGraph, &mut PartialPaths, &ExtendedDatabase<'a>) {
        (&self.owner.stack_graph, self.partials, self.database)
    }
}
