use std::{collections::HashMap, iter::once};

use metaslang_cst::kinds::KindTypes;
use stack_graphs::{
    partial::PartialPaths,
    stitching::{Database, DatabaseCandidates, ForwardPartialPathStitcher, StitcherConfig},
    NoCancellation,
};

use crate::{Bindings, Definition, GraphHandle, Reference};

pub struct DatabaseResolver<'a, KT: KindTypes + 'static> {
    owner: &'a BindingGraph<KT>,
    partials: PartialPaths,
    database: Database,
    references: HashMap<GraphHandle, Vec<GraphHandle>>,
}

impl<'a, KT: KindTypes + 'static> DatabaseResolver<'a, KT> {
    pub fn new(owner: &'a Bindings<KT>) -> Self {
        let database = Database::new();
        let partials = PartialPaths::new();

        Self {
            owner,
            partials,
            database,
            references: HashMap::new(),
        }
    }

    pub fn build(&mut self) {
        for file in self.owner.stack_graph.iter_files() {
            println!(
                "Finding minimal paths for {file}",
                file = self.owner.stack_graph[file].name()
            );
            ForwardPartialPathStitcher::find_minimal_partial_path_set_in_file(
                &self.owner.stack_graph,
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
        }
    }

    pub fn dump(&mut self) {
        for path in self.database.iter_partial_paths() {
            let path = &self.database[path];
            println!(
                "{path}",
                path = path.display(&self.owner.stack_graph, &mut self.partials)
            );
        }
    }

    fn resolve_internal(&mut self, reference: GraphHandle) -> Vec<GraphHandle> {
        if let Some(definitions) = self.references.get(&reference) {
            return definitions.to_vec();
        }

        let mut reference_paths = Vec::new();

        ForwardPartialPathStitcher::find_all_complete_partial_paths(
            &mut DatabaseCandidates::new(
                &self.owner.stack_graph,
                &mut self.partials,
                &mut self.database,
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
        results
    }

    pub fn resolve(&mut self, reference: &Reference<'a, KT>) -> Vec<Definition<'a, KT>> {
        self.resolve_internal(reference.handle)
            .iter()
            .map(|handle| {
                self.owner
                    .to_definition(*handle)
                    .expect("handle is a definition")
            })
            .collect()
    }

    pub fn resolve_all(&mut self) {
        for handle in self.owner.stack_graph.iter_nodes() {
            if self.owner.stack_graph[handle].is_reference() {
                let definition_handles = self.resolve_internal(handle);
                self.references.insert(handle, definition_handles);
            }
        }
    }
}
