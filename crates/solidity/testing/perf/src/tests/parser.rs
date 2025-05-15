use std::path::PathBuf;

use semver::Version;
use slang_solidity::compilation::InternalCompilationBuilder;
use slang_solidity::parser::{ParseOutput, Parser};

use crate::dataset::{SourceFile, SOLC_VERSION};
use crate::compilation_builder::{self};

#[derive(Clone)]
pub struct ParsedFile {
    pub path: PathBuf,
    pub parse_output: ParseOutput,
}

pub fn setup() -> (Vec<SourceFile>, Version) {
    (SourceFile::load_all(), SOLC_VERSION)
}

pub fn run(payload: (Vec<SourceFile>, Version)) -> Vec<ParsedFile> {
    let (files, language_version) = payload;
    let builder = InternalCompilationBuilder::create(language_version).unwrap();
    builder.add_file(id, contents)
    let mut results = vec![];

    for SourceFile { path, contents } in files {
        let parse_output = parser.parse_file_contents(&contents);

        assert!(
            parse_output.is_valid(),
            "Found parse errors:\n{0:#?}",
            parse_output.errors(),
        );

        results.push(ParsedFile { path, parse_output });
    }

    results
}
