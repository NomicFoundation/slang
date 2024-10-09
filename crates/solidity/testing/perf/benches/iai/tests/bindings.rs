use std::sync::Arc;

use metaslang_bindings::{FileKind, PathResolver};
use slang_solidity::bindings::{create_with_resolver, get_built_ins, Bindings};
use slang_solidity::parser::Parser;

use crate::dataset::SOLC_VERSION;

pub fn run() -> Bindings {
    let parser = Parser::create(SOLC_VERSION).unwrap();
    let mut bindings = create_with_resolver(SOLC_VERSION, Arc::new(NoOpResolver {}));

    let built_ins_parse_output = parser.parse(Parser::ROOT_KIND, get_built_ins(&SOLC_VERSION));
    assert!(
        built_ins_parse_output.is_valid(),
        "built-ins parse without errors"
    );
    bindings.add_system_file("built_ins.sol", built_ins_parse_output.create_tree_cursor());
    bindings
}

struct NoOpResolver;

impl PathResolver for NoOpResolver {
    fn resolve_path(&self, _context_path: &FileKind, path_to_resolve: &str) -> FileKind {
        FileKind::User(path_to_resolve.to_string())
    }
}
