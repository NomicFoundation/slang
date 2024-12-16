use std::rc::Rc;

use metaslang_bindings::PathResolver;
use slang_solidity::bindings::{create_with_resolver, get_built_ins, BindingGraph};
use slang_solidity::cst::{Cursor, KindTypes, TextIndex};
use slang_solidity::parser::{ParseOutput, Parser};
use slang_solidity::transform_built_ins_node;

use crate::dataset::SOLC_VERSION;

pub fn setup() -> ParseOutput {
    let parser = Parser::create(SOLC_VERSION).unwrap();

    let built_ins = parser.parse(Parser::ROOT_KIND, get_built_ins(&SOLC_VERSION));

    assert!(built_ins.is_valid(), "built-ins parse without errors");

    built_ins
}

pub fn run(built_ins: ParseOutput) -> BindingGraph {
    let mut binding_graph = create_with_resolver(SOLC_VERSION, Rc::new(NoOpResolver {}));

    let built_ins_cursor =
        transform_built_ins_node(built_ins.tree()).cursor_with_offset(TextIndex::ZERO);

    binding_graph.add_system_file("built_ins.sol", built_ins_cursor);

    binding_graph
}

struct NoOpResolver;

impl PathResolver<KindTypes> for NoOpResolver {
    fn resolve_path(&self, _context_path: &str, path_to_resolve: &Cursor) -> Option<String> {
        Some(path_to_resolve.node().unparse())
    }
}
