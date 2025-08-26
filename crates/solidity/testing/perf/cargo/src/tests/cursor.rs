use std::rc::Rc;

use slang_solidity::compilation::CompilationUnit;
use slang_solidity::cst::NonterminalKind;

pub fn setup(project: &str) -> Rc<CompilationUnit> {
    super::parser::run(super::setup::setup(project))
}

pub fn run(unit: Rc<CompilationUnit>) -> usize {
    let mut contract_count = 0;

    for file in &unit.files() {
        let mut cursor = file.create_tree_cursor();

        while cursor.go_to_next_nonterminal_with_kinds(&[
            NonterminalKind::ContractDefinition,
            NonterminalKind::InterfaceDefinition,
            NonterminalKind::LibraryDefinition,
        ]) {
            contract_count += 1;
        }
    }
    contract_count
}
