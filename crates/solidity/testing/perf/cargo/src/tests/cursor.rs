use std::rc::Rc;

use slang_solidity::compilation::CompilationUnit;
use slang_solidity::cst::NonterminalKind;

pub fn setup(project: &str) -> Rc<CompilationUnit> {
    super::parser::run(super::setup::setup(project))
}

pub fn run(unit: Rc<CompilationUnit>) {
    let mut functions_count = 0;

    for file in &unit.files() {
        let mut cursor = file.create_tree_cursor();

        while cursor.go_to_next_nonterminal_with_kind(NonterminalKind::FunctionDefinition) {
            functions_count += 1;
        }
    }

    // Sanity check: at least there's a function.
    assert_ne!(
        functions_count, 0,
        "Failed to fetch all function definitions"
    );
}
