use std::sync::Arc;

use metaslang_bindings::PathResolver;
use slang_solidity::bindings::{create_with_resolver, get_built_ins, Bindings};
use slang_solidity::parser::{ParseOutput, Parser};

use crate::dataset::SOLC_VERSION;

pub fn setup() -> ParseOutput {
    let parser = Parser::create(SOLC_VERSION).unwrap();

    let built_ins = parser.parse(Parser::ROOT_KIND, get_built_ins(&SOLC_VERSION));

    assert!(built_ins.is_valid(), "built-ins parse without errors");

    built_ins
}

pub fn run(built_ins: ParseOutput) -> Bindings {
    let mut bindings = create_with_resolver(SOLC_VERSION, Arc::new(NoOpResolver {}));

    bindings.add_system_file("built_ins.sol", built_ins.create_tree_cursor());

    bindings
}

struct NoOpResolver;

impl PathResolver for NoOpResolver {
    fn resolve_path(&self, _context_path: &str, path_to_resolve: &str) -> Option<String> {
        Some(path_to_resolve.to_string())
    }
}
