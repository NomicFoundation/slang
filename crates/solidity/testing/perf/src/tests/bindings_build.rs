use std::rc::Rc;

use infra_utils::paths::PathExtensions;
use metaslang_bindings::PathResolver;
use slang_solidity::bindings::{create_with_resolver, BindingGraph};
use slang_solidity::cst::{Cursor, KindTypes};

use crate::dataset::SOLC_VERSION;
use crate::tests::parser::ParsedFile;

pub fn setup() -> Vec<ParsedFile> {
    super::parser::run(super::parser::setup())
}

pub fn run(files: Vec<ParsedFile>) -> Rc<BindingGraph> {
    let mut bindings_graph_builder =
        create_with_resolver(SOLC_VERSION, Rc::new(Resolver {})).unwrap();

    for file in files {
        bindings_graph_builder.add_user_file(
            file.path.unwrap_str(),
            file.parse_output.create_tree_cursor(),
        );
    }

    bindings_graph_builder.build()
}

struct Resolver;

impl PathResolver<KindTypes> for Resolver {
    fn resolve_path(&self, _context_path: &str, path_to_resolve: &Cursor) -> Option<String> {
        let path = path_to_resolve.node().unparse();
        let path = path
            .strip_prefix(|c| matches!(c, '"' | '\''))?
            .strip_suffix(|c| matches!(c, '"' | '\''))?;

        Some(path.to_owned())
    }
}
