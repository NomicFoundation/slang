use slang_solidity::bindings::Bindings;
use slang_solidity::cst::TextIndex;

use crate::tests::parser::ParsedFile;

pub struct Dependencies {
    pub bindings: Bindings,
    pub files: Vec<ParsedFile>,
}

pub fn setup() -> Dependencies {
    let bindings = super::bindings::run();
    let files = super::parser::run(super::parser::setup());

    Dependencies { bindings, files }
}

pub fn run(dependencies: Dependencies) -> Bindings {
    let mut definition_count = 0_usize;
    let Dependencies {
        mut bindings,
        files,
    } = dependencies;

    for ParsedFile {
        path,
        contents: _,
        tree,
    } in &files
    {
        bindings.add_user_file(
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
