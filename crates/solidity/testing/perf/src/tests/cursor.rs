use std::rc::Rc;

use slang_solidity::compilation::CompilationUnit;
use slang_solidity::cst::NonterminalKind;

pub fn setup() -> Rc<CompilationUnit> {
    super::parser::run(super::parser::setup())
}

pub fn run(unit: Rc<CompilationUnit>) {
    let mut functions_count = 0;

    for file in &unit.files() {
        let mut cursor = file.create_tree_cursor();

        while cursor.go_to_next_nonterminal_with_kind(NonterminalKind::FunctionDefinition) {
            functions_count += 1;
        }
    }

    assert_eq!(
        functions_count, 244,
        "Failed to fetch all function definitions"
    );
}
