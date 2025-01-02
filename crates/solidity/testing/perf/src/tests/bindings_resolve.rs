use std::rc::Rc;

use slang_solidity::bindings::BindingGraph;
use slang_solidity::cst::{NonterminalKind, TerminalKind};

use crate::tests::parser::ParsedFile;

pub struct BuiltBindingGraph {
    files: Vec<ParsedFile>,
    binding_graph: Rc<BindingGraph>,
}

pub fn setup() -> BuiltBindingGraph {
    let files = super::bindings_build::setup();
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

        while cursor.go_to_next_terminal_with_kinds(&[
            TerminalKind::Identifier,
            TerminalKind::YulIdentifier,
        ]) {
            if matches!(
                cursor.ancestors().next(),
                Some(ancestor) if ancestor.kind == NonterminalKind::ExperimentalFeature
            ) {
                // ignore identifiers in `pragma experimental` directives, as they are unbound feature names:
                continue;
            }

            if binding_graph.definition_at(&cursor).is_none()
                && binding_graph.reference_at(&cursor).is_none()
            {
                panic!(
                    "Unbound identifier: '{value}' at '{range:?}'.",
                    value = cursor.node().unparse(),
                    range = cursor.text_range()
                );
            }
        }
    }
}
