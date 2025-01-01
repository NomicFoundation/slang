use slang_solidity::bindings::BindingGraphBuilder;

use crate::tests::parser::ParsedFile;

pub struct Dependencies {
    pub binding_graph_builder: BindingGraphBuilder,
    pub files: Vec<ParsedFile>,
}

pub fn setup() -> Dependencies {
    let binding_graph_builder = super::init_bindings::run();
    let files = super::parser::run(super::parser::setup());

    Dependencies {
        binding_graph_builder,
        files,
    }
}

pub fn run(dependencies: Dependencies) -> BindingGraphBuilder {
    let Dependencies {
        mut binding_graph_builder,
        files,
    } = dependencies;

    for ParsedFile {
        path,
        contents: _,
        parse_output,
    } in &files
    {
        binding_graph_builder
            .add_user_file(path.to_str().unwrap(), parse_output.create_tree_cursor());
    }

    binding_graph_builder
}
