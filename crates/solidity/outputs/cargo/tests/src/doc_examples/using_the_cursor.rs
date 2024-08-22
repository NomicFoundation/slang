use std::path::Path;
use std::rc::Rc;

use anyhow::Result;
use infra_utils::paths::PathExtensions;

#[test]
fn using_the_cursor() -> Result<()> {
    // --8<-- [start:imports]
    use semver::Version;
    use slang_solidity::cst::{EdgeLabel, NonterminalKind, TerminalKind, TextRangeExtensions};
    use slang_solidity::language::Language;
    // --8<-- [end:imports]

    let input_path = Path::repo_path("documentation/public/user-guide/inputs/using-the-cursor.sol");
    let input_path = input_path.unwrap_str();

    let source = std::fs::read_to_string(input_path)?;
    let source = source.trim();

    // --8<-- [start:parse-input]
    let language = Language::new(Version::parse("0.8.0")?)?;

    let parse_output = language.parse(NonterminalKind::SourceUnit, source);
    // --8<-- [end:parse-input]

    {
        // --8<-- [start:listing-contract-names]
        let mut contracts = Vec::new();

        let mut cursor = parse_output.create_tree_cursor();

        while cursor.go_to_next_nonterminal_with_kind(NonterminalKind::ContractDefinition) {
            assert!(cursor.go_to_first_child());
            assert!(cursor.go_to_next_terminal_with_kind(TerminalKind::Identifier));

            let terminal_node = cursor.node();
            contracts.push(terminal_node.as_terminal().unwrap().text.clone());

            // You have to make sure you return the cursor to its original position:
            assert!(cursor.go_to_parent());
        }

        assert_eq!(contracts, &["Foo", "Bar", "Baz"]);
        // --8<-- [end:listing-contract-names]
    }

    {
        // --8<-- [start:visiting-sub-tree]
        let mut contracts = Vec::new();

        let mut cursor = parse_output.create_tree_cursor();

        while cursor.go_to_next_nonterminal_with_kind(NonterminalKind::ContractDefinition) {
            let mut child_cursor = cursor.spawn();
            assert!(child_cursor.go_to_next_terminal_with_kind(TerminalKind::Identifier));

            let terminal_node = child_cursor.node();
            contracts.push(terminal_node.as_terminal().unwrap().text.clone());
        }

        assert_eq!(contracts, &["Foo", "Bar", "Baz"]);
        // --8<-- [end:visiting-sub-tree]
    }

    {
        // --8<-- [start:accessing-node-positions]
        let mut contracts = Vec::new();

        let mut cursor = parse_output.create_tree_cursor();

        while cursor.go_to_next_nonterminal_with_kind(NonterminalKind::ContractDefinition) {
            let range = cursor.text_range().utf8();
            let text = Rc::clone(cursor.node().as_nonterminal().unwrap()).unparse();

            contracts.push((range, text.trim().to_owned()));
        }

        assert_eq!(
            contracts,
            &[
                (0..16, "contract Foo {}".to_string()),
                (16..32, "contract Bar {}".to_string()),
                (32..47, "contract Baz {}".to_string()),
            ]
        );
        // --8<-- [end:accessing-node-positions]
    }

    {
        // --8<-- [start:using-iterator-api]
        let cursor = parse_output.create_tree_cursor();

        let identifiers: Vec<_> = cursor
            .filter_map(|node| {
                node.as_terminal_with_kind(TerminalKind::Identifier)
                    .cloned()
            })
            .map(|identifier| identifier.text.clone())
            .collect();

        assert_eq!(identifiers, &["Foo", "Bar", "Baz"]);
        // --8<-- [end:using-iterator-api]
    }

    {
        // --8<-- [start:using-labeled-cursors]
        let cursor = parse_output.create_tree_cursor();

        let identifiers: Vec<_> = cursor
            .with_edges()
            .filter(|node| node.label == Some(EdgeLabel::Name))
            .filter_map(|node| {
                node.as_terminal_with_kind(TerminalKind::Identifier)
                    .cloned()
            })
            .map(|identifier| identifier.text.clone())
            .collect();

        assert_eq!(identifiers, &["Foo", "Bar", "Baz"]);
        // --8<-- [end:using-labeled-cursors]
    }

    Ok(())
}
