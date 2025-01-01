use std::{collections::HashSet, iter::once};

use metaslang_cst::kinds::KindTypes;
use stack_graphs::{
    partial::PartialPaths,
    stitching::{Database, DatabaseCandidates, ForwardPartialPathStitcher, StitcherConfig},
    NoCancellation,
};

use crate::{Bindings, Definition, Reference};

pub struct DatabaseResolver<'a, KT: KindTypes + 'static> {
    pub owner: &'a Bindings<KT>,
    pub partials: PartialPaths,
    pub database: Database,
}

impl<'a, KT: KindTypes + 'static> DatabaseResolver<'a, KT> {
    pub fn new(owner: &'a Bindings<KT>) -> Self {
        let database = Database::new();
        let partials = PartialPaths::new();

        Self {
            owner,
            partials,
            database,
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

    pub fn resolve(&mut self, reference: &Reference<'a, KT>) -> Vec<Definition<'a, KT>> {
        let mut reference_paths = Vec::new();

        ForwardPartialPathStitcher::find_all_complete_partial_paths(
            &mut DatabaseCandidates::new(
                &self.owner.stack_graph,
                &mut self.partials,
                &mut self.database,
            ),
            once(reference.handle),
            StitcherConfig::default(),
            &NoCancellation,
            |_graph, _partials, path| {
                reference_paths.push(path.clone());
            },
        )
        .expect("not cancelled");

        let mut results = Vec::new();
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
                results.push(
                    self.owner
                        .to_definition(end_node)
                        .expect("path to end in a definition node"),
                );
                added_nodes.insert(end_node);
            }
        }
        results
    }
}
