use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Parser, Query, QueryCursor};

use crate::dataset::SolidityProject;

pub fn run(project: &SolidityProject, count_contracts: bool) -> usize {
    test(project, count_contracts)
}

pub fn test(project: &SolidityProject, count_contracts: bool) -> usize {
    let mut parser = Parser::new();
    let language = &tree_sitter_solidity::LANGUAGE.into();
    parser
        .set_language(language)
        .expect("Error loading Solidity parser");

    let mut contract_count = 0;
    for source in project.sources.values() {
        let tree = parser.parse(source, None).unwrap();
        let root_node = tree.root_node();
        assert!(!root_node.has_error());
        assert_eq!(root_node.kind(), "source_file");

        if count_contracts {
            contract_count += count_definition(language, source, root_node, "contract");
            contract_count += count_definition(language, source, root_node, "library");
            contract_count += count_definition(language, source, root_node, "interface");
        }
    }

    contract_count
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
