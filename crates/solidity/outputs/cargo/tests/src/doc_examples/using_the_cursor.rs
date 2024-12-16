use std::path::Path;

use anyhow::Result;
use infra_utils::paths::PathExtensions;

#[test]
fn using_the_cursor() -> Result<()> {
    // --8<-- [start:imports]
    use semver::Version;
    use slang_solidity::cst::{EdgeLabel, NonterminalKind, TerminalKind, TextRangeExtensions};
    use slang_solidity::parser::Parser;
    // --8<-- [end:imports]

    let input_path = Path::repo_path("documentation/public/user-guide/inputs/using-the-cursor.sol");
    let input_path = input_path.unwrap_str();

    let source = std::fs::read_to_string(input_path)?;
    let source = source.trim();

    // --8<-- [start:parse-input]
    let parser = Parser::create(Version::parse("0.8.0")?)?;

    let parse_output = parser.parse(NonterminalKind::SourceUnit, source);
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
            let text = cursor.node().unparse();

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

        let identifiers: Vec<_> = parse_output
            .tree()
            .clone()
            .descendants()
            .filter(|edge| edge.is_terminal_with_kind(TerminalKind::Identifier))
            .map(|identifier| identifier.unparse())
            .collect();

        assert_eq!(identifiers, &["Foo", "Bar", "Baz"]);
        // --8<-- [end:using-iterator-api]
    }

    {
        // --8<-- [start:using-cursors-with-labels]

        let identifiers: Vec<_> = parse_output
            .tree()
            .clone()
            .descendants()
            .filter(|edge| edge.label == Some(EdgeLabel::Name))
            .filter(|edge| edge.is_terminal_with_kind(TerminalKind::Identifier))
            .map(|identifier| identifier.unparse())
            .collect();

        assert_eq!(identifiers, &["Foo", "Bar", "Baz"]);
        // --8<-- [end:using-cursors-with-labels]
    }

    Ok(())
}
