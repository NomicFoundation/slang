use std::path::PathBuf;

use semver::Version;
use slang_solidity::parser::{ParseOutput, Parser};

use crate::dataset::{SourceFile, SOLC_VERSION};

#[derive(Clone)]
pub struct ParsedFile {
    pub path: PathBuf,

    #[allow(dead_code)] // false-positive. it is used below.
    pub contents: String,

    pub parse_output: ParseOutput,
}

pub fn setup() -> (Vec<SourceFile>, Version) {
    (SourceFile::load_all(), SOLC_VERSION)
}

pub fn run(payload: (Vec<SourceFile>, Version)) -> Vec<ParsedFile> {
    let (files, language_version) = payload;
    let parser = Parser::create(language_version).unwrap();

    let mut results = vec![];

    for SourceFile { path, contents } in files {
        let parse_output = parser.parse_file_contents(&contents);

        assert!(
            parse_output.is_valid(),
            "Found parse errors:\n{0:#?}",
            parse_output.errors(),
        );

        results.push(ParsedFile {
            path,
            contents,
            parse_output,
        });
    }

    results
}
