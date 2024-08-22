use slang_solidity::cst::{Query, TextIndex};

use crate::tests::parser::ParsedFile;

pub fn setup() -> Vec<ParsedFile> {
    let files = super::parser::setup();

    super::parser::run(files)
}

pub fn run(files: &[ParsedFile]) {
    let mut functions_count = 0;

    let queries = vec![Query::parse(
        "[FunctionDefinition
            @name name: [_]
        ]",
    )
    .unwrap()];

    for file in files {
        let cursor = file.tree.cursor_with_offset(TextIndex::ZERO);

        for query_match in cursor.query(queries.clone()) {
            assert_eq!(query_match.captures.len(), 1);

            functions_count += 1;
        }
    }

    assert_eq!(
        functions_count, 200,
        "Failed to fetch all function definitions"
    );
}
