use std::rc::Rc;

use slang_solidity::cst::{NodeKind, TerminalKindExtensions};

use crate::tests::bindings_build::BuiltBindingGraph;

pub fn setup(project: &str) -> BuiltBindingGraph {
    let unit = super::parser::run(super::setup::setup(project));
    super::bindings_build::run(Rc::clone(&unit))
}

pub fn run(dependencies: BuiltBindingGraph) -> BuiltBindingGraph {
    let _ = test(dependencies.clone());
    dependencies
}

pub fn test(dependencies: BuiltBindingGraph) -> usize {
    let BuiltBindingGraph {
        unit,
        binding_graph,
    } = dependencies;

    let mut ids = 0;

    for file in unit.files() {
        let mut cursor = file.create_tree_cursor();

        while cursor.go_to_next_terminal() {
            if !matches!(cursor.node().kind(), NodeKind::Terminal(kind) if kind.is_identifier()) {
                continue;
            }

            ids += 1;
            if binding_graph.definition_at(&cursor).is_none()
                && binding_graph
                    .reference_at(&cursor)
                    .is_none_or(|reference| reference.definitions().is_empty())
            {
                panic!(
                    "Unbound identifier or unresolved reference: '{value}' in '{file_path}:{line}:{column}'.",
                    value = cursor.node().unparse(),
                    file_path = file.id(),
                    line = cursor.text_range().start.line + 1,
                    column = cursor.text_range().start.column + 1,
                );
            }
        }
    }
    ids
}
