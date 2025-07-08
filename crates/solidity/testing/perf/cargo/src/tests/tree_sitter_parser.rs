use tree_sitter::Parser;

use crate::dataset::SolidityProject;

pub fn run(project: &SolidityProject) {
    let mut parser = Parser::new();
    let language = tree_sitter_solidity::LANGUAGE;
    parser
        .set_language(&language.into())
        .expect("Error loading Solidity parser");

    for source in project.sources.values() {
        let tree = parser.parse(source, None).unwrap();
        assert!(!tree.root_node().has_error());
    }
}
