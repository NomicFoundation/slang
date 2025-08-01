use std::cell::OnceCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use semver::Version;

use crate::bindings::{create_with_resolver, BindingGraph};
use crate::compilation::File;
use crate::cst::{Cursor, KindTypes};

/// A complete compilation unit is a complete view over all compilation inputs:
///
/// - All source files, stored as CSTs.
/// - Name binding graph that exposes relationships between definitions and references in these files.
/// - Any relevant compilation options.
///
/// It also exposes utilities to traverse the compilation unit and query it.
pub struct CompilationUnit {
    language_version: Version,
    files: BTreeMap<String, Rc<File>>,
    binding_graph: OnceCell<Rc<BindingGraph>>,
}

impl CompilationUnit {
    pub(super) fn create(language_version: Version, files: BTreeMap<String, Rc<File>>) -> Self {
        Self {
            language_version,
            files,
            binding_graph: OnceCell::new(),
        }
    }

    /// Returns the language version this compilation unit is configured for.
    pub fn language_version(&self) -> &Version {
        &self.language_version
    }

    /// Returns a list of all files in the compilation unit.
    pub fn files(&self) -> Vec<Rc<File>> {
        self.files.values().cloned().collect()
    }

    /// Returns the file with the specified ID, if it exists.
    pub fn file(&self, id: &str) -> Option<Rc<File>> {
        self.files.get(id).cloned()
    }

    /// Calculates name binding information for all source files within the compilation unit.
    /// Returns a [`BindingGraph`] that contains all found definitions and their references.
    ///
    /// Building this graph is an expensive operation. It is done lazily on the first access,
    /// and cached thereafter.
    pub fn binding_graph(&self) -> &Rc<BindingGraph> {
        self.binding_graph.get_or_init(|| {
            let resolver = Resolver {
                files: self.files.clone(),
            };

            let mut builder =
                create_with_resolver(self.language_version.clone(), Rc::new(resolver));

            for (id, file) in &self.files {
                builder.add_user_file(id, file.create_tree_cursor());
            }

            builder.build()
        })
    }
}

struct Resolver {
    files: BTreeMap<String, Rc<File>>,
}

impl metaslang_bindings::PathResolver<KindTypes> for Resolver {
    fn resolve_path(&self, context_path: &str, path_to_resolve: &Cursor) -> Option<String> {
        self.files
            .get(context_path)?
            .resolved_import(path_to_resolve)
            .cloned()
    }
}
