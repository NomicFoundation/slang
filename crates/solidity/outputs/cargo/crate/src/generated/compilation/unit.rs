// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::cell::OnceCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use semver::Version;

use crate::bindings::{
    create_with_resolver, BindingGraph, BindingGraphInitializationError, PathResolver,
};
use crate::compilation::File;
use crate::cst::{Cursor, KindTypes};

pub struct CompilationUnit {
    language_version: Version,
    files: BTreeMap<String, Rc<File>>,
    binding_graph: OnceCell<Result<Rc<BindingGraph>, BindingGraphInitializationError>>,
}

impl CompilationUnit {
    pub(super) fn new(language_version: Version, files: BTreeMap<String, Rc<File>>) -> Self {
        Self {
            language_version,
            files,
            binding_graph: OnceCell::new(),
        }
    }

    pub fn language_version(&self) -> &Version {
        &self.language_version
    }

    pub fn files(&self) -> Vec<Rc<File>> {
        self.files.values().cloned().collect()
    }

    pub fn file(&self, id: &str) -> Option<Rc<File>> {
        self.files.get(id).cloned()
    }

    pub fn binding_graph(&self) -> &Result<Rc<BindingGraph>, BindingGraphInitializationError> {
        self.binding_graph.get_or_init(|| {
            let resolver = Resolver {
                files: self.files.clone(),
            };

            let mut binding_graph =
                create_with_resolver(self.language_version.clone(), Rc::new(resolver))?;

            for (id, file) in &self.files {
                binding_graph.add_user_file(id, file.create_tree_cursor());
            }

            Ok(Rc::new(binding_graph))
        })
    }
}

struct Resolver {
    files: BTreeMap<String, Rc<File>>,
}

impl PathResolver<KindTypes> for Resolver {
    fn resolve_path(&self, context_path: &str, path_to_resolve: &Cursor) -> Option<String> {
        self.files
            .get(context_path)?
            .resolved_import(path_to_resolve)
            .cloned()
    }
}
