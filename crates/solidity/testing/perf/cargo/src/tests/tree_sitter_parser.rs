use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Node, Parser, Query, QueryCursor};

use crate::dataset::SolidityProject;

pub fn run(project: &SolidityProject) {
    go(
        project,
        (),
        |_: &Language, _: &String, _: Node<'_>, (): ()| (),
    );
}

pub fn test(project: &SolidityProject) -> usize {
    go(
        project,
        0,
        |language: &Language, source: &String, root_node: Node<'_>, prev: usize| {
            prev + count_definition(language, source, root_node, "contract")
                + count_definition(language, source, root_node, "library")
                + count_definition(language, source, root_node, "interface")
        },
    )
}

pub fn go<T: Copy, F>(project: &SolidityProject, initial: T, fold: F) -> T
where
    F: Fn(&Language, &String, Node<'_>, T) -> T,
{
    let mut parser = Parser::new();
    let language = &tree_sitter_solidity::LANGUAGE.into();
    parser
        .set_language(language)
        .expect("Error loading Solidity parser");

    let mut result = initial;
    for source in project.sources.values() {
        let tree = parser.parse(source, None).unwrap();
        let root_node = tree.root_node();
        assert!(!root_node.has_error());
        assert_eq!(root_node.kind(), "source_file");

        result = fold(language, source, root_node, result);
    }

    result
}

fn count_definition(
    language: &Language,
    source: &String,
    root_node: tree_sitter::Node<'_>,
    definition: &str,
) -> usize {
    let mut contract_count = 0;
    let contracts = Query::new(language, &format!("({definition}_declaration)")).unwrap();
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&contracts, root_node, source.as_bytes());

    while matches.next().is_some() {
        contract_count += 1;
    }
    contract_count
}
