use std::sync::Arc;

use metaslang_bindings::PathResolver;
use slang_solidity::bindings::{add_built_ins, create_with_resolver, Bindings};

use crate::dataset::SOLC_VERSION;

pub fn run() -> Bindings {
    let mut bindings = create_with_resolver(SOLC_VERSION, Arc::new(NoOpResolver {}));
    add_built_ins(&mut bindings, &SOLC_VERSION).unwrap();

    bindings
}

struct NoOpResolver;

impl PathResolver for NoOpResolver {
    fn resolve_path(&self, _context_path: &str, path_to_resolve: &str) -> Option<String> {
        Some(path_to_resolve.to_string())
    }
}
