use std::rc::Rc;

use infra_utils::paths::PathExtensions;
use slang_solidity::bindings::BindingGraph;
use slang_solidity::cst::{NodeKind, TerminalKindExtensions};

use crate::tests::parser::ParsedFile;

pub struct BuiltBindingGraph {
    files: Vec<ParsedFile>,
    binding_graph: Rc<BindingGraph>,
}

pub fn setup() -> BuiltBindingGraph {
    let files = super::parser::run(super::parser::setup());
    let binding_graph = super::bindings_build::run(files.clone());

    BuiltBindingGraph {
        files,
        binding_graph,
    }
}

pub fn run(dependencies: BuiltBindingGraph) {
    let BuiltBindingGraph {
        files,
        binding_graph,
    } = dependencies;

    for file in files {
        let mut cursor = file.parse_output.create_tree_cursor();

        while cursor.go_to_next_terminal() {
            if !matches!(cursor.node().kind(), NodeKind::Terminal(kind) if kind.is_identifier()) {
                continue;
            }

            if binding_graph.definition_at(&cursor).is_none()
                && binding_graph.reference_at(&cursor).is_none()
            {
                panic!(
                    "Unbound identifier: '{value}' in '{file_path}:{line}:{column}'.",
                    value = cursor.node().unparse(),
                    file_path = file.path.unwrap_str(),
                    line = cursor.text_range().start.line + 1,
                    column = cursor.text_range().start.column + 1,
                );
            }
        }
    }
}
