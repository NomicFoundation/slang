use std::rc::Rc;

use slang_solidity::bindings::{BindingGraph, Reference};
use slang_solidity::compilation::CompilationUnit;
use slang_solidity::cst::{NodeKind, NonterminalKind, TerminalKindExtensions};

pub struct BuiltBindingGraph {
    unit: Rc<CompilationUnit>,
    binding_graph: Rc<BindingGraph>,
}

pub fn setup(project: &str) -> BuiltBindingGraph {
    let unit = super::parser::run(super::parser::setup(project));
    let binding_graph = super::bindings_build::run(Rc::clone(&unit));

    BuiltBindingGraph {
        unit,
        binding_graph,
    }
}

pub fn run(dependencies: BuiltBindingGraph) {
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

            if matches!(
                cursor.ancestors().next(),
                Some(ancestor)
                // ignore identifiers in `pragma experimental` directives, as they are unbound feature names:
                if ancestor.kind == NonterminalKind::ExperimentalFeature
            ) {
                continue;
            }

            if binding_graph.definition_at(&cursor).is_none()
                && none_or_empty(binding_graph.reference_at(&cursor))
            {
                panic!(
                    "Unbound identifier: '{value}' in '{file_path}:{line}:{column}'.",
                    value = cursor.node().unparse(),
                    file_path = file.id(),
                    line = cursor.text_range().start.line + 1,
                    column = cursor.text_range().start.column + 1,
                );
            }
        }
    }
    assert_ne!(ids, 0);
}

fn none_or_empty(maybe_reference: Option<Reference>) -> bool {
    maybe_reference.is_none()
        || maybe_reference.is_some_and(|reference| reference.definitions().is_empty())
}
