use std::rc::Rc;

use slang_solidity::compilation::CompilationUnit;
use slang_solidity::cst::Query;
use solidity_testing_perf_utils::config::Library;

pub fn setup(project: &str) -> Rc<CompilationUnit> {
    super::parser::run(super::setup::setup(project, Library::Slang).unwrap())
}

pub fn run(unit: Rc<CompilationUnit>) -> usize {
    let mut contract_count = 0;

    let queries = vec![
        Query::create("[ContractDefinition]").unwrap(),
        Query::create("[InterfaceDefinition]").unwrap(),
        Query::create("[LibraryDefinition]").unwrap(),
    ];

    for file in &unit.files() {
        let cursor = file.create_tree_cursor();
        contract_count += cursor.query(queries.clone()).count();
    }

    contract_count
}
