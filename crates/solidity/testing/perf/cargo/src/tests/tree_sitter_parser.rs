use streaming_iterator::StreamingIterator;
use tree_sitter::{Parser, Query, QueryCursor};

use crate::dataset::SolidityProject;

pub fn run(project: &SolidityProject) -> Vec<tree_sitter::Tree> {
    go(project, Vec::new(), |mut trees, _source, tree| {
        trees.push(tree);
        trees
    })
}

pub fn test(project: &SolidityProject) -> usize {
    go(project, 0, |count, source, tree| {
        count + count_definitions(source, tree)
    })
}

fn go<T>(
    project: &SolidityProject,
    init: T,
    mut fold: impl FnMut(T, &String, tree_sitter::Tree) -> T,
) -> T {
    let mut parser = Parser::new();
    let language = &tree_sitter_solidity::LANGUAGE.into();
    parser
        .set_language(language)
        .expect("Error loading Solidity parser");

    let mut acc = init;
    for source in project.sources.values() {
        let tree = parser.parse(source, None).unwrap();
        let root_node = tree.root_node();
        assert!(!root_node.has_error());
        assert_eq!(root_node.kind(), "source_file");

        acc = fold(acc, source, tree);
    }

    acc
}

fn count_definitions(source: &String, tree: tree_sitter::Tree) -> usize {
    let contracts = Query::new(
        &tree.language(),
        "[(contract_declaration) (library_declaration) (interface_declaration)]",
    )
    .unwrap();
    let mut cursor = QueryCursor::new();
    let matches = cursor.matches(&contracts, tree.root_node(), source.as_bytes());
    matches.count()
}
