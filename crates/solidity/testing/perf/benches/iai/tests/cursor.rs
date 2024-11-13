use slang_solidity::cst::{NonterminalKind, TextIndex};

use crate::tests::parser::ParsedFile;

pub fn setup() -> Vec<ParsedFile> {
    let files = super::parser::setup();

    super::parser::run(files)
}

pub fn run(files: Vec<ParsedFile>) {
    let mut functions_count = 0;

    for file in &files {
        let mut cursor = file.tree.cursor_with_offset(TextIndex::ZERO);

        while cursor.go_to_next_nonterminal_with_kind(NonterminalKind::FunctionDefinition) {
            functions_count += 1;
        }
    }

    assert_eq!(
        functions_count, 200,
        "Failed to fetch all function definitions"
    );
}
