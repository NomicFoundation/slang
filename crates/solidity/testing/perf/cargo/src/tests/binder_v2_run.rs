use std::rc::Rc;

use slang_solidity::backend::binder::Resolution;
use slang_solidity::backend::SemanticAnalysis;
use slang_solidity::compilation::CompilationUnit;
use slang_solidity::cst::{NodeKind, TerminalKindExtensions};

#[derive(Clone)]
pub struct BuiltSemanticAnalysis {
    pub unit: Rc<CompilationUnit>,
    pub semantic_analysis: Rc<SemanticAnalysis>,
}

pub fn setup(project: &str) -> Rc<CompilationUnit> {
    let project = super::setup::setup(project);
    project.build_compilation_unit().unwrap().into()
}

pub fn run(unit: Rc<CompilationUnit>) -> BuiltSemanticAnalysis {
    let semantic_analysis = inner_run(&unit).0;
    BuiltSemanticAnalysis {
        unit,
        semantic_analysis,
    }
}

pub fn test(unit: Rc<CompilationUnit>) -> usize {
    inner_run(&unit).1
}

fn inner_run(unit: &Rc<CompilationUnit>) -> (Rc<SemanticAnalysis>, usize) {
    let semantic_analysis = unit.semantic_analysis();

    let mut ids = 0;

    for file in unit.files() {
        let mut cursor = file.create_tree_cursor();

        while cursor.go_to_next_terminal() {
            if !matches!(cursor.node().kind(), NodeKind::Terminal(kind) if kind.is_identifier()) {
                continue;
            }

            ids += 1;
            let node_id = cursor.node().id();
            if semantic_analysis
                .binder()
                .find_definition_by_identifier_node_id(node_id)
                .is_none()
                && semantic_analysis
                    .binder()
                    .find_reference_by_identifier_node_id(node_id)
                    .is_none_or(|reference| reference.resolution == Resolution::Unresolved)
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
    (Rc::clone(semantic_analysis), ids)
}
