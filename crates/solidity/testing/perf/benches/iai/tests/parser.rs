use std::path::PathBuf;

use slang_solidity::cst::Node;
use slang_solidity::parser::Parser;

use crate::dataset::{SourceFile, SOLC_VERSION};

pub struct ParsedFile {
    pub path: PathBuf,

    #[allow(dead_code)] // false-positive. it is used below.
    pub contents: String,

    pub tree: Node,
}

pub fn setup() -> Vec<SourceFile> {
    SourceFile::load_all()
}

pub fn run(files: Vec<SourceFile>) -> Vec<ParsedFile> {
    let parser = Parser::create(SOLC_VERSION).unwrap();

    let mut results = vec![];

    for SourceFile { path, contents } in files {
        let parse_output = parser.parse(Parser::ROOT_KIND, &contents);

        assert!(
            parse_output.is_valid(),
            "Found parse errors:\n{0:#?}",
            parse_output.errors(),
        );

        results.push(ParsedFile {
            path,
            contents,
            tree: parse_output.tree(),
        });
    }

    results
}
