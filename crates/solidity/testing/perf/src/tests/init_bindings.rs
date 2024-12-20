use std::rc::Rc;

use metaslang_bindings::PathResolver;
use slang_solidity::bindings::{create_with_resolver, BindingGraphBuilder};
use slang_solidity::cst::{Cursor, KindTypes};

use crate::dataset::SOLC_VERSION;

pub fn run() -> BindingGraphBuilder {
    create_with_resolver(SOLC_VERSION, Rc::new(NoOpResolver {})).unwrap()
}

struct NoOpResolver;

impl PathResolver<KindTypes> for NoOpResolver {
    fn resolve_path(&self, _context_path: &str, path_to_resolve: &Cursor) -> Option<String> {
        let path = path_to_resolve.node().unparse();
        let path = path
            .strip_prefix(|c| matches!(c, '"' | '\''))?
            .strip_suffix(|c| matches!(c, '"' | '\''))?;

        Some(path.to_owned())
    }
}
