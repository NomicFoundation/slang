use std::rc::Rc;

use slang_solidity::bindings::BindingGraph;

use crate::tests::parser::ParsedFile;

pub struct Dependencies {
    pub binding_graph: BindingGraph,
    pub files: Vec<ParsedFile>,
}

pub fn setup() -> Dependencies {
    let binding_graph = super::init_bindings::run();
    let files = super::parser::run(super::parser::setup());

    Dependencies {
        binding_graph,
        files,
    }
}

pub fn run(dependencies: Dependencies) -> Rc<BindingGraph> {
    let Dependencies {
        mut binding_graph,
        files,
    } = dependencies;

    for ParsedFile {
        path,
        contents: _,
        parse_output,
    } in &files
    {
        binding_graph.add_user_file(path.to_str().unwrap(), parse_output.create_tree_cursor());
    }

    let binding_graph = binding_graph.resolve();
    let definition_count = binding_graph
        .all_definitions()
        .filter(|definition| definition.get_file().is_user())
        .count();

    assert_eq!(definition_count, 882, "Failed to fetch all definitions");

    binding_graph
}
