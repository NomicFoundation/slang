use std::rc::Rc;

use slang_solidity::compilation::CompilationUnit;
use slang_solidity::cst::Query;

pub fn setup(project: &str) -> Rc<CompilationUnit> {
    super::parser::run(super::setup::setup(project))
}

pub fn run(unit: Rc<CompilationUnit>) {
    let mut functions_count = 0;

    let queries = vec![Query::create(
        "[FunctionDefinition
            @name name: [_]
        ]",
    )
    .unwrap()];

    for file in &unit.files() {
        let cursor = file.create_tree_cursor();

        for query_match in cursor.query(queries.clone()) {
            assert_eq!(query_match.captures.len(), 1);

            functions_count += 1;
        }
    }

    // Sanity check: at least there's a function.
    assert_ne!(
        functions_count, 0,
        "Failed to fetch all function definitions"
    );
}
