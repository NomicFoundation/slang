use slang_solidity::cst::Query;

use crate::tests::parser::ParsedFile;

pub fn setup() -> Vec<ParsedFile> {
    let files = super::parser::setup();

    super::parser::run(files)
}

pub fn run(files: Vec<ParsedFile>) {
    let mut functions_count = 0;

    let queries = vec![Query::create(
        "[FunctionDefinition
            @name name: [_]
        ]",
    )
    .unwrap()];

    for file in &files {
        let cursor = file.parse_output.create_tree_cursor();

        for query_match in cursor.query(queries.clone()) {
            assert_eq!(query_match.captures.len(), 1);

            functions_count += 1;
        }
    }

    assert_eq!(
        functions_count, 265,
        "Failed to fetch all function definitions"
    );
}
