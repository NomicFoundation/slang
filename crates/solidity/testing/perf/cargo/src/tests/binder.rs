use slang_solidity::backend::binder::Resolution;
use slang_solidity::backend::{build_binder_output, BinderOutput};
use slang_solidity::compilation::CompilationUnit;
use slang_solidity::cst::{NodeKind, TerminalKindExtensions};

pub fn setup(project: &str) -> CompilationUnit {
    let project = super::setup::setup(project);
    project.build_compilation_unit().unwrap()
}

pub fn run(unit: CompilationUnit) -> BinderOutput {
    inner_run(unit).0
}

pub fn test(unit: CompilationUnit) -> usize {
    inner_run(unit).1
}

fn inner_run(unit: CompilationUnit) -> (BinderOutput, usize) {
    let data = build_binder_output(unit);

    let mut ids = 0;

    for file in data.compilation_unit.files() {
        let mut cursor = file.create_tree_cursor();

        while cursor.go_to_next_terminal() {
            if !matches!(cursor.node().kind(), NodeKind::Terminal(kind) if kind.is_identifier()) {
                continue;
            }

            ids += 1;
            let node_id = cursor.node().id();
            if data
                .binder
                .find_definition_by_identifier_node_id(node_id)
                .is_none()
                && data
                    .binder
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
    (data, ids)
}
