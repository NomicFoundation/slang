use std::sync::Arc;

use metaslang_bindings::PathResolver;
use slang_solidity::bindings::{create_with_resolver, Bindings};
use slang_solidity::cst::TextIndex;
use slang_solidity::parser::Parser;

use crate::dataset::SOLC_VERSION;
use crate::tests::parser::ParsedFile;

pub fn setup() -> Vec<ParsedFile> {
    let files = super::parser::setup();

    super::parser::run(files)
}

pub fn run(files: &[ParsedFile]) -> Bindings {
    let mut definition_count = 0_usize;
    let parser = Parser::new(SOLC_VERSION).unwrap();
    let mut bindings = create_with_resolver(&parser, Arc::new(NoOpResolver {}));

    for ParsedFile {
        path,
        contents: _,
        tree,
    } in files
    {
        bindings.add_file(
            path.to_str().unwrap(),
            tree.cursor_with_offset(TextIndex::ZERO),
        );
        definition_count += bindings.all_definitions().count();
    }

    assert!(
        // TODO(#1077): finalize the assertion counts once bindings are fully implemented:
        definition_count >= 723,
        "Only found {definition_count} definitions"
    );

    bindings
}

struct NoOpResolver;

impl PathResolver for NoOpResolver {
    fn resolve_path(&self, _context_path: &str, path_to_resolve: &str) -> Option<String> {
        Some(path_to_resolve.to_string())
    }
}
